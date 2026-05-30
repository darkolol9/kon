use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Style, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, Paragraph};

use crate::db::QueryResult;
use crate::repl::syntax;
use crate::repl::{App, AppState, QueryBlock, ViewMode};

const MAX_COL_WIDTH: usize = 40;
const MIN_COL_WIDTH: usize = 8;

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        let mut t: String = s.chars().take(max.saturating_sub(3)).collect();
        t.push_str("...");
        t
    }
}

fn is_numeric(s: &str) -> bool {
    if s.is_empty() || s == "NULL" {
        return false;
    }
    let s = s.trim_start_matches('-');
    if s.contains('.') {
        s.parse::<f64>().is_ok()
    } else {
        s.parse::<i64>().is_ok()
    }
}

fn detect_numeric_cols(result: &QueryResult) -> Vec<bool> {
    result
        .columns
        .iter()
        .enumerate()
        .map(|(i, _)| {
            result
                .rows
                .iter()
                .take(20)
                .filter_map(|row| row.get(i).and_then(|v| v.as_deref()))
                .filter(|v| is_numeric(v))
                .count()
                > 2
        })
        .collect()
}

fn compute_col_widths(result: &QueryResult, available_width: usize) -> Vec<usize> {
    let col_count = result.columns.len();
    if col_count == 0 {
        return vec![];
    }

    let mut natural_widths: Vec<usize> = result
        .columns
        .iter()
        .enumerate()
        .map(|(i, col)| {
            let max_data = result
                .rows
                .iter()
                .filter_map(|row| row.get(i).and_then(|v| v.as_deref().map(|s| s.len())))
                .max()
                .unwrap_or(0);
            std::cmp::max(max_data, col.len()).min(MAX_COL_WIDTH)
        })
        .collect();

    let total_borders = col_count.saturating_sub(1);
    let available = available_width.saturating_sub(total_borders + 2);

    let total_natural: usize = natural_widths.iter().sum();
    if total_natural == 0 {
        return vec![MIN_COL_WIDTH; col_count];
    }

    let gutter_width = 6;

    if total_natural + gutter_width <= available {
        natural_widths.insert(0, gutter_width);
        return natural_widths;
    }

    let available_for_data = available.saturating_sub(gutter_width);

    let excess = total_natural.saturating_sub(available_for_data);
    if excess == 0 {
        natural_widths.insert(0, gutter_width);
        return natural_widths;
    }

    let mut adjusted: Vec<usize> = natural_widths
        .iter()
        .map(|&w| {
            if w > MIN_COL_WIDTH {
                w.saturating_sub((excess * w) / total_natural.max(1))
                    .max(MIN_COL_WIDTH)
            } else {
                w
            }
        })
        .collect();

    let fitted: usize = adjusted.iter().sum();
    let remaining = available_for_data.saturating_sub(fitted);
    if remaining > 0 {
        for w in &mut adjusted {
            let add = (remaining * *w) / fitted.max(1);
            *w = (*w + add).min(MAX_COL_WIDTH);
        }
    }

    adjusted.insert(0, gutter_width);
    adjusted
}

fn format_value(val: &Option<String>, width: usize, numeric: bool) -> Span<'static> {
    let raw = val.as_deref().unwrap_or("NULL");
    let s = truncate(raw, width);
    if val.is_none() {
        Span::styled(
            if numeric {
                format!("{:>width$}", s, width = width)
            } else {
                format!(" {:<width$}", s, width = width.saturating_sub(1))
            },
            Style::new().dim().italic(),
        )
    } else if numeric {
        Span::raw(format!("{:>width$}", s, width = width))
    } else {
        Span::raw(format!(" {:<width$}", s, width = width.saturating_sub(1)))
    }
}

