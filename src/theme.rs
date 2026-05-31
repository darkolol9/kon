use ratatui::style::{Color, Modifier, Style};
use std::sync::LazyLock;

#[allow(dead_code)]
pub struct Theme {
    pub name: &'static str,
    pub header_fg: Color,
    pub header_bg: Color,
    pub input_fg: Color,
    pub input_bg: Color,
    pub status_fg: Color,
    pub status_bg: Color,
    pub table_header_focused: Style,
    pub table_header_unfocused: Style,
    pub table_row_alt_bg: Color,
    pub table_row_num: Style,
    pub null_value: Style,
    pub sql_focused: Style,
    pub sql_unfocused: Style,
    pub error: Style,
    pub summary: Style,
    pub vertical_col: Style,
    pub completion_kw: Color,
    pub completion_table: Color,
    pub completion_column: Color,
    pub completion_fn: Color,
    pub completion_command: Color,
    pub completion_selected: Style,
    pub completion_border: Style,
    pub syntax_keyword: Style,
    pub syntax_number: Style,
    pub syntax_string: Style,
    pub syntax_operator: Style,
    pub picker_selected: Style,
    // new fields
    pub header_style: Style,
    pub schema_browser_border: Style,
    pub tab_active: Style,
    pub tab_inactive: Style,
    pub tab_border: Style,
    pub scrollbar_thumb: Style,
    pub scrollbar_track: Style,
    pub border_primary: Style,
    pub border_secondary: Style,
    pub command_palette_selected: Style,
    pub help_key: Style,
    pub help_desc: Style,
    pub help_section: Style,

    // app background
    pub bg: Color,

    // top / bottom bars
    pub top_bar_bg: Color,
    pub top_bar_active: Style,
    pub top_bar_inactive: Style,
    pub bottom_bar_bg: Color,
    pub bottom_bar_fg: Color,
}

pub static ALL_THEMES: LazyLock<Vec<&'static Theme>> = LazyLock::new(|| {
    vec![
        &DEFAULT,
        &DRACULA,
        &NORD,
        &MONOKAI,
        &LIGHT,
        &TOKYO_NIGHT,
        &CATPPUCCIN,
    ]
});

pub fn theme_names() -> Vec<&'static str> {
    ALL_THEMES.iter().map(|t| t.name).collect()
}

pub fn from_name(name: &str) -> Option<&'static Theme> {
    ALL_THEMES.iter().copied().find(|t| t.name == name)
}

