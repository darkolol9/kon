use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::style::Stylize;
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use crate::db::QueryResult;
use crate::theme::Theme;

pub fn render_vertical_lines(
    result: &QueryResult,
    theme: &Theme,
    row_offset: usize,
) -> Vec<Line<'static>> {
    let mut lines: Vec<Line<'static>> = Vec::new();

    if result.columns.is_empty() {
        let summary = format!("Query OK, {} row(s) affected", result.rows_affected);
        lines.push(Line::from(summary).style(theme.summary));
        lines.push(Line::from(format!("({} ms)", result.execution_time_ms)).dim());
        return lines;
    }

    let start = row_offset.min(result.rows.len());
    for (rel_idx, row) in result.rows[start..].iter().enumerate() {
        let row_idx = start + rel_idx;
        if row_idx > 0 {
            lines.push(Line::from(format!("─── row {} ───", row_idx + 1)).dim());
        }

        let max_col_len = result.columns.iter().map(|c| c.len()).max().unwrap_or(0);

        for (i, col) in result.columns.iter().enumerate() {
            let raw: String = row
                .get(i)
                .and_then(|v| v.as_deref())
                .unwrap_or("NULL")
                .to_string();
            let val_style = if row.get(i).and_then(|v| v.as_ref()).is_none() {
                theme.null_value
            } else {
                Style::new()
            };

            lines.push(Line::from(vec![
                Span::styled(
                    format!("{:width$}", col, width = max_col_len),
                    theme.vertical_col,
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
    lines.push(Line::from(summary).style(theme.summary));

    lines
}

#[allow(dead_code)]
pub fn render(frame: &mut Frame, area: Rect, result: &QueryResult, scroll: usize, theme: &Theme) {
    let lines = render_vertical_lines(result, theme, scroll);
    let total = lines.len();
    let viewport = area.height as usize;

    let scroll_offset = total.saturating_sub(viewport).saturating_sub(scroll);

    let para = Paragraph::new(ratatui::text::Text::from(lines))
        .scroll((scroll_offset as u16, 0))
        .left_aligned();
    frame.render_widget(para, area);
}

pub fn format_vertical_as_text(result: &QueryResult) -> Vec<Line<'static>> {
    let mut lines: Vec<Line<'static>> = Vec::new();

    if result.columns.is_empty() {
        let summary = format!("Query OK, {} row(s) affected", result.rows_affected);
        lines.push(Line::from(summary));
        lines.push(Line::from(format!("({} ms)", result.execution_time_ms)));
        return lines;
    }

    for (row_idx, row) in result.rows.iter().enumerate() {
        if row_idx > 0 {
            lines.push(Line::from(format!("─── row {} ───", row_idx + 1)));
        }

        let max_col_len = result.columns.iter().map(|c| c.len()).max().unwrap_or(0);

        for (i, col) in result.columns.iter().enumerate() {
            let raw = row.get(i).and_then(|v| v.as_deref()).unwrap_or("NULL");
            lines.push(Line::from(format!(
                "{:width$} : {}",
                col,
                raw,
                width = max_col_len
            )));
        }
    }

    let summary = format!(
        "{} row(s) in set ({} ms)",
        result.rows_affected, result.execution_time_ms
    );
    lines.push(Line::from(summary));

    lines
}