fn format_table_header(
    result: &QueryResult,
    available_width: usize,
    block_focused: bool,
) -> Vec<Line<'static>> {
    let col_widths = compute_col_widths(result, available_width);
    if col_widths.is_empty() {
        return vec![];
    }

    let gutter_width = col_widths[0];
    let data_widths = &col_widths[1..];

    let sep = data_widths
        .iter()
        .map(|w| "─".repeat(w + 2))
        .collect::<Vec<_>>()
        .join("┼");

    let header_text = data_widths
        .iter()
        .enumerate()
        .map(|(i, &w)| {
            let c = truncate(&result.columns[i], MAX_COL_WIDTH);
            format!(" {:width$} ", c, width = w)
        })
        .collect::<Vec<_>>()
        .join("│");

    let header_style = if block_focused {
        Style::new().bold().cyan()
    } else {
        Style::new().bold().dark_gray()
    };

    vec![
        Line::from(
            std::iter::once(format!("{:width$}", "", width = gutter_width))
                .chain(std::iter::once(header_text))
                .collect::<String>(),
        )
        .style(header_style),
        Line::from(format!("{} {}", " ".repeat(gutter_width), sep)).dim(),
    ]
}

fn format_result_table(
    result: &QueryResult,
    available_width: usize,
    _scroll_x: usize,
    _viewport_height: usize,
    block_focused: bool,
) -> Vec<Line<'static>> {
    let mut lines: Vec<Line<'static>> = Vec::new();

    let col_count = result.columns.len();
    if col_count == 0 {
        let summary = format!("Query OK, {} row(s) affected", result.rows_affected);
        lines.push(Line::from(summary).green());
        lines.push(Line::from(
            format!("({} ms)", result.execution_time_ms).dim(),
        ));
        return lines;
    }

    let col_widths = compute_col_widths(result, available_width);
    if col_widths.is_empty() {
        return lines;
    }

    let numeric_cols = detect_numeric_cols(result);
    let gutter_width = col_widths[0];
    let data_widths = &col_widths[1..];

    let sep = data_widths
        .iter()
        .map(|w| "─".repeat(w + 2))
        .collect::<Vec<_>>()
        .join("┼");

    let header_text = data_widths
        .iter()
        .enumerate()
        .map(|(i, &w)| {
            let c = truncate(&result.columns[i], MAX_COL_WIDTH);
            format!(" {:width$} ", c, width = w)
        })
        .collect::<Vec<_>>()
        .join("│");

    let header_style = if block_focused {
        Style::new().bold().cyan()
    } else {
        Style::new().bold().dark_gray()
    };

    lines.push(Line::from(format!("{:width$}", "", width = gutter_width)));
    lines.push(Line::from(
        std::iter::once(format!("{:width$}", "", width = gutter_width))
            .chain(std::iter::once(header_text))
            .collect::<String>(),
    )
    .style(header_style));
    lines.push(Line::from(format!("{:width$}", "", width = gutter_width)).dim());

    lines.push(Line::from(format!(
        "{} {}",
        " ".repeat(gutter_width),
        sep
    ))
    .dim());

    for (row_idx, row) in result.rows.iter().enumerate() {
        let row_bg = if row_idx % 2 == 1 {
            Style::new().on_dark_gray()
        } else {
            Style::new()
        };

        let row_num = format!("{:>width$})", row_idx + 1, width = gutter_width.saturating_sub(1));
        let mut spans = vec![Span::styled(row_num, Style::new().dim())];

        for (i, val) in row.iter().enumerate() {
            if i > 0 {
                spans.push(Span::raw("│"));
            }
            let styled_val = format_value(val, data_widths[i], numeric_cols[i]);
            let styled_val = if row_bg != Style::new() {
                Span::styled(styled_val.content.to_string(), row_bg)
            } else {
                styled_val
            };
            spans.push(styled_val);
        }

        let line = Line::from(spans).style(row_bg);
        lines.push(line);
    }

    lines.push(Line::from(format!(
        "{} {}",
        " ".repeat(gutter_width),
        sep
    ))
    .dim());

    let summary = format!(
        "{} row(s) in set ({} ms)",
        result.rows_affected, result.execution_time_ms
    );
    lines.push(Line::from(
        std::iter::once(format!("{:width$}", "", width = gutter_width))
            .chain(std::iter::once(summary))
            .collect::<String>(),
    )
    .green());

    lines
}

