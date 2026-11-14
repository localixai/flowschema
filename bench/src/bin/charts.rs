//! The comparison chart the README shows, drawn from the result files, never typed in:
//! `results/micro.json`, `results/corpus.json` and `results/size.json`. Writes
//! `results/charts/compare.svg`, which `charts.sh` rasterizes at twice its size into
//! `compare.png`: one light card, the same on a light page, a dark one and anywhere it is reposted.
//!
//! Four sections, so a time is never read against a size: a request (its schema compiled and its
//! answer checked), checks once compiled, MaskBench's real schemas (their times and how often
//! each validator is wrong), and what a program takes on. Each panel is scaled to its own longest
//! bar, and the number beside a bar is the measurement.

use flowschema_bench::{results, shown, thousands};
use serde_json::Value;

/// Every panel lists the two in this order, so a row means the same library everywhere.
const LIBRARIES: [(&str, bool); 2] = [("flowschema", true), ("jsonschema", false)];

const PAGE: &str = "#ffffff";
const TEXT: &str = "#1c1c1b";
const MUTED: &str = "#5f5f5b";
const BAR: &str = "#d6d6d3";
const OURS: &str = "#6e56cf";

const COLUMNS: usize = 3;
const PANEL_W: f64 = 372.0;
const GAP_X: f64 = 42.0;
const ROW_H: f64 = 26.0;
const HEAD_H: f64 = 54.0;
const GAP_Y: f64 = 30.0;
const SECTION_H: f64 = 58.0;
const SECTION_GAP: f64 = 26.0;
const LABEL_W: f64 = 92.0;
const VALUE_W: f64 = 92.0;
/// The shortest bar drawn, so the faster row is a bar and not a hairline beside a slow one.
const MIN_BAR: f64 = 6.0;
const PAD: f64 = 36.0;
const CORNER: f64 = 16.0;

/// How a panel's numbers read.
#[derive(Clone, Copy)]
enum Unit {
    /// Nanoseconds, written as a table writes a time (`398 ns`, `6.00 µs`).
    Time,
    /// Milliseconds, to the decimals given, as the result's table writes them.
    Ms(usize),
    /// Bytes, written in KB.
    Kb,
    /// A count.
    Count,
}

struct Panel {
    title: String,
    note: String,
    unit: Unit,
    /// flowschema's value, then jsonschema's.
    values: [f64; 2],
}

struct Section {
    title: &'static str,
    note: &'static str,
    panels: Vec<Panel>,
}

fn read(name: &str) -> Value {
    let text = std::fs::read_to_string(results(name)).unwrap_or_else(|why| {
        panic!("results/{name}: {why}; run the benchmark that writes it first")
    });
    serde_json::from_str(&text).unwrap_or_else(|why| panic!("results/{name}: {why}"))
}

fn number(value: &Value, path: &[&str]) -> f64 {
    let found = path.iter().fold(value, |at, key| &at[*key]);
    found
        .as_f64()
        .unwrap_or_else(|| panic!("no number at {path:?}"))
}

/// A row of one of micro.json's tables (`checks`, `compiled`, `requests`) by its name.
fn row<'a>(micro: &'a Value, table: &str, name: &str) -> &'a Value {
    (micro[table].as_array().into_iter().flatten())
        .find(|row| row["name"] == name)
        .unwrap_or_else(|| panic!("micro.json has no row {name:?} in {table}"))
}

fn timed(micro: &Value, table: &str, name: &str, title: &str, note: &str) -> Panel {
    let row = row(micro, table, name);
    Panel {
        title: title.into(),
        note: note.into(),
        unit: Unit::Time,
        values: [
            number(row, &["ours", "ns"]),
            number(row, &["jsonschema", "ns"]),
        ],
    }
}