pub static DEFAULT: LazyLock<Theme> = LazyLock::new(|| Theme {
    name: "default",
    header_fg: Color::White,
    header_bg: Color::Blue,
    input_fg: Color::White,
    input_bg: Color::DarkGray,
    status_fg: Color::White,
    status_bg: Color::Black,
    table_header_focused: Style::new().fg(Color::Cyan).add_modifier(Modifier::BOLD),
    table_header_unfocused: Style::new()
        .fg(Color::DarkGray)
        .add_modifier(Modifier::BOLD),
    table_row_alt_bg: Color::DarkGray,
    table_row_num: Style::new().add_modifier(Modifier::DIM),
    null_value: Style::new().add_modifier(Modifier::DIM | Modifier::ITALIC),
    sql_focused: Style::new().fg(Color::Cyan).add_modifier(Modifier::BOLD),
    sql_unfocused: Style::new().add_modifier(Modifier::BOLD),
    error: Style::new().fg(Color::Red),
    summary: Style::new().fg(Color::Green),
    vertical_col: Style::new().fg(Color::Cyan).add_modifier(Modifier::BOLD),
    completion_kw: Color::Cyan,
    completion_table: Color::Yellow,
    completion_column: Color::Magenta,
    completion_fn: Color::Green,
    completion_command: Color::White,
    completion_selected: Style::new().bg(Color::Blue).fg(Color::White),
    completion_border: Style::new().fg(Color::Cyan),
    syntax_keyword: Style::new().fg(Color::Cyan).add_modifier(Modifier::BOLD),
    syntax_number: Style::new().fg(Color::Yellow),
    syntax_string: Style::new().fg(Color::Green),
    syntax_operator: Style::new().fg(Color::White).add_modifier(Modifier::DIM),
    picker_selected: Style::new().fg(Color::Cyan).add_modifier(Modifier::BOLD),
    header_style: Style::new()
        .bg(Color::Blue)
        .fg(Color::White)
        .add_modifier(Modifier::BOLD),
    schema_browser_border: Style::new().fg(Color::Cyan),
    tab_active: Style::new().fg(Color::Cyan).add_modifier(Modifier::BOLD),
    tab_inactive: Style::new()
        .fg(Color::DarkGray)
        .add_modifier(Modifier::BOLD),
    tab_border: Style::new().fg(Color::DarkGray),
    scrollbar_thumb: Style::new().fg(Color::White),
    scrollbar_track: Style::new().dim(),
    border_primary: Style::new().fg(Color::Cyan),
    border_secondary: Style::new().fg(Color::DarkGray),
    command_palette_selected: Style::new().bg(Color::Blue).fg(Color::White),
    help_key: Style::new().fg(Color::Cyan).add_modifier(Modifier::BOLD),
    help_desc: Style::new(),
    help_section: Style::new().fg(Color::Cyan).add_modifier(Modifier::BOLD),

    top_bar_bg: Color::Blue,
    top_bar_active: Style::new()
        .fg(Color::White)
        .bg(Color::Blue)
        .add_modifier(Modifier::BOLD),
    top_bar_inactive: Style::new().fg(Color::White).bg(Color::Blue),
    bg: Color::Black,
    bottom_bar_bg: Color::Black,
    bottom_bar_fg: Color::White,
});