fn format_result_vertical(result: &QueryResult) -> Vec<Line<'static>> {
    let mut lines: Vec<Line<'static>> = Vec::new();

    if result.columns.is_empty() {
        let summary = format!("Query OK, {} row(s) affected", result.rows_affected);
        lines.push(Line::from(summary).green());
        lines.push(Line::from(
            format!("({} ms)", result.execution_time_ms).dim(),
        ));
        return lines;
    }

    for (row_idx, row) in result.rows.iter().enumerate() {
        if row_idx > 0 {
            lines.push(Line::from(format!("─── row {} ───", row_idx + 1)).dim());
        }

        let max_col_len = result
            .columns
            .iter()
            .map(|c| c.len())
            .max()
            .unwrap_or(0);

        for (i, col) in result.columns.iter().enumerate() {
            let raw: String = row.get(i).and_then(|v| v.as_deref()).unwrap_or("NULL").to_string();
            let val_style = if row.get(i).and_then(|v| v.as_ref()).is_none() {
                Style::new().dim().italic()
            } else {
                Style::new()
            };

            lines.push(Line::from(vec![
                Span::styled(
                    format!("{:width$}", col, width = max_col_len),
                    Style::new().bold().cyan(),
                ),
                Span::raw(" : "),
                Span::styled(raw, val_style),
            ]));
        }
    }

    let summary = format!(
        "{} row(s) in set ({} ms)",
        result.rows_affected, result.execution_time_ms
    );
    lines.push(Line::from(summary).green());

    lines
}

pub fn format_block_as_text(block: &QueryBlock) -> String {
    let mut out = String::new();
    out.push_str(&format!("mysql> {}\n", block.sql));
    if let Some(result) = &block.result {
        let lines = match block.view_mode {
            ViewMode::Table => format_result_table(result, 9999, 0, 0, false),
            ViewMode::Vertical => format_result_vertical(result),
        };
        for line in lines {
            let text: String = line.spans.iter().map(|s| s.content.as_ref()).collect();
            out.push_str(&text);
            out.push('\n');
        }
    }
    if let Some(err) = &block.error {
        out.push_str(&format!("ERROR: {err}\n"));
    }
    out.push('\n');
    out
}

fn render_query_block(
    block: &QueryBlock,
    available_width: usize,
    scroll_x: usize,
    viewport_height: usize,
    focused: bool,
) -> (Vec<Line<'static>>, Option<Vec<Line<'static>>>) {
    let mut lines: Vec<Line<'static>> = Vec::new();
    let mut sticky_header: Option<Vec<Line<'static>>> = None;

    let prefix = if focused { "▶ " } else { "  " };
    let sql_line = format!("{}mysql> {}", prefix, block.sql);
    if focused {
        lines.push(Line::from(sql_line).bold().cyan());
    } else {
        lines.push(Line::from(sql_line).bold());
    }

    if let Some(result) = &block.result {
        let (result_lines, header) = match block.view_mode {
            ViewMode::Table => {
                let has_cols = !result.columns.is_empty();
                let header = if focused && has_cols {
                    Some(format_table_header(result, available_width, true))
                } else {
                    None
                };
                let body = format_result_table(result, available_width, scroll_x, viewport_height, focused);
                let body = if focused && has_cols {
                    body.into_iter().skip(4).collect()
                } else {
                    body
                };
                (body, header)
            }
            ViewMode::Vertical => (format_result_vertical(result), None),
        };
        if focused {
            sticky_header = header;
        }
        lines.extend(result_lines);
    }
    if let Some(err) = &block.error {
        lines.push(Line::from(format!("ERROR: {}", err)).red());
    }
    lines.push(Line::from(""));

    (lines, sticky_header)
}

