//! A JSON text the output limit cut off: what of it was written whole. A model's answer or a
//! call's arguments stop midway when the limit is reached; the fields written before the cut
//! are still there, and are what a program can act on or check.

use serde_json::Value;

/// Parses tried when the text stops inside a string: as it came, then with its last escape left
/// out, twice at most (half a pair of surrogates is two escapes).
const TRIES: usize = 3;

/// The value `text` holds, where it is whole, or what of it was written whole where it stops
/// midway: the string it stops in closed (a field's value, not its name), every object and
/// array still open closed, and what the cut left unfinished left out (a name with no value, a
/// number or a word that may go on, half an escape). `None` for a text that is not the start of
/// a JSON value.
pub fn whole(text: &str) -> Option<Value> {
    if let Ok(value) = serde_json::from_str(text) {
        return Some(value);
    }
    // The closers of what is open, innermost last, and the last place the text can be cut at
    // with what is open then: after an opening bracket, or before a comma.
    let mut open = String::new();
    let mut safe: Option<(usize, String)> = None;
    let (mut in_string, mut escaped) = (false, false);
    for (at, c) in text.char_indices() {
        match (in_string, escaped, c) {
            (true, true, _) => escaped = false,
            (true, false, '\\') => escaped = true,
            (true, false, '"') => in_string = false,
            (true, false, _) => {}
            (false, _, '"') => in_string = true,
            (false, _, '{' | '[') => {
                open.push(if c == '{' { '}' } else { ']' });
                safe = Some((at + 1, open.clone()));
            }
            (false, _, '}' | ']') => {
                open.pop();
            }
            (false, _, ',') => safe = Some((at, open.clone())),
            (false, _, _) => {}
        }
    }
    let closed = |text: &str, open: &str| -> Option<Value> {
        let ends: String = open.chars().rev().collect();
        serde_json::from_str(&format!("{text}{ends}")).ok()
    };
    if in_string {
        // A value's string is kept as far as it came; a name's has no value, and is left out.
        let mut kept = text;
        for _ in 0..TRIES {
            if let Some(value) = closed(&format!("{kept}\""), &open) {
                return Some(value);
            }
            match kept.rfind('\\') {
                Some(escape) => kept = kept.get(..escape)?,
                None => break,
            }
        }
    } else if !unfinished(text)
        && let Some(value) = closed(text, &open)
    {
        // It stops where a value ends.
        return Some(value);
    }
    let (at, open) = safe?;
    closed(text.get(..at)?, &open)
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