pub static DRACULA: LazyLock<Theme> = LazyLock::new(|| Theme {
    name: "dracula",
    header_fg: Color::Rgb(248, 248, 242),
    header_bg: Color::Rgb(68, 71, 90),
    input_fg: Color::Rgb(248, 248, 242),
    input_bg: Color::Rgb(40, 42, 54),
    status_fg: Color::Rgb(248, 248, 242),
    status_bg: Color::Rgb(33, 34, 44),
    table_header_focused: Style::new()
        .fg(Color::Rgb(139, 233, 253))
        .add_modifier(Modifier::BOLD),
    table_header_unfocused: Style::new().fg(Color::Rgb(98, 114, 164)),
    table_row_alt_bg: Color::Rgb(68, 71, 90),
    table_row_num: Style::new().fg(Color::Rgb(98, 114, 164)),
    null_value: Style::new()
        .fg(Color::Rgb(98, 114, 164))
        .add_modifier(Modifier::ITALIC),
    sql_focused: Style::new()
        .fg(Color::Rgb(80, 250, 123))
        .add_modifier(Modifier::BOLD),
    sql_unfocused: Style::new().add_modifier(Modifier::BOLD),
    error: Style::new().fg(Color::Rgb(255, 85, 85)),
    summary: Style::new().fg(Color::Rgb(80, 250, 123)),
    vertical_col: Style::new()
        .fg(Color::Rgb(255, 121, 198))
        .add_modifier(Modifier::BOLD),
    completion_kw: Color::Rgb(139, 233, 253),
    completion_table: Color::Rgb(255, 184, 108),
    completion_column: Color::Rgb(255, 121, 198),
    completion_fn: Color::Rgb(80, 250, 123),
    completion_command: Color::Rgb(248, 248, 242),
    completion_selected: Style::new()
        .bg(Color::Rgb(68, 71, 90))
        .fg(Color::Rgb(248, 248, 242)),
    completion_border: Style::new().fg(Color::Rgb(189, 147, 249)),
    syntax_keyword: Style::new()
        .fg(Color::Rgb(255, 121, 198))
        .add_modifier(Modifier::BOLD),
    syntax_number: Style::new().fg(Color::Rgb(241, 250, 140)),
    syntax_string: Style::new().fg(Color::Rgb(241, 250, 140)),
    syntax_operator: Style::new().fg(Color::Rgb(98, 114, 164)),
    picker_selected: Style::new()
        .fg(Color::Rgb(139, 233, 253))
        .add_modifier(Modifier::BOLD),
    header_style: Style::new()
        .bg(Color::Rgb(68, 71, 90))
        .fg(Color::Rgb(248, 248, 242))
        .add_modifier(Modifier::BOLD),
    schema_browser_border: Style::new().fg(Color::Rgb(139, 233, 253)),
    tab_active: Style::new()
        .fg(Color::Rgb(139, 233, 253))
        .add_modifier(Modifier::BOLD),
    tab_inactive: Style::new()
        .fg(Color::Rgb(98, 114, 164))
        .add_modifier(Modifier::BOLD),
    tab_border: Style::new().fg(Color::Rgb(98, 114, 164)),
    scrollbar_thumb: Style::new().fg(Color::Rgb(248, 248, 242)),
    scrollbar_track: Style::new().dim(),
    border_primary: Style::new().fg(Color::Rgb(139, 233, 253)),
    border_secondary: Style::new().fg(Color::Rgb(98, 114, 164)),
    command_palette_selected: Style::new()
        .bg(Color::Rgb(68, 71, 90))
        .fg(Color::Rgb(248, 248, 242)),
    help_key: Style::new()
        .fg(Color::Rgb(139, 233, 253))
        .add_modifier(Modifier::BOLD),
    help_desc: Style::new(),
    help_section: Style::new()
        .fg(Color::Rgb(139, 233, 253))
        .add_modifier(Modifier::BOLD),

    top_bar_bg: Color::Rgb(68, 71, 90),
    top_bar_active: Style::new()
        .fg(Color::Rgb(248, 248, 242))
        .bg(Color::Rgb(68, 71, 90))
        .add_modifier(Modifier::BOLD),
    top_bar_inactive: Style::new()
        .fg(Color::Rgb(139, 233, 253))
        .bg(Color::Rgb(68, 71, 90)),
    bg: Color::Rgb(40, 42, 54),
    bottom_bar_bg: Color::Rgb(33, 34, 44),
    bottom_bar_fg: Color::Rgb(248, 248, 242),
});