fn sections() -> Vec<Section> {
    let micro = read("micro.json");
    let corpus = read("corpus.json");
    let size = read("size.json");
    let tree = row(&micro, "checks", "tree of 511 nodes by `$ref`");
    vec![
        Section {
            title: "A request",
            note: "a schema that comes with the request, compiled, and one answer checked · \
                   shorter is better",
            panels: vec![
                timed(
                    &micro,
                    "requests",
                    "review verdict",
                    "A structured answer",
                    "its schema compiled, the answer checked",
                ),
                timed(
                    &micro,
                    "requests",
                    "one tool call",
                    "A tool call",
                    "the tool's schema compiled, the call checked",
                ),
                timed(
                    &micro,
                    "compiled",
                    "tool calls",
                    "Eight tools",
                    "a union of 8 tools, compiled",
                ),
            ],
        },
        Section {
            title: "Checks, once compiled",
            note: "one check of what a model wrote, the median of 31 rounds · shorter is better",
            panels: vec![
                timed(
                    &micro,
                    "checks",
                    "review verdict, valid",
                    "A structured answer",
                    "a review's verdict, valid",
                ),
                timed(
                    &micro,
                    "checks",
                    "review verdict, invalid",
                    "The same, invalid",
                    "stops at the first failure",
                ),
                timed(
                    &micro,
                    "checks",
                    "64 tool calls, `oneOf` of 8",
                    "Tool calls",
                    "64 calls, a union of 8 tools",
                ),
                timed(
                    &micro,
                    "checks",
                    "1,000 rows",
                    "A table",
                    "1,000 rows an answer lists",
                ),
                timed(
                    &micro,
                    "checks",
                    "500 objects × 20 fields",
                    "Wide objects",
                    "500 objects of 20 fields each",
                ),
                timed(
                    &micro,
                    "checks",
                    "tree of 511 nodes by `$ref`",
                    "A recursive schema",
                    "a tree of 511 nodes by $ref",
                ),
            ],
        },
        Section {
            title: "Real LLM tasks",
            note: "MaskBench: schemas of function calling, GitHub, Kubernetes, JSON Schema Store · \
                   less is better",
            panels: vec![
                Panel {
                    title: "Compiling every schema".into(),
                    note: format!(
                        "{} schemas, each compiled once",
                        thousands(number(&corpus, &["files"]) as usize)
                    ),
                    unit: Unit::Ms(1),
                    values: [
                        number(&corpus, &["compileMs", "ours"]),
                        number(&corpus, &["compileMs", "jsonschema"]),
                    ],
                },
                Panel {
                    title: "Checking every answer".into(),
                    note: format!(
                        "the answers of the {} schemas both check whole",
                        thousands(number(&corpus, &["timed"]) as usize)
                    ),
                    unit: Unit::Ms(2),
                    values: [
                        number(&corpus, &["checkMs", "ours"]),
                        number(&corpus, &["checkMs", "jsonschema"]),
                    ],
                },
                Panel {
                    title: "Wrong verdicts".into(),
                    note: format!(
                        "answers judged against their label, of {}",
                        thousands(number(&corpus, &["answers"]) as usize)
                    ),
                    unit: Unit::Count,
                    values: [
                        number(&corpus, &["answers"]) - number(&corpus, &["oursRight"]),
                        number(&corpus, &["answers"]) - number(&corpus, &["jsonschemaRight"]),
                    ],
                },
            ],
        },
        Section {
            title: "Weight",
            note: "what a program takes on · smaller is better",
            panels: vec![
                Panel {
                    title: "Added to a binary".into(),
                    note: "a release built for size, stripped".into(),
                    unit: Unit::Kb,
                    values: [
                        number(&size, &["flowschema"]),
                        number(&size, &["jsonschema"]),
                    ],
                },
                Panel {
                    title: "Crates it adds".into(),
                    note: "to a build that reads JSON, itself included".into(),
                    unit: Unit::Count,
                    values: [
                        number(&size, &["crates", "flowschema"]),
                        number(&size, &["crates", "jsonschema"]),
                    ],
                },
                Panel {
                    title: "Allocations on a check".into(),
                    note: "the recursive schema's tree, checked once".into(),
                    unit: Unit::Count,
                    values: [
                        number(tree, &["ours", "allocations"]),
                        number(tree, &["jsonschema", "allocations"]),
                    ],
                },
            ],
        },
    ]
}

fn format(value: f64, unit: Unit) -> String {
    match unit {
        Unit::Time => shown(value),
        Unit::Ms(decimals) => {
            let text = format!("{value:.decimals$}");
            let (whole, fraction) = text.split_once('.').unwrap_or((&text, ""));
            let whole = thousands(whole.parse().unwrap_or(0));
            match fraction.is_empty() {
                true => format!("{whole} ms"),
                false => format!("{whole}.{fraction} ms"),
            }
        }
        // A half rounded to the even, as size.md rounds it: the two never differ by one.
        Unit::Kb => {
            let kb = format!("{:.0}", value / 1024.0);
            format!("{} KB", thousands(kb.parse().unwrap_or(0)))
        }
        Unit::Count => thousands(value.round() as usize),
    }
}

fn escape(text: &str) -> String {
    text.replace('&', "&amp;").replace('<', "&lt;")
}

