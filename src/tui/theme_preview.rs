use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Clear, Paragraph};

use crate::theme::Theme;

pub fn render_preview(frame: &mut Frame, area: Rect, theme: &Theme) {
    if area.width < 36 || area.height < 10 {
        return;
    }

    frame.render_widget(Clear, area);
    frame.render_widget(Paragraph::new("").style(Style::new().bg(theme.bg)), area);

    let top_bar_area = Rect::new(area.x, area.y, area.width, 1);
    render_top_bar(frame, top_bar_area, theme);

    let bottom_bar_area = Rect::new(area.x, area.y + area.height - 1, area.width, 1);
    render_preview_bottom_bar(frame, bottom_bar_area, theme);

    let content_y = area.y + 1;
    let content_h = area.height.saturating_sub(2);
    if content_h < 3 {
        return;
    }

    let sidebar_w = 14u16.min(area.width.saturating_sub(4) / 3).max(8);
    let main_w = area.width.saturating_sub(sidebar_w);

    render_sidebar(
        frame,
        Rect::new(area.x, content_y, sidebar_w, content_h),
        theme,
    );
    render_main_content(
        frame,
        Rect::new(area.x + sidebar_w, content_y, main_w, content_h),
        theme,
    );
}

fn render_top_bar(frame: &mut Frame, area: Rect, theme: &Theme) {
    let left = vec![
        Span::styled(
            " kon ",
            Style::new()
                .fg(theme.top_bar_active.fg.unwrap_or(theme.bg))
                .bg(theme.top_bar_active.bg.unwrap_or(theme.top_bar_bg))
                .bold(),
        ),
        Span::raw(" "),
        Span::styled(" Editor ", theme.top_bar_active),
        Span::raw(" "),
        Span::styled(" Connections ", theme.top_bar_inactive),
        Span::raw(" "),
        Span::styled(" Settings ", theme.top_bar_inactive),
    ];

    let left_len: u16 = left.iter().map(|s| s.content.len() as u16).sum();
    let right = " ● mydb";
    let pad = area.width.saturating_sub(left_len + right.len() as u16 + 1);

    let mut spans = left;
    spans.push(Span::raw(" ".repeat(pad as usize)));
    spans.push(Span::styled(
        " ● ",
        Style::new().fg(theme.summary.fg.unwrap_or(theme.bottom_bar_fg)),
    ));
    spans.push(Span::styled("mydb", Style::new().fg(theme.input_fg).dim()));

    frame.render_widget(
        Paragraph::new(Line::from(spans)).style(Style::new().bg(theme.top_bar_bg)),
        area,
    );
}

fn render_preview_bottom_bar(frame: &mut Frame, area: Rect, theme: &Theme) {
    let text = Line::from(Span::styled(
        " ↑↓ Nav  ↵ Apply  │  ⎋ Back ",
        Style::new().fg(theme.bottom_bar_fg),
    ));
    frame.render_widget(
        Paragraph::new(text).style(Style::new().bg(theme.bottom_bar_bg)),
        area,
    );
}

fn render_sidebar(frame: &mut Frame, area: Rect, theme: &Theme) {
    let block = Block::bordered()
        .title(" Databases ")
        .border_style(theme.border_primary);
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let entries = [("mysql", true), ("myapp", false), ("test", false)];
    for (offset, (name, active)) in entries.iter().enumerate() {
        let y = inner.y + offset as u16;
        if y >= inner.y + inner.height {
            break;
        }
        let prefix = if *active { " ◉ " } else { "   " };
        let style = if *active {
            Style::new().fg(theme.completion_kw).bold()
        } else {
            Style::new().fg(theme.input_fg)
        };
        frame.render_widget(
            Paragraph::new(Line::from(Span::styled(
                format!("{}{}", prefix, name),
                style,
            ))),
            Rect::new(inner.x, y, inner.width, 1),
        );
    }
}