pub fn render(frame: &mut Frame, app: &App) {
    let area = frame.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(area);

    render_header(frame, chunks[0], app);
    render_results(frame, chunks[1], app);
    render_input(frame, chunks[2], app);
    render_status(frame, chunks[3], app);

    if app.completion_active {
        render_completion_popup(frame, chunks[2], app);
    }
}

fn render_header(frame: &mut Frame, area: Rect, app: &App) {
    let text = format!(" kon · {} ", app.conn_name);
    let para = Paragraph::new(Text::from(text))
        .style(Style::new().on_blue().white().bold())
        .left_aligned();
    frame.render_widget(para, area);
}

fn render_results(frame: &mut Frame, area: Rect, app: &App) {
    if app.query_blocks.is_empty() {
        let welcome = Text::from(Line::from(
            "Welcome to kon! Type a SQL query and press Enter to execute.",
        ));
        let para = Paragraph::new(welcome)
            .style(Style::new().dim())
            .left_aligned();
        frame.render_widget(para, area);
        return;
    }

    let mut lines: Vec<Line<'static>> = Vec::new();
    let mut sticky_header: Option<Vec<Line<'static>>> = None;

    for (idx, block) in app.query_blocks.iter().enumerate() {
        let focused = idx == app.active_block;
        let (block_lines, block_header) = render_query_block(
            block,
            area.width as usize,
            app.scroll_x,
            area.height as usize,
            focused,
        );
        if focused {
            sticky_header = block_header;
        }
        lines.extend(block_lines);
    }

    let header_height = sticky_header.as_ref().map(|h| h.len() as u16).unwrap_or(0);
    let [header_area, content_area] =
        Layout::vertical([Constraint::Length(header_height), Constraint::Min(0)]).areas(area);

    if let Some(header) = sticky_header {
        frame.render_widget(Paragraph::new(Text::from(header)), header_area);
    }

    let total = lines.len();
    let viewport = content_area.height as usize;

    let scroll_offset = total.saturating_sub(viewport).saturating_sub(app.scroll);

    let text = Text::from(lines);
    let para = Paragraph::new(text)
        .scroll((scroll_offset as u16, 0))
        .left_aligned();
    frame.render_widget(para, content_area);
}

fn render_input(frame: &mut Frame, area: Rect, app: &App) {
    let prefix = " sql> ";
    let prefix_width = prefix.len();

    let input_line = if let AppState::Executing = app.state {
        Line::from("Executing...")
    } else {
        let tokens = syntax::highlight(&app.input);
        let mut spans: Vec<Span> = Vec::with_capacity(tokens.len() + 1);
        let mut pos = 0;
        for ht in &tokens {
            if ht.start > pos {
                spans.push(Span::raw(&app.input[pos..ht.start]));
            }
            let text = &app.input[ht.start..ht.end.min(app.input.len())];
            spans.push(Span::styled(text.to_string(), ht.style));
            pos = ht.end.min(app.input.len());
        }
        if pos < app.input.len() {
            spans.push(Span::raw(&app.input[pos..]));
        }
        if spans.is_empty() {
            spans.push(Span::raw(&app.input));
        }
        Line::from(spans)
    };

    let full_line = Line::from(
        std::iter::once(Span::raw(prefix))
            .chain(input_line.spans.iter().cloned())
            .collect::<Vec<_>>(),
    );

    let text = Text::from(full_line);
    let para = Paragraph::new(text)
        .style(Style::new().on_dark_gray().white())
        .left_aligned();
    frame.render_widget(para, area);

    let cursor_x = area.x + (prefix_width + app.cursor) as u16;
    let cursor_y = area.y;
    frame.set_cursor_position((cursor_x.min(area.right().saturating_sub(1)), cursor_y));
}