fn main() {
    let sections = sections();
    let panel_h = HEAD_H + LIBRARIES.len() as f64 * ROW_H;
    let width = COLUMNS as f64 * PANEL_W + (COLUMNS - 1) as f64 * GAP_X;
    let mut parts = vec![];
    let mut top = 0.0;
    for section in &sections {
        if top > 0.0 {
            top += SECTION_GAP;
        }
        parts.push(format!(
            r#"<text x="0" y="{}" class="section">{}</text>"#,
            top + 16.0,
            escape(section.title)
        ));
        parts.push(format!(
            r#"<text x="0" y="{}" class="note">{}</text>"#,
            top + 35.0,
            escape(section.note)
        ));
        parts.push(format!(
            r#"<line x1="0" y1="{y}" x2="{width}" y2="{y}" stroke="{BAR}"/>"#,
            y = top + 46.0
        ));
        top += SECTION_H;
        for (index, panel) in section.panels.iter().enumerate() {
            let x = (index % COLUMNS) as f64 * (PANEL_W + GAP_X);
            let y = top + (index / COLUMNS) as f64 * (panel_h + GAP_Y);
            let longest = panel
                .values
                .iter()
                .copied()
                .fold(0.0, f64::max)
                .max(f64::MIN_POSITIVE);
            let track = PANEL_W - LABEL_W - VALUE_W;
            parts.push(format!(
                r#"<text x="{x}" y="{}" class="title">{}</text>"#,
                y + 18.0,
                escape(&panel.title)
            ));
            parts.push(format!(
                r#"<text x="{x}" y="{}" class="note">{}</text>"#,
                y + 37.0,
                escape(&panel.note)
            ));
            for (row, ((label, ours), value)) in LIBRARIES.iter().zip(panel.values).enumerate() {
                let row_top = y + HEAD_H + row as f64 * ROW_H;
                let mid = row_top + ROW_H / 2.0;
                let bar = match value > 0.0 {
                    true => (value / longest * track).max(MIN_BAR),
                    false => 0.0,
                };
                let class = if *ours { "ours" } else { "label" };
                let fill = if *ours { OURS } else { BAR };
                parts.push(format!(
                    r#"<text x="{}" y="{}" text-anchor="end" class="{class}">{label}</text>"#,
                    x + LABEL_W - 10.0,
                    mid + 4.0
                ));
                if bar > 0.0 {
                    parts.push(format!(
                        r#"<rect x="{}" y="{}" width="{bar:.1}" height="{}" rx="3" fill="{fill}"/>"#,
                        x + LABEL_W,
                        row_top + 5.0,
                        ROW_H - 10.0
                    ));
                }
                let value_class = if *ours { "ours" } else { "value" };
                parts.push(format!(
                    r#"<text x="{}" y="{}" class="{value_class}">{}</text>"#,
                    x + LABEL_W + bar + if bar > 0.0 { 8.0 } else { 0.0 },
                    mid + 4.0,
                    format(value, panel.unit)
                ));
            }
        }
        let rows = section.panels.len().div_ceil(COLUMNS) as f64;
        top += rows * panel_h + (rows - 1.0) * GAP_Y;
    }
    let foot = top + 34.0;
    parts.push(format!(
        r#"<line x1="0" y1="{y}" x2="{width}" y2="{y}" stroke="{BAR}"/>"#,
        y = foot - 18.0
    ));
    parts.push(format!(
        r#"<text x="0" y="{foot}" class="note">Each panel has its own scale; the number beside a bar is the measurement. Every figure comes from bench/results/.</text>"#
    ));
    parts.push(format!(
        r#"<text x="0" y="{}" class="note">Times built for speed (opt-level 3, fat LTO), weight built for size; jsonschema 0.60 with formats asserted as flowschema asserts them.</text>"#,
        foot + 18.0
    ));
    let (w, h) = (width + PAD * 2.0, foot + 26.0 + PAD * 2.0);
    let svg = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{w}" height="{h}" viewBox="0 0 {w} {h}">
<defs><style>
text {{ font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", "Noto Sans", Helvetica, Arial, sans-serif; fill: {TEXT}; }}
.section {{ font-size: 17px; font-weight: 700; }}
.title {{ font-size: 15px; font-weight: 600; }}
.note {{ font-size: 12px; fill: {MUTED}; }}
.label, .value {{ font-size: 12.5px; }}
.value {{ fill: {MUTED}; font-variant-numeric: tabular-nums; }}
.ours {{ font-size: 12.5px; font-weight: 600; fill: {OURS}; font-variant-numeric: tabular-nums; }}
</style></defs>
<rect width="{w}" height="{h}" rx="{CORNER}" fill="{PAGE}"/>
<g transform="translate({PAD} {PAD})">
{}
</g>
</svg>
"#,
        parts.join("\n")
    );
    let dir = results("charts");
    std::fs::create_dir_all(&dir).unwrap_or_else(|why| panic!("results/charts: {why}"));
    std::fs::write(dir.join("compare.svg"), svg).unwrap_or_else(|why| panic!("compare.svg: {why}"));
    println!("{w} {h}");
}
