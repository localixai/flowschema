//! What a call costs, counted rather than timed: a global allocator that counts every allocation,
//! the instructions and CPU cycles the kernel counts for the process, the fast cores asked for,
//! and the fewest of several rounds of a fixed number of calls. Shared by the bench crate and the
//! quick loop (`quick/`), which include this file.

use std::alloc::{GlobalAlloc, Layout, System};
use std::hint::black_box;
use std::sync::atomic::{AtomicU64, Ordering::Relaxed};
#[cfg(not(target_os = "macos"))]
use std::time::Instant;

/// The system's allocator, every allocation counted: a check's allocations are exact.
struct Counting;

static ALLOCATIONS: AtomicU64 = AtomicU64::new(0);

// SAFETY: every call is passed to the system's allocator as it came.
unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCATIONS.fetch_add(1, Relaxed);
        // SAFETY: the caller's layout, as `GlobalAlloc::alloc` takes it.
        unsafe { System.alloc(layout) }
    }

    unsafe fn dealloc(&self, at: *mut u8, layout: Layout) {
        // SAFETY: a block this allocator gave, with its layout.
        unsafe { System.dealloc(at, layout) }
    }

    unsafe fn realloc(&self, at: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        ALLOCATIONS.fetch_add(1, Relaxed);
        // SAFETY: a block this allocator gave, with its layout.
        unsafe { System.realloc(at, layout, size) }
    }
}

#[global_allocator]
static ALLOCATOR: Counting = Counting;

/// Allocations made so far by this process: two reads around a call count its allocations exactly.
pub fn allocations() -> u64 {
    ALLOCATIONS.load(Relaxed)
}

/// What this process has run so far: the instructions its threads retired and the CPU cycles
/// they ran, as the kernel counts them for it. Neither is the wall clock: a frequency change or
/// another process moves neither, and the instructions of the same work are the same on every
/// run, on any core. Elsewhere than macOS, nanoseconds of the monotonic clock stand in for both.
#[cfg(target_os = "macos")]
pub fn counters() -> (u64, u64) {
    /// `struct rusage_info_v4` of `<sys/resource.h>`: a UUID, then 35 counters, of which
    /// `ri_instructions` is the 30th and `ri_cycles` the 31st.
    #[repr(C)]
    struct Usage {
        uuid: [u8; 16],
        counters: [u64; 35],
    }
    unsafe extern "C" {
        fn proc_pid_rusage(pid: i32, flavor: i32, buffer: *mut Usage) -> i32;
        fn getpid() -> i32;
    }
    let mut usage = Usage {
        uuid: [0; 16],
        counters: [0; 35],
    };
    // SAFETY: a buffer of `rusage_info_v4`'s layout, for the flavor that fills it (4).
    let read = unsafe { proc_pid_rusage(getpid(), 4, &raw mut usage) };
    assert_eq!(read, 0, "proc_pid_rusage failed");
    (usage.counters[29], usage.counters[30])
}

#[cfg(not(target_os = "macos"))]
pub fn counters() -> (u64, u64) {
    static START: std::sync::OnceLock<Instant> = std::sync::OnceLock::new();
    let ns = START.get_or_init(Instant::now).elapsed().as_nanos() as u64;
    (ns, ns)
}

/// What one call costs: the instructions it retires, the same on every run, and the cycles it
/// takes, which also pay for missed caches and mispredicted branches.
#[derive(Clone, Copy, Default)]
pub struct Cost {
    pub instructions: f64,
    pub cycles: f64,
}

impl std::ops::Sub for Cost {
    type Output = Cost;
    fn sub(self, other: Cost) -> Cost {
        Cost {
            instructions: self.instructions - other.instructions,
            cycles: self.cycles - other.cycles,
        }
    }
}

/// This thread asks for the fast cores (QoS user-interactive on macOS): an efficiency core runs
/// the same work in other cycles, so a run that moved between them would not compare.
pub fn fast_cores() {
    #[cfg(target_os = "macos")]
    {
        unsafe extern "C" {
            fn pthread_set_qos_class_self_np(qos: u32, relative: i32) -> i32;
        }
        // SAFETY: QOS_CLASS_USER_INTERACTIVE (0x21) for the calling thread.
        unsafe { pthread_set_qos_class_self_np(0x21, 0) };
    }
}

/// What one call of `f` costs, the fewest of `rounds` rounds of `calls` calls each (each counter
/// on its own), after one call that warms the caches. The calls are a fixed number, so every run
/// does the same work, and the fewest is the round no interruption reached.
pub fn cost_per<T>(calls: u64, rounds: usize, mut f: impl FnMut() -> T) -> Cost {
    black_box(f());
    let (mut instructions, mut cycles) = (u64::MAX, u64::MAX);
    for _ in 0..rounds {
        let (i0, c0) = counters();
        for _ in 0..calls {
            black_box(f());
        }
        let (i1, c1) = counters();
        instructions = instructions.min(i1 - i0);
        cycles = cycles.min(c1 - c0);
    }
    Cost {
        instructions: instructions as f64 / calls as f64,
        cycles: cycles as f64 / calls as f64,
    }
}