fn render_status(frame: &mut Frame, area: Rect, app: &App) {
    let text = match &app.state {
        AppState::Executing => " Executing query...".to_string(),
        AppState::Idle => {
            let mut parts = vec![];

            parts.push(format!("{} queries", app.query_blocks.len()));

            if !app.query_blocks.is_empty() {
                parts.push(format!(
                    "block {}/{}",
                    app.active_block + 1,
                    app.query_blocks.len()
                ));
                if app.block_view_mode(app.active_block).is_vertical() {
                    parts.push("vertical".to_string());
                }
            }

            if app.completion_active {
                let idx = app.completion.selection() + 1;
                let total = app.completion.candidate_count();
                if let Some(cand) = app.completion.current_candidate() {
                    let kind_icon = match cand.kind {
                        "keyword" => "K",
                        "table" => "T",
                        "column" => "C",
                        "function" => "F",
                        _ => "?",
                    };
                    parts.push(format!(
                        "Tab: {} [{:>2}/{:<2}]",
                        cand.display,
                        format!("{}{}", kind_icon, idx),
                        total
                    ));
                }
            }

            if app.scroll_x > 0 {
                parts.push(format!("H-scroll: {}", app.scroll_x));
            }

            parts.push("Ctrl+O:Editor".to_string());
            parts.push("Ctrl+R:Refresh".to_string());
            parts.push("Ctrl+V:View".to_string());
            parts.push("Ctrl+D:Quit".to_string());
            parts.push("Tab:Complete".to_string());
            parts.push("PgUp/PgDn:Scroll".to_string());
            parts.reverse();
            parts.join(" | ")
        }
    };
    let para = Paragraph::new(Text::from(text))
        .style(Style::new().on_black().white())
        .left_aligned();
    frame.render_widget(para, area);
}

fn render_completion_popup(frame: &mut Frame, input_area: Rect, app: &App) {
    let candidates = app.completion.candidates();
    if candidates.is_empty() {
        return;
    }

    let max_visible = 8.min(candidates.len());
    let popup_height = max_visible as u16 + 2;
    let popup_width = 48;

    let popup_x = input_area.x + 6;
    let popup_y = input_area.y.saturating_sub(popup_height);

    let available_width = input_area.width.saturating_sub(6);
    let actual_width = popup_width.min(available_width);

    let popup_area = Rect::new(popup_x, popup_y, actual_width, popup_height);

    let items: Vec<ListItem> = candidates
        .iter()
        .enumerate()
        .map(|(i, cand)| {
            let prefix = match cand.kind {
                "keyword" => " K ",
                "table" => " T ",
                "column" => " C ",
                "function" => " F ",
                _ => " ? ",
            };
            let prefix_style = match cand.kind {
                "keyword" => Style::new().fg(ratatui::style::Color::Cyan),
                "table" => Style::new().fg(ratatui::style::Color::Yellow),
                "column" => Style::new().fg(ratatui::style::Color::Magenta),
                "function" => Style::new().fg(ratatui::style::Color::Green),
                _ => Style::new(),
            };

            let suffix = cand
                .table
                .as_ref()
                .map(|t| format!(" ({})", t))
                .unwrap_or_default();

            let selected = i == app.completion.selection();
            let item_style = if selected {
                Style::new().on_blue().white()
            } else {
                Style::new()
            };

            let (display, _) = if selected && cand.display.len() + suffix.len() + 3 > actual_width as usize {
                let max_d = actual_width as usize - suffix.len() - 4;
                let d = truncate(&cand.display, max_d);
                (d, true)
            } else {
                (cand.display.clone(), false)
            };

            ListItem::new(Line::from(vec![
                Span::styled(prefix, prefix_style),
                Span::styled(display.clone(), item_style),
                Span::styled(suffix, Style::new().dim()),
            ]))
        })
        .collect();

    let title = format!(
        " Completions ({}/{}) ",
        app.completion.selection() + 1,
        candidates.len()
    );

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
                .border_style(Style::new().cyan()),
        )
        .highlight_style(Style::new().on_blue().white());

    frame.render_widget(Clear, popup_area);
    frame.render_widget(list, popup_area);
}
