use anyhow::Result;
use console::{style, Style};
use similar::{ChangeTag, TextDiff};
use std::fmt::Display;
use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

struct Line(Option<usize>);

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            None => write!(f, ""),
            Some(idx) => write!(f, "{:<4}", idx + 1),
        }
    }
}

pub fn diff_text(text1: &str, text2: &str) -> Result<String> {
    let mut output = String::new();
    let diff = TextDiff::from_lines(text1, text2);

    for (idx, group) in diff.grouped_ops(3).iter().enumerate() {
        if idx > 0 {
            output.push_str(&format!("{:-^1$}\n", "-", 80));
        }
        for op in group {
            for change in diff.iter_inline_changes(op) {
                let (sign, s) = match change.tag() {
                    ChangeTag::Delete => ("-", Style::new().red()),
                    ChangeTag::Insert => ("+", Style::new().green()),
                    ChangeTag::Equal => (" ", Style::new().dim()),
                };
                output.push_str(&format!(
                    "{}{} |{}",
                    style(Line(change.old_index())).dim(),
                    style(Line(change.new_index())).dim(),
                    s.apply_to(sign).bold(),
                ));
                for (emphasized, value) in change.iter_strings_lossy() {
                    if emphasized {
                        output.push_str(&format!("{}", s.apply_to(value).underlined().on_black()));
                    } else {
                        output.push_str(&format!("{}", s.apply_to(value)));
                    }
                }
                if change.missing_newline() {
                    output.push('\n');
                }
            }
        }
    }

    Ok(output)
}

pub fn highlight_test(text: &str, extension: &str) -> Result<String> {
    // Load these once at the start of your program
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let syntax = ps.find_syntax_by_extension(extension).unwrap();
    let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);

    let mut output = String::new();

    for line in LinesWithEndings::from(text) {
        let ranges = h.highlight_line(line, &ps).unwrap();
        let escaped = as_24_bit_terminal_escaped(&ranges[..], false);
        output.push_str(&escaped);
    }
    Ok(output)
}