pub static NORD: LazyLock<Theme> = LazyLock::new(|| Theme {
    name: "nord",
    header_fg: Color::Rgb(236, 239, 244),
    header_bg: Color::Rgb(46, 52, 64),
    input_fg: Color::Rgb(236, 239, 244),
    input_bg: Color::Rgb(59, 66, 82),
    status_fg: Color::Rgb(236, 239, 244),
    status_bg: Color::Rgb(46, 52, 64),
    table_header_focused: Style::new()
        .fg(Color::Rgb(136, 192, 208))
        .add_modifier(Modifier::BOLD),
    table_header_unfocused: Style::new().fg(Color::Rgb(76, 86, 106)),
    table_row_alt_bg: Color::Rgb(59, 66, 82),
    table_row_num: Style::new().fg(Color::Rgb(76, 86, 106)),
    null_value: Style::new()
        .fg(Color::Rgb(76, 86, 106))
        .add_modifier(Modifier::ITALIC),
    sql_focused: Style::new()
        .fg(Color::Rgb(163, 190, 140))
        .add_modifier(Modifier::BOLD),
    sql_unfocused: Style::new().add_modifier(Modifier::BOLD),
    error: Style::new().fg(Color::Rgb(191, 97, 106)),
    summary: Style::new().fg(Color::Rgb(163, 190, 140)),
    vertical_col: Style::new()
        .fg(Color::Rgb(180, 142, 173))
        .add_modifier(Modifier::BOLD),
    completion_kw: Color::Rgb(136, 192, 208),
    completion_table: Color::Rgb(208, 135, 112),
    completion_column: Color::Rgb(180, 142, 173),
    completion_fn: Color::Rgb(163, 190, 140),
    completion_command: Color::Rgb(236, 239, 244),
    completion_selected: Style::new()
        .bg(Color::Rgb(59, 66, 82))
        .fg(Color::Rgb(236, 239, 244)),
    completion_border: Style::new().fg(Color::Rgb(136, 192, 208)),
    syntax_keyword: Style::new()
        .fg(Color::Rgb(136, 192, 208))
        .add_modifier(Modifier::BOLD),
    syntax_number: Style::new().fg(Color::Rgb(235, 203, 139)),
    syntax_string: Style::new().fg(Color::Rgb(163, 190, 140)),
    syntax_operator: Style::new().fg(Color::Rgb(76, 86, 106)),
    picker_selected: Style::new()
        .fg(Color::Rgb(136, 192, 208))
        .add_modifier(Modifier::BOLD),
    header_style: Style::new()
        .bg(Color::Rgb(46, 52, 64))
        .fg(Color::Rgb(236, 239, 244))
        .add_modifier(Modifier::BOLD),
    schema_browser_border: Style::new().fg(Color::Rgb(136, 192, 208)),
    tab_active: Style::new()
        .fg(Color::Rgb(136, 192, 208))
        .add_modifier(Modifier::BOLD),
    tab_inactive: Style::new()
        .fg(Color::Rgb(76, 86, 106))
        .add_modifier(Modifier::BOLD),
    tab_border: Style::new().fg(Color::Rgb(76, 86, 106)),
    scrollbar_thumb: Style::new().fg(Color::Rgb(236, 239, 244)),
    scrollbar_track: Style::new().dim(),
    border_primary: Style::new().fg(Color::Rgb(136, 192, 208)),
    border_secondary: Style::new().fg(Color::Rgb(76, 86, 106)),
    command_palette_selected: Style::new()
        .bg(Color::Rgb(59, 66, 82))
        .fg(Color::Rgb(236, 239, 244)),
    help_key: Style::new()
        .fg(Color::Rgb(136, 192, 208))
        .add_modifier(Modifier::BOLD),
    help_desc: Style::new(),
    help_section: Style::new()
        .fg(Color::Rgb(136, 192, 208))
        .add_modifier(Modifier::BOLD),

    top_bar_bg: Color::Rgb(46, 52, 64),
    top_bar_active: Style::new()
        .fg(Color::Rgb(236, 239, 244))
        .bg(Color::Rgb(46, 52, 64))
        .add_modifier(Modifier::BOLD),
    top_bar_inactive: Style::new()
        .fg(Color::Rgb(136, 192, 208))
        .bg(Color::Rgb(46, 52, 64)),
    bg: Color::Rgb(46, 52, 64),
    bottom_bar_bg: Color::Rgb(46, 52, 64),
    bottom_bar_fg: Color::Rgb(236, 239, 244),
});

