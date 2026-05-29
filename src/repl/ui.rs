use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Style, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use crate::db::QueryResult;
use crate::repl::syntax;
use crate::repl::{App, AppState, QueryBlock};

fn format_result(result: &QueryResult) -> Vec<Line<'static>> {
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

    let col_widths: Vec<usize> = result
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
            std::cmp::max(max_data, col.len())
        })
        .collect();

    let sep = col_widths
        .iter()
        .map(|w| "─".repeat(w + 2))
        .collect::<Vec<_>>()
        .join("┼");

    let header = result
        .columns
        .iter()
        .enumerate()
        .map(|(i, col)| format!(" {:width$} ", col, width = col_widths[i]))
        .collect::<Vec<_>>()
        .join("│");

    lines.push(Line::from(""));
    lines.push(Line::from(header).bold().cyan());
    lines.push(Line::from(format!("─{}─", sep)).dim());

    for row in &result.rows {
        let row_str = row
            .iter()
            .enumerate()
            .map(|(i, val)| {
                let s = val.as_deref().unwrap_or("NULL");
                if val.is_none() {
                    Span::styled(
                        format!(" {:width$} ", s, width = col_widths[i]),
                        Style::new().dim().italic(),
                    )
                } else {
                    Span::raw(format!(" {:width$} ", s, width = col_widths[i]))
                }
            })
            .collect::<Vec<_>>();

        let mut spans = Vec::new();
        for (i, span) in row_str.iter().enumerate() {
            if i > 0 {
                spans.push(Span::raw("│"));
            }
            spans.push(span.clone());
        }
        lines.push(Line::from(spans));
    }

    lines.push(Line::from(format!("─{}─", sep)).dim());
    let summary = format!(
        "{} row(s) in set ({} ms)",
        result.rows_affected, result.execution_time_ms
    );
    lines.push(Line::from(summary).green());

    lines
}

fn render_query_block(block: &QueryBlock) -> Vec<Line<'static>> {
    let mut lines: Vec<Line<'static>> = Vec::new();

    let sql_line = format!("mysql> {}", block.sql);
    lines.push(Line::from(sql_line).bold());

    if let Some(result) = &block.result {
        lines.extend(format_result(result));
    }
    if let Some(err) = &block.error {
        lines.push(Line::from(format!("ERROR: {}", err)).red());
    }
    lines.push(Line::from(""));

    lines
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
    for block in &app.query_blocks {
        lines.extend(render_query_block(block));
    }

    let total = lines.len();
    let viewport = area.height as usize;

    let scroll_offset = total.saturating_sub(viewport).saturating_sub(app.scroll);

    let text = Text::from(lines);
    let para = Paragraph::new(text)
        .scroll((scroll_offset as u16, 0))
        .left_aligned();
    frame.render_widget(para, area);
}

fn render_input(frame: &mut Frame, area: Rect, app: &App) {
    let prefix = " sql> ";
    let prefix_width = prefix.len();

    let input_line = if app.state.is_executing() {
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
    frame.set_cursor_position((
        cursor_x.min(area.right().saturating_sub(1)),
        cursor_y,
    ));
}

fn render_status(frame: &mut Frame, area: Rect, app: &App) {
    let text = match &app.state {
        AppState::Executing => " Executing query...".to_string(),
        AppState::Idle => {
            let mut parts = vec![format!(" {} queries", app.query_blocks.len())];
            if app.completion.has_completions() {
                let idx = app.completion.candidate_index() + 1;
                let total = app.completion.candidate_count();
                if let Some(cand) = app.completion.current_candidate() {
                    parts.push(format!(" Tab: {} ({}/{})", cand, idx, total));
                }
            }
            parts.push("Ctrl+D:Quit".to_string());
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

impl AppState {
    fn is_executing(&self) -> bool {
        matches!(self, AppState::Executing)
    }
}
