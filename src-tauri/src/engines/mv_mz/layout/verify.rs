//! Fit check on the final values, run at export before anything is written.
//!
//! A fitted message or description must draw without a line wider than its
//! box, without a blank page, with no control characters besides `\n` and
//! U+000C, and with exactly the translated text: the same visible
//! characters (whitespace aside) and the same escape codes (colour and font
//! size codes may be repeated at page starts).

use serde::Serialize;

use super::font::Metrics;
use super::simulate::{simulate, BoxSpec};
use super::text::{tokenize, Inserts, Token, TokenKind, PAGE_BREAK};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum FitProblem {
    /// A line is wider than the box (or below a help box).
    Overflow { page: usize, line: usize },
    /// A page with nothing drawn on it.
    BlankPage { page: usize },
    /// A control character other than `\n` / U+000C.
    ControlCharacter { code: u32 },
    /// The text or its escape codes differ from the translation.
    TextChanged,
}

/// What must survive fitting: visible characters without whitespace, and
/// every escape code except colour/font codes (re-emitted at page starts).
fn essence(text: &str, inserts: &Inserts) -> (String, Vec<String>, usize) {
    let mut visible = String::new();
    let mut codes = Vec::new();
    let mut colors = 0;
    for token in tokenize(text, inserts) {
        match &token.kind {
            TokenKind::Text => visible.extend(token.raw.chars().filter(|c| !c.is_whitespace())),
            TokenKind::Newline | TokenKind::PageBreak | TokenKind::FontSize(_) => {}
            TokenKind::Color(_) => colors += 1,
            _ => codes.push(normalized_code(&token)),
        }
    }
    (visible, codes, colors)
}

fn normalized_code(token: &Token) -> String {
    token.raw.to_ascii_uppercase()
}

/// Check a fitted text (`output`) drawn in `spec` against the translated
/// text it came from. `allow_overflow`: the user chose to leave it.
pub fn check(
    translated: &str,
    output: &str,
    spec: &BoxSpec,
    metrics: &Metrics,
    inserts: &Inserts,
    allow_overflow: bool,
) -> Vec<FitProblem> {
    let mut problems = Vec::new();
    for c in output.chars() {
        if (c as u32) < 0x20 && c != '\n' && c != PAGE_BREAK {
            problems.push(FitProblem::ControlCharacter { code: c as u32 });
        }
    }
    let (visible_in, codes_in, colors_in) = essence(translated, inserts);
    let (visible_out, codes_out, colors_out) = essence(output, inserts);
    if visible_in != visible_out || codes_in != codes_out || colors_out < colors_in {
        problems.push(FitProblem::TextChanged);
    }
    let pages = simulate(output, spec, metrics, inserts);
    for (page_index, page) in pages.iter().enumerate() {
        if !page.is_visible() && pages.len() > 1 {
            problems.push(FitProblem::BlankPage { page: page_index });
        }
        if allow_overflow {
            continue;
        }
        for (line_index, line) in page.lines.iter().enumerate() {
            if line.overflow || line.clipped {
                problems.push(FitProblem::Overflow {
                    page: page_index,
                    line: line_index,
                });
            }
        }
    }
    problems
}

#[cfg(test)]
mod tests {
    use super::*;

    fn spec() -> BoxSpec {
        BoxSpec {
            text_x: 4.0,
            limit: 780.0,
            height: 152.0,
            main_font: 26.0,
            line_spacing: 10.0,
            paging: true,
        }
    }

    fn problems(translated: &str, output: &str) -> Vec<FitProblem> {
        check(
            translated,
            output,
            &spec(),
            &Metrics::Estimated,
            &Inserts::default(),
            false,
        )
    }

    #[test]
    fn accepts_rewrapped_text_with_repeated_codes() {
        assert!(problems("a b \\C[2]c d", "a b\n\\C[2]c\u{000C}\n\\FS[26]\\C[2]d").is_empty());
    }

    #[test]
    fn rejects_lost_or_added_text_and_codes() {
        assert_eq!(problems("a b", "a"), vec![FitProblem::TextChanged]);
        assert_eq!(problems("a \\I[3]", "a"), vec![FitProblem::TextChanged]);
        assert_eq!(problems("a \\C[2]b", "a b"), vec![FitProblem::TextChanged]);
        assert_eq!(
            problems("ab", "a\u{0007}b"),
            vec![
                FitProblem::ControlCharacter { code: 7 },
                FitProblem::TextChanged
            ]
        );
    }

    #[test]
    fn rejects_overflow_and_blank_pages() {
        let long = "x".repeat(70);
        assert_eq!(
            problems(&long, &long),
            vec![FitProblem::Overflow { page: 0, line: 0 }]
        );
        assert!(check(
            &long,
            &long,
            &spec(),
            &Metrics::Estimated,
            &Inserts::default(),
            true
        )
        .is_empty());
        assert_eq!(
            problems("1 2 3 4 5", "1\n2\n3\n4\n\u{000C}5"),
            vec![FitProblem::BlankPage { page: 1 }]
        );
    }
}
