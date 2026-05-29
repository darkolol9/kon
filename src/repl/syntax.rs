use ratatui::style::{Color, Modifier, Style};
use sqlparser::dialect::MySqlDialect;
use sqlparser::keywords::Keyword;
use sqlparser::tokenizer::{Token, TokenWithSpan, Tokenizer};

pub struct HighlightToken {
    pub start: usize,
    pub end: usize,
    pub style: Style,
}

pub fn highlight(sql: &str) -> Vec<HighlightToken> {
    let dialect = MySqlDialect {};
    let mut tokenizer = Tokenizer::new(&dialect, sql);
    let mut buf: Vec<TokenWithSpan> = Vec::new();

    let _ = tokenizer.tokenize_with_location_into_buf(&mut buf);

    buf.iter()
        .filter_map(|twl| {
            let start = (twl.span.start.column.saturating_sub(1)) as usize;
            let end = (twl.span.end.column.saturating_sub(1)) as usize;
            let style = token_style(&twl.token)?;
            Some(HighlightToken { start, end, style })
        })
        .collect()
}

fn token_style(token: &Token) -> Option<Style> {
    match token {
        Token::Word(w) if w.keyword != Keyword::NoKeyword => {
            Some(Style::new().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        }
        Token::Word(_) => None,
        Token::Number(_, _) => {
            Some(Style::new().fg(Color::Yellow))
        }
        Token::SingleQuotedString(_)
        | Token::DoubleQuotedString(_)
        | Token::TripleSingleQuotedString(_)
        | Token::TripleDoubleQuotedString(_)
        | Token::NationalStringLiteral(_) => {
            Some(Style::new().fg(Color::Green))
        }
        Token::Whitespace(_) => None,
        Token::Comma => None,
        _ => {
            Some(Style::new().fg(Color::White).add_modifier(Modifier::DIM))
        }
    }
}
