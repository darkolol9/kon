use ratatui::style::Style;
use sqlparser::dialect::MySqlDialect;
use sqlparser::keywords::Keyword;
use sqlparser::tokenizer::{Token, TokenWithSpan, Tokenizer};

use crate::theme::Theme;

pub struct HighlightToken {
    pub start: usize,
    pub end: usize,
    pub style: Style,
}

pub fn highlight(sql: &str, theme: &Theme) -> Vec<HighlightToken> {
    let dialect = MySqlDialect {};
    let mut tokenizer = Tokenizer::new(&dialect, sql);
    let mut buf: Vec<TokenWithSpan> = Vec::new();

    let _ = tokenizer.tokenize_with_location_into_buf(&mut buf);

    buf.iter()
        .filter_map(|twl| {
            let start = (twl.span.start.column.saturating_sub(1)) as usize;
            let end = (twl.span.end.column.saturating_sub(1)) as usize;
            let style = token_style(&twl.token, theme)?;
            Some(HighlightToken { start, end, style })
        })
        .collect()
}

fn token_style(token: &Token, theme: &Theme) -> Option<Style> {
    match token {
        Token::Word(w) if w.keyword != Keyword::NoKeyword => Some(theme.syntax_keyword),
        Token::Word(_) => None,
        Token::Number(_, _) => Some(theme.syntax_number),
        Token::SingleQuotedString(_)
        | Token::DoubleQuotedString(_)
        | Token::TripleSingleQuotedString(_)
        | Token::TripleDoubleQuotedString(_)
        | Token::NationalStringLiteral(_) => Some(theme.syntax_string),
        Token::Whitespace(_) => None,
        Token::Comma => None,
        _ => Some(theme.syntax_operator),
    }
}
