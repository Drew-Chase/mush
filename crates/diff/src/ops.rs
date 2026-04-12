use crate::cli::DiffConfig;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiffLine {
    Context(String),
    Added(String),
    Removed(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiffHunk {
    pub start1: usize,
    pub count1: usize,
    pub start2: usize,
    pub count2: usize,
    pub changes: Vec<DiffLine>,
}

/// Normalize a line for comparison according to config flags.
fn normalize_line(line: &str, config: &DiffConfig) -> String {
    let mut s = line.to_string();
    if config.ignore_all_space {
        s.retain(|c| !c.is_whitespace());
    } else if config.ignore_space_change {
        let mut result = String::new();
        let mut in_space = false;
        for c in s.chars() {
            if c.is_whitespace() {
                if !in_space {
                    result.push(' ');
                    in_space = true;
                }
            } else {
                result.push(c);
                in_space = false;
            }
        }
        s = result;
    }
    if config.ignore_case {
        s = s.to_lowercase();
    }
    s
}

/// Compute the longest common subsequence table using O(NM) DP.
fn lcs_table(a: &[String], b: &[String]) -> Vec<Vec<usize>> {
    let n = a.len();
    let m = b.len();
    let mut dp = vec![vec![0usize; m + 1]; n + 1];

    for i in 1..=n {
        for j in 1..=m {
            if a[i - 1] == b[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
    }

    dp
}

/// Backtrack through the LCS table to produce an edit script.
/// Returns a list of (tag, line_from_1, line_from_2) where tag is '=', '+', or '-'.
fn backtrack(
    dp: &[Vec<usize>],
    a: &[&str],
    b: &[&str],
    norm_a: &[String],
    norm_b: &[String],
) -> Vec<(char, Option<String>, Option<String>)> {
    let mut result = Vec::new();
    let mut i = a.len();
    let mut j = b.len();

    while i > 0 || j > 0 {
        if i > 0 && j > 0 && norm_a[i - 1] == norm_b[j - 1] {
            result.push(('=', Some(a[i - 1].to_string()), Some(b[j - 1].to_string())));
            i -= 1;
            j -= 1;
        } else if j > 0 && (i == 0 || dp[i][j - 1] >= dp[i - 1][j]) {
            result.push(('+', None, Some(b[j - 1].to_string())));
            j -= 1;
        } else {
            result.push(('-', Some(a[i - 1].to_string()), None));
            i -= 1;
        }
    }

    result.reverse();
    result
}

/// Compute diff hunks between two sets of lines.
pub fn compute_diff(lines1: &[&str], lines2: &[&str], config: &DiffConfig) -> Vec<DiffHunk> {
    let norm1: Vec<String> = lines1.iter().map(|l| normalize_line(l, config)).collect();
    let norm2: Vec<String> = lines2.iter().map(|l| normalize_line(l, config)).collect();

    let dp = lcs_table(&norm1, &norm2);
    let edits = backtrack(&dp, lines1, lines2, &norm1, &norm2);

    // Determine context size
    let ctx = if config.unified.is_some() {
        config.unified.unwrap_or(3)
    } else if config.context.is_some() {
        config.context.unwrap_or(3)
    } else if config.github {
        3
    } else if config.side_by_side {
        0
    } else {
        // default: no context (normal format)
        0
    };

    // For unified/context/github formats, we group changes with context
    if config.unified.is_some() || config.context.is_some() || config.github {
        return build_hunks_with_context(&edits, ctx);
    }

    // For side-by-side and normal format, build hunks without context
    build_hunks_no_context(&edits)
}

/// Build hunks grouping consecutive changes (no context lines).
fn build_hunks_no_context(
    edits: &[(char, Option<String>, Option<String>)],
) -> Vec<DiffHunk> {
    let mut hunks = Vec::new();
    let mut line1 = 0usize;
    let mut line2 = 0usize;
    let mut i = 0;

    while i < edits.len() {
        let (tag, _, _) = &edits[i];
        if *tag == '=' {
            line1 += 1;
            line2 += 1;
            i += 1;
            continue;
        }

        // Start of a change group
        let start1 = line1;
        let start2 = line2;
        let mut changes = Vec::new();

        while i < edits.len() && edits[i].0 != '=' {
            match edits[i].0 {
                '-' => {
                    changes.push(DiffLine::Removed(edits[i].1.clone().unwrap()));
                    line1 += 1;
                }
                '+' => {
                    changes.push(DiffLine::Added(edits[i].2.clone().unwrap()));
                    line2 += 1;
                }
                _ => unreachable!(),
            }
            i += 1;
        }

        hunks.push(DiffHunk {
            start1: start1 + 1,
            count1: line1 - start1,
            start2: start2 + 1,
            count2: line2 - start2,
            changes,
        });
    }

    hunks
}

/// Build hunks with surrounding context lines.
fn build_hunks_with_context(
    edits: &[(char, Option<String>, Option<String>)],
    ctx: usize,
) -> Vec<DiffHunk> {
    // First, find the indices of all change edits
    let change_indices: Vec<usize> = edits
        .iter()
        .enumerate()
        .filter(|(_, (tag, _, _))| *tag != '=')
        .map(|(i, _)| i)
        .collect();

    if change_indices.is_empty() {
        return Vec::new();
    }

    // Group changes that are within 2*ctx of each other
    let mut groups: Vec<(usize, usize)> = Vec::new(); // (first_change_idx, last_change_idx)
    let mut group_start = change_indices[0];
    let mut group_end = change_indices[0];

    for &ci in &change_indices[1..] {
        // Count context lines between group_end and ci
        let context_between = edits[group_end + 1..ci]
            .iter()
            .filter(|(tag, _, _)| *tag == '=')
            .count();
        if context_between <= 2 * ctx {
            group_end = ci;
        } else {
            groups.push((group_start, group_end));
            group_start = ci;
            group_end = ci;
        }
    }
    groups.push((group_start, group_end));

    // Build hunks from groups
    let mut hunks = Vec::new();
    for (gs, ge) in groups {
        // Find start of hunk (ctx lines before first change)
        let hunk_start = if gs >= ctx {
            // Count backwards ctx context lines
            let mut start = gs;
            let mut context_count = 0;
            while start > 0 && context_count < ctx {
                start -= 1;
                if edits[start].0 == '=' {
                    context_count += 1;
                }
            }
            start
        } else {
            0
        };

        // Find end of hunk (ctx lines after last change)
        let hunk_end = {
            let mut end = ge;
            let mut context_count = 0;
            while end + 1 < edits.len() && context_count < ctx {
                end += 1;
                if edits[end].0 == '=' {
                    context_count += 1;
                }
            }
            end
        };

        let mut changes = Vec::new();
        let mut l1 = 0usize;
        let mut l2 = 0usize;

        // Count lines before hunk_start
        for edit in &edits[..hunk_start] {
            match edit.0 {
                '=' => {
                    l1 += 1;
                    l2 += 1;
                }
                '-' => l1 += 1,
                '+' => l2 += 1,
                _ => {}
            }
        }

        let start1 = l1;
        let start2 = l2;

        for edit in &edits[hunk_start..=hunk_end] {
            match edit.0 {
                '=' => {
                    changes.push(DiffLine::Context(edit.1.clone().unwrap()));
                    l1 += 1;
                    l2 += 1;
                }
                '-' => {
                    changes.push(DiffLine::Removed(edit.1.clone().unwrap()));
                    l1 += 1;
                }
                '+' => {
                    changes.push(DiffLine::Added(edit.2.clone().unwrap()));
                    l2 += 1;
                }
                _ => {}
            }
        }

        hunks.push(DiffHunk {
            start1: start1 + 1,
            count1: l1 - start1,
            start2: start2 + 1,
            count2: l2 - start2,
            changes,
        });
    }

    hunks
}

/// Format hunks as unified diff output.
pub fn format_unified(hunks: &[DiffHunk], file1: &str, file2: &str, color: bool) -> Vec<String> {
    let mut output = Vec::new();
    if hunks.is_empty() {
        return output;
    }

    let (red, green, cyan, reset) = if color {
        ("\x1b[31m", "\x1b[32m", "\x1b[36m", "\x1b[0m")
    } else {
        ("", "", "", "")
    };

    output.push(format!("{red}--- a/{file1}{reset}"));
    output.push(format!("{green}+++ b/{file2}{reset}"));

    for hunk in hunks {
        output.push(format!(
            "{cyan}@@ -{},{} +{},{} @@{reset}",
            hunk.start1, hunk.count1, hunk.start2, hunk.count2
        ));
        for change in &hunk.changes {
            match change {
                DiffLine::Context(line) => output.push(format!(" {line}")),
                DiffLine::Removed(line) => output.push(format!("{red}-{line}{reset}")),
                DiffLine::Added(line) => output.push(format!("{green}+{line}{reset}")),
            }
        }
    }

    output
}

/// Format hunks in GitHub-style diff format with dual line numbers and color-coded +/-.
pub fn format_github(hunks: &[DiffHunk], file1: &str, file2: &str, color: bool) -> Vec<String> {
    let mut output = Vec::new();
    if hunks.is_empty() {
        return output;
    }

    let (red_bg, green_bg, cyan, reset, dim) = if color {
        ("\x1b[41m", "\x1b[42m", "\x1b[36m", "\x1b[0m", "\x1b[2m")
    } else {
        ("", "", "", "", "")
    };

    output.push(format!("{dim}--- a/{file1}{reset}"));
    output.push(format!("{dim}+++ b/{file2}{reset}"));

    for hunk in hunks {
        output.push(format!(
            "{cyan}@@ -{},{} +{},{} @@{reset}",
            hunk.start1, hunk.count1, hunk.start2, hunk.count2
        ));

        let mut old_line = hunk.start1;
        let mut new_line = hunk.start2;

        for change in &hunk.changes {
            match change {
                DiffLine::Context(line) => {
                    output.push(format!("{dim}{old_line:>4} {new_line:>4}{reset}   {line}"));
                    old_line += 1;
                    new_line += 1;
                }
                DiffLine::Removed(line) => {
                    output.push(format!("{red_bg}{old_line:>4}     {reset} {red_bg}-{reset} {red_bg}{line}{reset}"));
                    old_line += 1;
                }
                DiffLine::Added(line) => {
                    output.push(format!("{green_bg}     {new_line:>4}{reset} {green_bg}+{reset} {green_bg}{line}{reset}"));
                    new_line += 1;
                }
            }
        }
    }

    output
}

/// Format hunks as normal diff output (e.g., "1,3c4,6").
pub fn format_normal(hunks: &[DiffHunk]) -> Vec<String> {
    let mut output = Vec::new();

    for hunk in hunks {
        let has_removed = hunk.changes.iter().any(|c| matches!(c, DiffLine::Removed(_)));
        let has_added = hunk.changes.iter().any(|c| matches!(c, DiffLine::Added(_)));

        let tag = if has_removed && has_added {
            'c'
        } else if has_removed {
            'd'
        } else {
            'a'
        };

        let left = format_range(hunk.start1, hunk.count1);
        let right = format_range(hunk.start2, hunk.count2);

        output.push(format!("{left}{tag}{right}"));

        for change in &hunk.changes {
            match change {
                DiffLine::Removed(line) => output.push(format!("< {line}")),
                DiffLine::Added(line) => output.push(format!("> {line}")),
                DiffLine::Context(_) => {}
            }
        }

        if has_removed && has_added {
            // Insert separator between removed and added
            let last = output.len();
            // Find where removed lines end and added lines begin
            let mut insert_pos = None;
            for (idx, out_line) in output.iter().enumerate().rev() {
                if out_line.starts_with("> ") && insert_pos.is_none() {
                    // keep going
                } else if out_line.starts_with("< ") {
                    insert_pos = Some(idx + 1);
                    break;
                } else if !out_line.starts_with("> ") {
                    break;
                }
            }
            if let Some(pos) = insert_pos
                && pos < last
            {
                output.insert(pos, "---".to_string());
            }
        }
    }

    output
}

/// Format hunks in side-by-side format.
pub fn format_side_by_side(hunks: &[DiffHunk], lines1: &[&str], lines2: &[&str], width: usize) -> Vec<String> {
    let col_width = (width - 3) / 2; // 3 for " X " separator
    let mut output = Vec::new();

    // Rebuild full side-by-side view from the original lines and hunks
    let mut pos1 = 0usize;
    let mut pos2 = 0usize;

    for hunk in hunks {
        let hunk_start1 = hunk.start1 - 1;
        let hunk_start2 = hunk.start2 - 1;

        // Print equal lines before this hunk
        while pos1 < hunk_start1 && pos2 < hunk_start2 {
            let left = pad_or_truncate(lines1[pos1], col_width);
            let right = lines2[pos2];
            output.push(format!("{left}   {right}"));
            pos1 += 1;
            pos2 += 1;
        }

        // Process hunk changes
        let mut removed: Vec<&str> = Vec::new();
        let mut added: Vec<&str> = Vec::new();

        for change in &hunk.changes {
            match change {
                DiffLine::Removed(line) => removed.push(line),
                DiffLine::Added(line) => added.push(line),
                DiffLine::Context(_) => {}
            }
        }

        let max_len = removed.len().max(added.len());
        for idx in 0..max_len {
            let has_left = idx < removed.len();
            let has_right = idx < added.len();

            match (has_left, has_right) {
                (true, true) => {
                    let left = pad_or_truncate(removed[idx], col_width);
                    output.push(format!("{left} | {}", added[idx]));
                }
                (true, false) => {
                    let left = pad_or_truncate(removed[idx], col_width);
                    output.push(format!("{left} <"));
                }
                (false, true) => {
                    let left = pad_or_truncate("", col_width);
                    output.push(format!("{left} > {}", added[idx]));
                }
                (false, false) => {}
            }
        }

        pos1 = hunk_start1 + hunk.count1;
        pos2 = hunk_start2 + hunk.count2;
    }

    // Print remaining equal lines
    while pos1 < lines1.len() && pos2 < lines2.len() {
        let left = pad_or_truncate(lines1[pos1], col_width);
        let right = lines2[pos2];
        output.push(format!("{left}   {right}"));
        pos1 += 1;
        pos2 += 1;
    }

    output
}

fn format_range(start: usize, count: usize) -> String {
    if count == 0 {
        format!("{}", start.saturating_sub(1))
    } else if count == 1 {
        format!("{start}")
    } else {
        format!("{},{}", start, start + count - 1)
    }
}

fn pad_or_truncate(s: &str, width: usize) -> String {
    let len = s.len();
    if len >= width {
        s[..width].to_string()
    } else {
        format!("{s}{}", " ".repeat(width - len))
    }
}