pub static MONOKAI: LazyLock<Theme> = LazyLock::new(|| Theme {
    name: "monokai",
    header_fg: Color::Rgb(248, 248, 242),
    header_bg: Color::Rgb(39, 40, 34),
    input_fg: Color::Rgb(248, 248, 242),
    input_bg: Color::Rgb(39, 40, 34),
    status_fg: Color::Rgb(248, 248, 242),
    status_bg: Color::Rgb(33, 34, 28),
    table_header_focused: Style::new()
        .fg(Color::Rgb(102, 217, 239))
        .add_modifier(Modifier::BOLD),
    table_header_unfocused: Style::new().fg(Color::Rgb(117, 113, 94)),
    table_row_alt_bg: Color::Rgb(50, 51, 42),
    table_row_num: Style::new().fg(Color::Rgb(117, 113, 94)),
    null_value: Style::new()
        .fg(Color::Rgb(117, 113, 94))
        .add_modifier(Modifier::ITALIC),
    sql_focused: Style::new()
        .fg(Color::Rgb(166, 226, 46))
        .add_modifier(Modifier::BOLD),
    sql_unfocused: Style::new().add_modifier(Modifier::BOLD),
    error: Style::new().fg(Color::Rgb(249, 38, 114)),
    summary: Style::new().fg(Color::Rgb(166, 226, 46)),
    vertical_col: Style::new()
        .fg(Color::Rgb(249, 38, 114))
        .add_modifier(Modifier::BOLD),
    completion_kw: Color::Rgb(102, 217, 239),
    completion_table: Color::Rgb(253, 151, 31),
    completion_column: Color::Rgb(249, 38, 114),
    completion_fn: Color::Rgb(166, 226, 46),
    completion_command: Color::Rgb(248, 248, 242),
    completion_selected: Style::new()
        .bg(Color::Rgb(50, 51, 42))
        .fg(Color::Rgb(248, 248, 242)),
    completion_border: Style::new().fg(Color::Rgb(174, 129, 255)),
    syntax_keyword: Style::new()
        .fg(Color::Rgb(249, 38, 114))
        .add_modifier(Modifier::BOLD),
    syntax_number: Style::new().fg(Color::Rgb(174, 129, 255)),
    syntax_string: Style::new().fg(Color::Rgb(230, 219, 116)),
    syntax_operator: Style::new().fg(Color::Rgb(117, 113, 94)),
    picker_selected: Style::new()
        .fg(Color::Rgb(102, 217, 239))
        .add_modifier(Modifier::BOLD),
    header_style: Style::new()
        .bg(Color::Rgb(39, 40, 34))
        .fg(Color::Rgb(248, 248, 242))
        .add_modifier(Modifier::BOLD),
    schema_browser_border: Style::new().fg(Color::Rgb(102, 217, 239)),
    tab_active: Style::new()
        .fg(Color::Rgb(102, 217, 239))
        .add_modifier(Modifier::BOLD),
    tab_inactive: Style::new()
        .fg(Color::Rgb(117, 113, 94))
        .add_modifier(Modifier::BOLD),
    tab_border: Style::new().fg(Color::Rgb(117, 113, 94)),
    scrollbar_thumb: Style::new().fg(Color::Rgb(248, 248, 242)),
    scrollbar_track: Style::new().dim(),
    border_primary: Style::new().fg(Color::Rgb(102, 217, 239)),
    border_secondary: Style::new().fg(Color::Rgb(117, 113, 94)),
    command_palette_selected: Style::new()
        .bg(Color::Rgb(50, 51, 42))
        .fg(Color::Rgb(248, 248, 242)),
    help_key: Style::new()
        .fg(Color::Rgb(102, 217, 239))
        .add_modifier(Modifier::BOLD),
    help_desc: Style::new(),
    help_section: Style::new()
        .fg(Color::Rgb(102, 217, 239))
        .add_modifier(Modifier::BOLD),

    top_bar_bg: Color::Rgb(39, 40, 34),
    top_bar_active: Style::new()
        .fg(Color::Rgb(248, 248, 242))
        .bg(Color::Rgb(39, 40, 34))
        .add_modifier(Modifier::BOLD),
    top_bar_inactive: Style::new()
        .fg(Color::Rgb(102, 217, 239))
        .bg(Color::Rgb(39, 40, 34)),
    bg: Color::Rgb(39, 40, 34),
    bottom_bar_bg: Color::Rgb(33, 34, 28),
    bottom_bar_fg: Color::Rgb(248, 248, 242),
});

