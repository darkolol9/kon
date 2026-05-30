use std::ops::Range;

use ratatui::Frame;
use ratatui::layout::{Constraint, Rect};
use ratatui::style::{Style, Stylize};
use ratatui::text::Line;
use ratatui::widgets::{Cell, Paragraph, Row, Table};

use crate::db::QueryResult;
use crate::theme::Theme;

const MAX_COL_WIDTH: u16 = 40;
const COL_SPACING: u16 = 2;

pub fn render_table(
    frame: &mut Frame,
    area: Rect,
    result: &QueryResult,
    theme: &Theme,
    focused: bool,
    scroll_x: u16,
) {
    if result.columns.is_empty() {
        let summary = format!("Query OK, {} row(s) affected", result.rows_affected);
        let lines = [
            Line::from(summary).style(theme.summary),
            Line::from(format!("({} ms)", result.execution_time_ms)).dim(),
        ];
        frame.render_widget(Paragraph::new(lines.to_vec()), area);
        return;
    }

    let col_count = result.columns.len();
    if col_count == 0 || area.width < 1 {
        return;
    }

    let widths = compute_natural_widths(result);
    let total_gaps = col_count.saturating_sub(1) as u16 * COL_SPACING;
    let total_natural: u16 = widths.iter().sum::<u16>() + total_gaps;

    let col_range = if total_natural <= area.width {
        0..col_count
    } else {
        let max_scroll = total_natural.saturating_sub(area.width);
        let scroll_x = scroll_x.min(max_scroll);
        find_visible_range(&widths, scroll_x, area.width)
    };

    let constraints: Vec<Constraint> = widths[col_range.clone()]
        .iter()
        .map(|w| Constraint::Max(*w))
        .collect();

    let header_style = if focused {
        theme.table_header_focused
    } else {
        theme.table_header_unfocused
    };

    let header = Row::new(
        result.columns[col_range.clone()]
            .iter()
            .map(|c| Cell::from(c.as_str()))
            .collect::<Vec<_>>(),
    )
    .style(header_style);

    let rows: Vec<Row> = result
        .rows
        .iter()
        .enumerate()
        .map(|(i, row)| {
            let row_style = if i % 2 == 1 {
                Style::new().bg(theme.table_row_alt_bg)
            } else {
                Style::new()
            };
            let cells: Vec<Cell> = row[col_range.clone()]
                .iter()
                .map(|val| match val {
                    None => Cell::from("NULL").style(theme.null_value),
                    Some(s) => Cell::from(s.as_str()),
                })
                .collect();
            Row::new(cells).style(row_style)
        })
        .collect();

    let table = Table::new(rows, constraints)
        .header(header)
        .column_spacing(COL_SPACING);
    frame.render_widget(table, area);
}

fn compute_natural_widths(result: &QueryResult) -> Vec<u16> {
    result
        .columns
        .iter()
        .enumerate()
        .map(|(i, col)| {
            let max_data = result
                .rows
                .iter()
                .filter_map(|row| {
                    row.get(i)
                        .and_then(|v| v.as_deref().map(|s| s.len() as u16))
                })
                .max()
                .unwrap_or(0);
            (col.len() as u16).max(max_data).clamp(8, MAX_COL_WIDTH)
        })
        .collect()
}

fn find_visible_range(widths: &[u16], scroll_x: u16, visible_width: u16) -> Range<usize> {
    let right_edge = scroll_x.saturating_add(visible_width);
    let mut cum = 0u16;
    let mut start = None;
    let mut end = 0;

    for (i, w) in widths.iter().enumerate() {
        let col_end = cum.saturating_add(*w);

        if col_end > scroll_x && start.is_none() {
            start = Some(i);
        }

        if cum < right_edge {
            end = i + 1;
        }

        cum = col_end.saturating_add(COL_SPACING);
    }

    start.unwrap_or(0)..end
}

pub fn format_result_as_text(result: &QueryResult) -> Vec<Line<'static>> {
    let mut lines: Vec<Line<'static>> = Vec::new();

    if result.columns.is_empty() {
        let summary = format!("Query OK, {} row(s) affected", result.rows_affected);
        lines.push(Line::from(summary));
        lines.push(Line::from(format!("({} ms)", result.execution_time_ms)));
        return lines;
    }

    let mut header = String::new();
    for (i, col) in result.columns.iter().enumerate() {
        if i > 0 {
            header.push_str(" | ");
        }
        header.push_str(col);
    }
    lines.push(Line::from(header));

    for row in &result.rows {
        let mut row_str = String::new();
        for (i, val) in row.iter().enumerate() {
            if i > 0 {
                row_str.push_str(" | ");
            }
            row_str.push_str(val.as_deref().unwrap_or("NULL"));
        }
        lines.push(Line::from(row_str));
    }

    let summary = format!(
        "{} row(s) in set ({} ms)",
        result.rows_affected, result.execution_time_ms
    );
    lines.push(Line::from(summary));

    lines
}