fn render_main_content(frame: &mut Frame, area: Rect, theme: &Theme) {
    if area.height < 4 {
        return;
    }

    let header_y = area.y;
    let sep_y = area.y + area.height - 2;
    let input_y = area.y + area.height - 1;
    let results_area = Rect::new(
        area.x,
        area.y + 1,
        area.width,
        area.height.saturating_sub(3),
    );

    // Block header
    let header = Line::from(vec![
        Span::styled(" [1] ", Style::new().fg(theme.bg).bg(theme.completion_kw)),
        Span::raw(" "),
        Span::styled("SELECT * FROM users", theme.sql_focused),
    ]);
    frame.render_widget(
        Paragraph::new(header),
        Rect::new(area.x, header_y, area.width, 1),
    );

    // Results table
    render_results_table(frame, results_area, theme);

    // Separator
    let sep = Line::from(Span::styled(
        "─".repeat(area.width as usize),
        theme.border_secondary,
    ));
    frame.render_widget(
        Paragraph::new(sep).style(Style::new().bg(theme.bg)),
        Rect::new(area.x, sep_y, area.width, 1),
    );

    // Input line with syntax highlighting
    let input = Line::from(vec![
        Span::styled(" mydb> ", Style::new().fg(theme.completion_kw)),
        Span::styled("SELECT", theme.syntax_keyword),
        Span::raw(" * FROM "),
        Span::styled("users", Style::new().fg(theme.input_fg)),
        Span::raw(" LIMIT "),
        Span::styled("10", theme.syntax_number),
        Span::styled(";", theme.syntax_operator),
    ]);
    frame.render_widget(
        Paragraph::new(input).style(Style::new().bg(theme.input_bg).fg(theme.input_fg)),
        Rect::new(area.x, input_y, area.width, 1),
    );
}

fn render_results_table(frame: &mut Frame, area: Rect, theme: &Theme) {
    let cols = ["id", "name", "email"];
    let rows = [
        ["1", "Alice", "alice@ex…"],
        ["2", "Bob", "bob@test"],
        ["3", "Carol", "carol@fo…"],
    ];

    let widths: Vec<usize> = cols
        .iter()
        .enumerate()
        .map(|(i, c)| {
            let max_data = rows.iter().map(|r| r[i].len()).max().unwrap_or(0);
            c.len().max(max_data) + 2
        })
        .collect();

    let total: usize = widths.iter().sum::<usize>() + (cols.len() - 1) * 2;
    let scale = if total > area.width as usize {
        (area.width as f64 - (cols.len() - 1) as f64 * 2.0)
            / (total as f64 - (cols.len() - 1) as f64 * 2.0)
    } else {
        1.0
    };

    let w = |i: usize| -> usize {
        let n = widths[i];
        if scale >= 1.0 {
            n
        } else {
            (n as f64 * scale).max(4.0).ceil() as usize
        }
    };

    let fmt_cell = |text: &str, ci: usize| -> String {
        let max_w = w(ci);
        let inner = max_w.saturating_sub(2);
        if text.len() > inner {
            format!(" {:.width$}", &text[..inner], width = inner)
        } else {
            format!(" {:width$}", text, width = inner)
        }
    };

    let mut y = area.y;
    let bottom = area.y + area.height;

    // Header
    if y < bottom {
        let mut spans: Vec<Span> = Vec::new();
        for (i, col) in cols.iter().enumerate() {
            if i > 0 {
                spans.push(Span::raw("  "));
            }
            spans.push(Span::styled(fmt_cell(col, i), theme.table_header_focused));
        }
        frame.render_widget(
            Paragraph::new(Line::from(spans)),
            Rect::new(area.x, y, area.width, 1),
        );
        y += 1;
    }

    // Rows
    for (ri, row) in rows.iter().enumerate() {
        if y >= bottom {
            break;
        }
        let row_style = if ri % 2 == 1 {
            Style::new().bg(theme.table_row_alt_bg)
        } else {
            Style::new()
        };
        let mut spans: Vec<Span> = Vec::new();
        for (i, val) in row.iter().enumerate() {
            if i > 0 {
                spans.push(Span::raw("  "));
            }
            spans.push(Span::raw(fmt_cell(val, i)));
        }
        frame.render_widget(
            Paragraph::new(Line::from(spans)).style(row_style),
            Rect::new(area.x, y, area.width, 1),
        );
        y += 1;
    }

    // Summary
    if y < bottom {
        frame.render_widget(
            Paragraph::new(Line::from(Span::styled(
                " 3 rows in set (3 ms)",
                theme.summary,
            ))),
            Rect::new(area.x, y, area.width, 1),
        );
    }
}
