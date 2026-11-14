//! A JSON text the output limit cut off, or one still being written: what of it was written
//! whole, and where it was still writing. A model's answer or a call's arguments stop midway
//! when the limit is reached; the fields written before the cut are still there, and are what a
//! program can act on or check.

use serde_json::Value;

/// Parses tried when the text stops inside a string: as it came, then with its last escape left
/// out, twice at most (half a pair of surrogates is two escapes).
const TRIES: usize = 3;

/// A step from a value into the one inside it that was still being written.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Step {
    Key(String),
    Index(usize),
}

/// What a text wrote whole, and where it was still writing.
pub(crate) struct Cut {
    pub(crate) value: Value,
    /// From the root down to the innermost value still open: an object, an array, or the string
    /// `text` says. None where a name on the way could not be read.
    pub(crate) path: Option<Vec<Step>>,
    /// Whether the innermost open value is a string cut midway.
    pub(crate) text: bool,
    /// Whether nothing was open: the text is one whole value.
    pub(crate) whole: bool,
}

/// The value `text` holds, where it is whole, or what of it was written whole where it stops
/// midway: the string it stops in closed (a field's value, not its name), every object and
/// array still open closed, and what the cut left unfinished left out (a name with no value, a
/// number or a word that may go on, half an escape). `None` for a text that is not the start of
/// a JSON value.
pub fn whole(text: &str) -> Option<Value> {
    cut(text).map(|cut| cut.value)
}

/// `text` read as `whole` reads it, with where it was still writing.
pub(crate) fn cut(text: &str) -> Option<Cut> {
    if let Ok(value) = serde_json::from_str(text) {
        let (path, text, whole) = (Some(vec![]), false, true);
        return Some(Cut {
            value,
            path,
            text,
            whole,
        });
    }
    let scanned = scan(text);
    let closed = |text: &str, frames: &[Frame]| -> Option<Value> {
        serde_json::from_str(&format!("{text}{}", closers(frames))).ok()
    };
    if scanned.in_string {
        // A value's string is kept as far as it came; a name's has no value, and is left out.
        let mut kept = text;
        for _ in 0..TRIES {
            if let Some(value) = closed(&format!("{kept}\""), &scanned.frames) {
                let path = path(&scanned.frames, true);
                return Some(Cut {
                    value,
                    path,
                    text: true,
                    whole: false,
                });
            }
            match kept.rfind('\\') {
                Some(escape) => kept = kept.get(..escape)?,
                None => break,
            }
        }
    } else if !unfinished(text)
        && let Some(value) = closed(text, &scanned.frames)
    {
        // It stops where a value ends.
        let path = path(&scanned.frames, false);
        return Some(Cut {
            value,
            path,
            text: false,
            whole: false,
        });
    }
    // The last place the text can be cut at: after an opening bracket, or before a comma.
    let kept = text.get(..scanned.safe?)?;
    let frames = scan(kept).frames;
    let value = closed(kept, &frames)?;
    let path = path(&frames, false);
    Some(Cut {
        value,
        path,
        text: false,
        whole: false,
    })
}

/// An object or an array still open where the text stops, and where in it the text stands.
enum Frame {
    /// The name last read (none where it could not be), and whether a name is being read.
    Object {
        key: Option<String>,
        naming: bool,
    },
    Array {
        index: usize,
    },
}

/// What is open where a text stops.
struct Scan {
    /// Outermost first.
    frames: Vec<Frame>,
    in_string: bool,
    /// The last place the text can be cut at.
    safe: Option<usize>,
}

fn scan(text: &str) -> Scan {
    let mut frames = Vec::new();
    let mut safe = None;
    let (mut in_string, mut escaped, mut started) = (false, false, 0);
    for (at, c) in text.char_indices() {
        match (in_string, escaped, c) {
            (true, true, _) => escaped = false,
            (true, false, '\\') => escaped = true,
            (true, false, '"') => {
                in_string = false;
                if let Some(Frame::Object { key, naming: true }) = frames.last_mut() {
                    let raw = text.get(started..=at);
                    *key = raw.and_then(|raw| serde_json::from_str(raw).ok());
                }
            }
            (true, false, _) => {}
            (false, _, '"') => (in_string, started) = (true, at),
            (false, _, '{') => {
                frames.push(Frame::Object {
                    key: None,
                    naming: true,
                });
                safe = Some(at + 1);
            }
            (false, _, '[') => {
                frames.push(Frame::Array { index: 0 });
                safe = Some(at + 1);
            }
            (false, _, '}' | ']') => {
                frames.pop();
            }
            (false, _, ':') => {
                if let Some(Frame::Object { naming, .. }) = frames.last_mut() {
                    *naming = false;
                }
            }
            (false, _, ',') => {
                safe = Some(at);
                match frames.last_mut() {
                    Some(Frame::Object { key, naming }) => (*key, *naming) = (None, true),
                    Some(Frame::Array { index }) => *index += 1,
                    None => {}
                }
            }
            (false, _, _) => {}
        }
    }
    Scan {
        frames,
        in_string,
        safe,
    }
}

/// What closes every frame, innermost first.
fn closers(frames: &[Frame]) -> String {
    let closer = |frame: &Frame| match frame {
        Frame::Object { .. } => '}',
        Frame::Array { .. } => ']',
    };
    frames.iter().rev().map(closer).collect()
}

/// The steps from the root to the innermost open container, and one more into the value it
/// stands in where that is an open string (`into`). None where a name on the way was not read.
fn path(frames: &[Frame], into: bool) -> Option<Vec<Step>> {
    let steps = match into {
        true => frames.len(),
        false => frames.len().saturating_sub(1),
    };
    let step = |frame: &Frame| match frame {
        Frame::Object { key, .. } => key.clone().map(Step::Key),
        Frame::Array { index } => Some(Step::Index(*index)),
    };
    frames.iter().take(steps).map(step).collect()
}

/// Whether `text` stops in a number, which may go on: a digit, a sign, a point, or an exponent's
/// letter after a digit.
fn unfinished(text: &str) -> bool {
    let mut last = text.trim_end().chars().rev();
    match (last.next(), last.next()) {
        (Some('0'..='9' | '.' | '-' | '+'), _) => true,
        (Some('e' | 'E'), Some('0'..='9' | '.')) => true,
        (Some(_) | None, Some(_) | None) => false,
    }
}

#[cfg(test)]
#[path = "cut_tests.rs"]
mod tests;