pub static LIGHT: LazyLock<Theme> = LazyLock::new(|| Theme {
    name: "light",
    header_fg: Color::White,
    header_bg: Color::Blue,
    input_fg: Color::Black,
    input_bg: Color::White,
    status_fg: Color::Black,
    status_bg: Color::Gray,
    table_header_focused: Style::new().fg(Color::Blue).add_modifier(Modifier::BOLD),
    table_header_unfocused: Style::new()
        .fg(Color::DarkGray)
        .add_modifier(Modifier::BOLD),
    table_row_alt_bg: Color::Gray,
    table_row_num: Style::new().fg(Color::DarkGray),
    null_value: Style::new()
        .fg(Color::DarkGray)
        .add_modifier(Modifier::ITALIC),
    sql_focused: Style::new().fg(Color::Blue).add_modifier(Modifier::BOLD),
    sql_unfocused: Style::new().add_modifier(Modifier::BOLD),
    error: Style::new().fg(Color::Red),
    summary: Style::new().fg(Color::Green),
    vertical_col: Style::new().fg(Color::Blue).add_modifier(Modifier::BOLD),
    completion_kw: Color::Blue,
    completion_table: Color::Rgb(178, 107, 0),
    completion_column: Color::Magenta,
    completion_fn: Color::Green,
    completion_command: Color::DarkGray,
    completion_selected: Style::new().bg(Color::Blue).fg(Color::White),
    completion_border: Style::new().fg(Color::Blue),
    syntax_keyword: Style::new().fg(Color::Blue).add_modifier(Modifier::BOLD),
    syntax_number: Style::new().fg(Color::Rgb(178, 107, 0)),
    syntax_string: Style::new().fg(Color::Green),
    syntax_operator: Style::new().fg(Color::DarkGray).add_modifier(Modifier::DIM),
    picker_selected: Style::new().fg(Color::Blue).add_modifier(Modifier::BOLD),
    header_style: Style::new()
        .bg(Color::Blue)
        .fg(Color::White)
        .add_modifier(Modifier::BOLD),
    schema_browser_border: Style::new().fg(Color::Blue),
    tab_active: Style::new().fg(Color::Blue).add_modifier(Modifier::BOLD),
    tab_inactive: Style::new()
        .fg(Color::DarkGray)
        .add_modifier(Modifier::BOLD),
    tab_border: Style::new().fg(Color::DarkGray),
    scrollbar_thumb: Style::new().fg(Color::Black),
    scrollbar_track: Style::new().dim(),
    border_primary: Style::new().fg(Color::Blue),
    border_secondary: Style::new().fg(Color::DarkGray),
    command_palette_selected: Style::new().bg(Color::Blue).fg(Color::White),
    help_key: Style::new().fg(Color::Blue).add_modifier(Modifier::BOLD),
    help_desc: Style::new().fg(Color::Black),
    help_section: Style::new().fg(Color::Blue).add_modifier(Modifier::BOLD),

    top_bar_bg: Color::Blue,
    top_bar_active: Style::new()
        .fg(Color::White)
        .bg(Color::Blue)
        .add_modifier(Modifier::BOLD),
    top_bar_inactive: Style::new().fg(Color::White).bg(Color::Blue),
    bg: Color::White,
    bottom_bar_bg: Color::Gray,
    bottom_bar_fg: Color::Black,
});

