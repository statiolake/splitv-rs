use itertools::Itertools;
use std::cmp;
use unicode_width::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pane {
    lines: Vec<String>,
    width: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct WrappedPane {
    lines: Vec<WrappedLine>,
    width: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct WrappedLine {
    wrapped_lines_in_line: Vec<String>,
    width: usize,
}

impl WrappedLine {
    fn new(width: usize) -> WrappedLine {
        WrappedLine {
            wrapped_lines_in_line: Vec::new(),
            width,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct LineWise {
    lines_in_panes_in_line: Vec<WrappedLine>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct MergedLine(WrappedLine);

fn map_collect<T: IntoIterator, U, F: FnMut(T::Item) -> U>(iter: T, f: F) -> Vec<U> {
    iter.into_iter().map(f).collect()
}

pub fn vsplit(panes: Vec<Pane>, delims: Vec<String>) -> Vec<String> {
    assert_eq!(
        panes.len(),
        delims.len() + 1,
        "The number of delimiters is incorrect."
    );

    let wrapped_panes = map_collect(panes, wrap_pane);
    let linewise = transpose(wrapped_panes);
    let merged_lines = map_collect(linewise, |lw| merge(lw, &delims));

    merged_lines
        .into_iter()
        .flat_map(|MergedLine(wl)| wl.wrapped_lines_in_line.into_iter())
        .collect()
}

fn wrap_pane(pane: Pane) -> WrappedPane {
    let width = pane.width;
    let lines = pane
        .lines
        .into_iter()
        .map(|line| wrap_at(&line, width))
        .collect();
    WrappedPane { lines, width }
}

fn wrap_at(original: &str, width: usize) -> WrappedLine {
    let mut wrapped_lines_in_line = Vec::new();
    let mut current = String::new();
    let mut current_width = 0;

    for ch in original.chars() {
        let ch_width = ch.width().unwrap_or(0);
        if current_width + ch_width > width {
            wrapped_lines_in_line.push(current);
            current = ch.to_string();
            current_width = ch_width;
        } else {
            current.push(ch);
            current_width += ch_width;
        }
    }

    if !current.is_empty() {
        wrapped_lines_in_line.push(current);
    }

    WrappedLine {
        wrapped_lines_in_line,
        width,
    }
}

fn transpose(wrapped: Vec<WrappedPane>) -> Vec<LineWise> {
    let max_lines = wrapped
        .iter()
        .fold(0, |max, w| cmp::max(max, w.lines.len()));

    let mut linewises = Vec::new();
    for line_no in 0..max_lines {
        let extract_line = |w: &WrappedPane| {
            w.lines
                .get(line_no)
                .cloned()
                .unwrap_or_else(|| WrappedLine::new(w.width))
        };

        let lines_in_panes_in_line = map_collect(&wrapped, extract_line);
        linewises.push(LineWise {
            lines_in_panes_in_line,
        });
    }

    linewises
}

fn merge(
    LineWise {
        lines_in_panes_in_line,
    }: LineWise,
    delims: &[String],
) -> MergedLine {
    let max_lines = lines_in_panes_in_line.iter().fold(0, |max, pane| {
        cmp::max(max, pane.wrapped_lines_in_line.len())
    });

    let wrapped_lines_in_line = map_collect(0..max_lines, |i| {
        let extract_line = |wl: &WrappedLine| {
            wl.wrapped_lines_in_line
                .get(i)
                .cloned()
                .map(|line| {
                    let width = line.width();
                    line + &" ".repeat(wl.width - width)
                })
                .unwrap_or_else(|| " ".repeat(wl.width))
        };

        lines_in_panes_in_line
            .iter()
            .map(extract_line)
            .interleave(delims.iter().cloned())
            .join("")
    });

    let width = lines_in_panes_in_line
        .iter()
        .map(|wl| wl.width)
        .chain(delims.iter().map(|delim| delim.width()))
        .sum();

    MergedLine(WrappedLine {
        wrapped_lines_in_line,
        width,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(non_snake_case)]
    fn S(s: &str) -> String {
        s.into()
    }

    #[test]
    fn wrap() {
        assert_eq!(
            wrap_at("abcdefghijklmnopqrstuvwxyz", 5),
            WrappedLine {
                wrapped_lines_in_line: vec![
                    S("abcde"),
                    S("fghij"),
                    S("klmno"),
                    S("pqrst"),
                    S("uvwxy"),
                    S("z")
                ],
                width: 5
            }
        );

        assert_eq!(
            wrap_at("あいうえおかきくけこ", 5),
            WrappedLine {
                wrapped_lines_in_line: vec![
                    S("あい"),
                    S("うえ"),
                    S("おか"),
                    S("きく"),
                    S("けこ")
                ],
                width: 5
            }
        );
    }

    #[test]
    fn display() {
        let lines = Pane {
            lines: vec![S("1"), S("2"), S("3"), S("4")],
            width: 3,
        };
        let pane1 = Pane { lines: vec![S("Lorem ipsum dolor sit amet, consectetur adipiscing elit,"), S("sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat."), S("Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur."), S("Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.")], width: 30};
        let pane2 = Pane { lines: vec![S("Lorem ipsum dolor sit amet, consectetur adipiscing elit,"), S("sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat."), S("Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur."), S("とりあえずここに自然に日本語がまぎれてきてもたぶんいい感じに切ってくれるはずだよね")], width: 40};

        println!(
            "{}",
            vsplit(vec![lines, pane1, pane2], vec![S(" | "), S(" | ")]).join("\n")
        );
    }
}