pub static TOKYO_NIGHT: LazyLock<Theme> = LazyLock::new(|| Theme {
    name: "tokyo-night",
    header_fg: Color::Rgb(169, 177, 214),
    header_bg: Color::Rgb(26, 27, 38),
    input_fg: Color::Rgb(169, 177, 214),
    input_bg: Color::Rgb(36, 38, 59),
    status_fg: Color::Rgb(169, 177, 214),
    status_bg: Color::Rgb(26, 27, 38),
    table_header_focused: Style::new()
        .fg(Color::Rgb(122, 162, 247))
        .add_modifier(Modifier::BOLD),
    table_header_unfocused: Style::new().fg(Color::Rgb(86, 95, 137)),
    table_row_alt_bg: Color::Rgb(36, 38, 59),
    table_row_num: Style::new().fg(Color::Rgb(86, 95, 137)),
    null_value: Style::new()
        .fg(Color::Rgb(86, 95, 137))
        .add_modifier(Modifier::ITALIC),
    sql_focused: Style::new()
        .fg(Color::Rgb(158, 206, 106))
        .add_modifier(Modifier::BOLD),
    sql_unfocused: Style::new().add_modifier(Modifier::BOLD),
    error: Style::new().fg(Color::Rgb(247, 118, 142)),
    summary: Style::new().fg(Color::Rgb(158, 206, 106)),
    vertical_col: Style::new()
        .fg(Color::Rgb(187, 154, 247))
        .add_modifier(Modifier::BOLD),
    completion_kw: Color::Rgb(122, 162, 247),
    completion_table: Color::Rgb(255, 158, 100),
    completion_column: Color::Rgb(187, 154, 247),
    completion_fn: Color::Rgb(158, 206, 106),
    completion_command: Color::Rgb(169, 177, 214),
    completion_selected: Style::new()
        .bg(Color::Rgb(36, 38, 59))
        .fg(Color::Rgb(169, 177, 214)),
    completion_border: Style::new().fg(Color::Rgb(122, 162, 247)),
    syntax_keyword: Style::new()
        .fg(Color::Rgb(187, 154, 247))
        .add_modifier(Modifier::BOLD),
    syntax_number: Style::new().fg(Color::Rgb(224, 175, 104)),
    syntax_string: Style::new().fg(Color::Rgb(158, 206, 106)),
    syntax_operator: Style::new().fg(Color::Rgb(86, 95, 137)),
    picker_selected: Style::new()
        .fg(Color::Rgb(122, 162, 247))
        .add_modifier(Modifier::BOLD),
    header_style: Style::new()
        .bg(Color::Rgb(26, 27, 38))
        .fg(Color::Rgb(169, 177, 214))
        .add_modifier(Modifier::BOLD),
    schema_browser_border: Style::new().fg(Color::Rgb(122, 162, 247)),
    tab_active: Style::new()
        .fg(Color::Rgb(122, 162, 247))
        .add_modifier(Modifier::BOLD),
    tab_inactive: Style::new()
        .fg(Color::Rgb(86, 95, 137))
        .add_modifier(Modifier::BOLD),
    tab_border: Style::new().fg(Color::Rgb(86, 95, 137)),
    scrollbar_thumb: Style::new().fg(Color::Rgb(169, 177, 214)),
    scrollbar_track: Style::new().dim(),
    border_primary: Style::new().fg(Color::Rgb(122, 162, 247)),
    border_secondary: Style::new().fg(Color::Rgb(86, 95, 137)),
    command_palette_selected: Style::new()
        .bg(Color::Rgb(36, 38, 59))
        .fg(Color::Rgb(169, 177, 214)),
    help_key: Style::new()
        .fg(Color::Rgb(122, 162, 247))
        .add_modifier(Modifier::BOLD),
    help_desc: Style::new(),
    help_section: Style::new()
        .fg(Color::Rgb(122, 162, 247))
        .add_modifier(Modifier::BOLD),

    top_bar_bg: Color::Rgb(26, 27, 38),
    top_bar_active: Style::new()
        .fg(Color::Rgb(169, 177, 214))
        .bg(Color::Rgb(26, 27, 38))
        .add_modifier(Modifier::BOLD),
    top_bar_inactive: Style::new()
        .fg(Color::Rgb(122, 162, 247))
        .bg(Color::Rgb(26, 27, 38)),
    bg: Color::Rgb(26, 27, 38),
    bottom_bar_bg: Color::Rgb(26, 27, 38),
    bottom_bar_fg: Color::Rgb(169, 177, 214),
});

pub static CATPPUCCIN: LazyLock<Theme> = LazyLock::new(|| Theme {
    name: "catppuccin",
    header_fg: Color::Rgb(205, 214, 244),
    header_bg: Color::Rgb(30, 30, 46),
    input_fg: Color::Rgb(205, 214, 244),
    input_bg: Color::Rgb(49, 50, 68),
    status_fg: Color::Rgb(205, 214, 244),
    status_bg: Color::Rgb(24, 24, 37),
    table_header_focused: Style::new()
        .fg(Color::Rgb(137, 180, 250))
        .add_modifier(Modifier::BOLD),
    table_header_unfocused: Style::new().fg(Color::Rgb(108, 112, 134)),
    table_row_alt_bg: Color::Rgb(49, 50, 68),
    table_row_num: Style::new().fg(Color::Rgb(108, 112, 134)),
    null_value: Style::new()
        .fg(Color::Rgb(108, 112, 134))
        .add_modifier(Modifier::ITALIC),
    sql_focused: Style::new()
        .fg(Color::Rgb(166, 227, 161))
        .add_modifier(Modifier::BOLD),
    sql_unfocused: Style::new().add_modifier(Modifier::BOLD),
    error: Style::new().fg(Color::Rgb(243, 139, 168)),
    summary: Style::new().fg(Color::Rgb(166, 227, 161)),
    vertical_col: Style::new()
        .fg(Color::Rgb(203, 166, 247))
        .add_modifier(Modifier::BOLD),
    completion_kw: Color::Rgb(137, 180, 250),
    completion_table: Color::Rgb(250, 179, 135),
    completion_column: Color::Rgb(203, 166, 247),
    completion_fn: Color::Rgb(166, 227, 161),
    completion_command: Color::Rgb(205, 214, 244),
    completion_selected: Style::new()
        .bg(Color::Rgb(49, 50, 68))
        .fg(Color::Rgb(205, 214, 244)),
    completion_border: Style::new().fg(Color::Rgb(137, 180, 250)),
    syntax_keyword: Style::new()
        .fg(Color::Rgb(203, 166, 247))
        .add_modifier(Modifier::BOLD),
    syntax_number: Style::new().fg(Color::Rgb(249, 226, 175)),
    syntax_string: Style::new().fg(Color::Rgb(166, 227, 161)),
    syntax_operator: Style::new().fg(Color::Rgb(108, 112, 134)),
    picker_selected: Style::new()
        .fg(Color::Rgb(137, 180, 250))
        .add_modifier(Modifier::BOLD),
    header_style: Style::new()
        .bg(Color::Rgb(30, 30, 46))
        .fg(Color::Rgb(205, 214, 244))
        .add_modifier(Modifier::BOLD),
    schema_browser_border: Style::new().fg(Color::Rgb(137, 180, 250)),
    tab_active: Style::new()
        .fg(Color::Rgb(137, 180, 250))
        .add_modifier(Modifier::BOLD),
    tab_inactive: Style::new()
        .fg(Color::Rgb(108, 112, 134))
        .add_modifier(Modifier::BOLD),
    tab_border: Style::new().fg(Color::Rgb(108, 112, 134)),
    scrollbar_thumb: Style::new().fg(Color::Rgb(205, 214, 244)),
    scrollbar_track: Style::new().dim(),
    border_primary: Style::new().fg(Color::Rgb(137, 180, 250)),
    border_secondary: Style::new().fg(Color::Rgb(108, 112, 134)),
    command_palette_selected: Style::new()
        .bg(Color::Rgb(49, 50, 68))
        .fg(Color::Rgb(205, 214, 244)),
    help_key: Style::new()
        .fg(Color::Rgb(137, 180, 250))
        .add_modifier(Modifier::BOLD),
    help_desc: Style::new(),
    help_section: Style::new()
        .fg(Color::Rgb(137, 180, 250))
        .add_modifier(Modifier::BOLD),

    top_bar_bg: Color::Rgb(30, 30, 46),
    top_bar_active: Style::new()
        .fg(Color::Rgb(205, 214, 244))
        .bg(Color::Rgb(30, 30, 46))
        .add_modifier(Modifier::BOLD),
    top_bar_inactive: Style::new()
        .fg(Color::Rgb(137, 180, 250))
        .bg(Color::Rgb(30, 30, 46)),
    bg: Color::Rgb(30, 30, 46),
    bottom_bar_bg: Color::Rgb(24, 24, 37),
    bottom_bar_fg: Color::Rgb(205, 214, 244),
});
