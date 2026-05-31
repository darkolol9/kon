use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Span;
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

    pub bg: Color,
    pub top_bar_bg: Color,
    pub top_bar_active: Style,
    pub top_bar_inactive: Style,
    pub bottom_bar_bg: Color,
    pub bottom_bar_fg: Color,
}

impl Theme {
    pub fn sample(&self) -> Vec<Span<'static>> {
        vec![
            Span::styled("SELECT", self.syntax_keyword),
            Span::raw(" "),
            Span::styled("*", self.syntax_operator),
            Span::raw(" FROM "),
            Span::styled("users", Style::new()),
            Span::raw(" LIMIT "),
            Span::styled("42", self.syntax_number),
            Span::styled(";", self.syntax_operator),
        ]
    }
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
        &GRUVBOX,
        &SOLARIZED_DARK,
        &ONE_DARK,
        &ROSE_PINE,
        &EVERFOREST,
        &AYU_DARK,
    ]
});

pub fn theme_names() -> Vec<&'static str> {
    ALL_THEMES.iter().map(|t| t.name).collect()
}

pub fn from_name(name: &str) -> Option<&'static Theme> {
    ALL_THEMES.iter().copied().find(|t| t.name == name)
}

// ── Default ──
pub static DEFAULT: LazyLock<Theme> = LazyLock::new(|| Theme {
    name: "default",
    header_fg: Color::White,
    header_bg: Color::Blue,
    input_fg: Color::White,
    input_bg: Color::Rgb(28, 28, 28),
    status_fg: Color::White,
    status_bg: Color::Rgb(16, 16, 16),
    table_header_focused: Style::new().fg(Color::Cyan).add_modifier(Modifier::BOLD),
    table_header_unfocused: Style::new()
        .fg(Color::DarkGray)
        .add_modifier(Modifier::BOLD),
    table_row_alt_bg: Color::Rgb(35, 35, 35),
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
    completion_selected: Style::new().bg(Color::Rgb(0, 80, 200)).fg(Color::White),
    completion_border: Style::new().fg(Color::Cyan),
    syntax_keyword: Style::new().fg(Color::Cyan).add_modifier(Modifier::BOLD),
    syntax_number: Style::new().fg(Color::Yellow),
    syntax_string: Style::new().fg(Color::Green),
    syntax_operator: Style::new().fg(Color::Rgb(140, 140, 140)),
    picker_selected: Style::new().fg(Color::Cyan).add_modifier(Modifier::BOLD),
    header_style: Style::new()
        .bg(Color::Blue)
        .fg(Color::White)
        .add_modifier(Modifier::BOLD),
    schema_browser_border: Style::new().fg(Color::Cyan),
    tab_active: Style::new().fg(Color::Cyan).add_modifier(Modifier::BOLD),
    tab_inactive: Style::new()
        .fg(Color::Rgb(140, 180, 230))
        .add_modifier(Modifier::BOLD),
    tab_border: Style::new().fg(Color::Rgb(60, 80, 140)),
    scrollbar_thumb: Style::new().fg(Color::White),
    scrollbar_track: Style::new().dim(),
    border_primary: Style::new().fg(Color::Cyan),
    border_secondary: Style::new().fg(Color::Rgb(80, 80, 80)),
    command_palette_selected: Style::new().bg(Color::Rgb(0, 80, 200)).fg(Color::White),
    help_key: Style::new().fg(Color::Cyan).add_modifier(Modifier::BOLD),
    help_desc: Style::new(),
    help_section: Style::new().fg(Color::Cyan).add_modifier(Modifier::BOLD),

    top_bar_bg: Color::Blue,
    top_bar_active: Style::new()
        .fg(Color::White)
        .bg(Color::Rgb(0, 100, 255))
        .add_modifier(Modifier::BOLD),
    top_bar_inactive: Style::new().fg(Color::Rgb(180, 210, 255)).bg(Color::Blue),
    bg: Color::Rgb(16, 16, 16),
    bottom_bar_bg: Color::Rgb(22, 22, 22),
    bottom_bar_fg: Color::Rgb(200, 200, 200),
});

// ── Dracula ──
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
        .fg(Color::Rgb(139, 233, 253))
        .add_modifier(Modifier::DIM),
    tab_border: Style::new().fg(Color::Rgb(98, 114, 164)),
    scrollbar_thumb: Style::new().fg(Color::Rgb(248, 248, 242)),
    scrollbar_track: Style::new().dim(),
    border_primary: Style::new().fg(Color::Rgb(139, 233, 253)),
    border_secondary: Style::new().fg(Color::Rgb(110, 126, 176)),
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
        .bg(Color::Rgb(98, 114, 164))
        .add_modifier(Modifier::BOLD),
    top_bar_inactive: Style::new()
        .fg(Color::Rgb(139, 233, 253))
        .bg(Color::Rgb(68, 71, 90)),
    bg: Color::Rgb(40, 42, 54),
    bottom_bar_bg: Color::Rgb(33, 34, 44),
    bottom_bar_fg: Color::Rgb(248, 248, 242),
});

// ── Nord ──
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
        .fg(Color::Rgb(136, 192, 208))
        .add_modifier(Modifier::DIM),
    tab_border: Style::new().fg(Color::Rgb(76, 86, 106)),
    scrollbar_thumb: Style::new().fg(Color::Rgb(236, 239, 244)),
    scrollbar_track: Style::new().dim(),
    border_primary: Style::new().fg(Color::Rgb(136, 192, 208)),
    border_secondary: Style::new().fg(Color::Rgb(90, 100, 120)),
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

    top_bar_bg: Color::Rgb(56, 62, 74),
    top_bar_active: Style::new()
        .fg(Color::Rgb(236, 239, 244))
        .bg(Color::Rgb(76, 86, 106))
        .add_modifier(Modifier::BOLD),
    top_bar_inactive: Style::new()
        .fg(Color::Rgb(136, 192, 208))
        .bg(Color::Rgb(56, 62, 74)),
    bg: Color::Rgb(46, 52, 64),
    bottom_bar_bg: Color::Rgb(56, 62, 74),
    bottom_bar_fg: Color::Rgb(236, 239, 244),
});

// ── Monokai ──
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
        .fg(Color::Rgb(102, 217, 239))
        .add_modifier(Modifier::DIM),
    tab_border: Style::new().fg(Color::Rgb(117, 113, 94)),
    scrollbar_thumb: Style::new().fg(Color::Rgb(248, 248, 242)),
    scrollbar_track: Style::new().dim(),
    border_primary: Style::new().fg(Color::Rgb(102, 217, 239)),
    border_secondary: Style::new().fg(Color::Rgb(130, 126, 107)),
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

    top_bar_bg: Color::Rgb(49, 50, 44),
    top_bar_active: Style::new()
        .fg(Color::Rgb(248, 248, 242))
        .bg(Color::Rgb(80, 80, 60))
        .add_modifier(Modifier::BOLD),
    top_bar_inactive: Style::new()
        .fg(Color::Rgb(102, 217, 239))
        .bg(Color::Rgb(49, 50, 44)),
    bg: Color::Rgb(39, 40, 34),
    bottom_bar_bg: Color::Rgb(33, 34, 28),
    bottom_bar_fg: Color::Rgb(248, 248, 242),
});

// ── Light ──
pub static LIGHT: LazyLock<Theme> = LazyLock::new(|| Theme {
    name: "light",
    header_fg: Color::White,
    header_bg: Color::Blue,
    input_fg: Color::Rgb(40, 40, 40),
    input_bg: Color::Rgb(250, 250, 250),
    status_fg: Color::Rgb(40, 40, 40),
    status_bg: Color::Rgb(230, 230, 230),
    table_header_focused: Style::new().fg(Color::Blue).add_modifier(Modifier::BOLD),
    table_header_unfocused: Style::new()
        .fg(Color::Rgb(150, 150, 150))
        .add_modifier(Modifier::BOLD),
    table_row_alt_bg: Color::Rgb(240, 240, 240),
    table_row_num: Style::new().fg(Color::Rgb(180, 180, 180)),
    null_value: Style::new()
        .fg(Color::Rgb(180, 180, 180))
        .add_modifier(Modifier::ITALIC),
    sql_focused: Style::new().fg(Color::Blue).add_modifier(Modifier::BOLD),
    sql_unfocused: Style::new().add_modifier(Modifier::BOLD),
    error: Style::new().fg(Color::Red),
    summary: Style::new().fg(Color::Rgb(0, 140, 0)),
    vertical_col: Style::new().fg(Color::Blue).add_modifier(Modifier::BOLD),
    completion_kw: Color::Blue,
    completion_table: Color::Rgb(178, 107, 0),
    completion_column: Color::Magenta,
    completion_fn: Color::Green,
    completion_command: Color::Rgb(100, 100, 100),
    completion_selected: Style::new().bg(Color::Rgb(200, 220, 255)).fg(Color::Black),
    completion_border: Style::new().fg(Color::Blue),
    syntax_keyword: Style::new().fg(Color::Blue).add_modifier(Modifier::BOLD),
    syntax_number: Style::new().fg(Color::Rgb(178, 107, 0)),
    syntax_string: Style::new().fg(Color::Rgb(0, 140, 0)),
    syntax_operator: Style::new().fg(Color::Rgb(140, 140, 140)),
    picker_selected: Style::new().fg(Color::Blue).add_modifier(Modifier::BOLD),
    header_style: Style::new()
        .bg(Color::Blue)
        .fg(Color::White)
        .add_modifier(Modifier::BOLD),
    schema_browser_border: Style::new().fg(Color::Blue),
    tab_active: Style::new().fg(Color::Blue).add_modifier(Modifier::BOLD),
    tab_inactive: Style::new()
        .fg(Color::Rgb(180, 210, 240))
        .add_modifier(Modifier::BOLD),
    tab_border: Style::new().fg(Color::Rgb(120, 160, 200)),
    scrollbar_thumb: Style::new().fg(Color::Rgb(80, 80, 80)),
    scrollbar_track: Style::new().dim(),
    border_primary: Style::new().fg(Color::Blue),
    border_secondary: Style::new().fg(Color::Rgb(200, 200, 200)),
    command_palette_selected: Style::new().bg(Color::Rgb(200, 220, 255)).fg(Color::Black),
    help_key: Style::new().fg(Color::Blue).add_modifier(Modifier::BOLD),
    help_desc: Style::new().fg(Color::Rgb(80, 80, 80)),
    help_section: Style::new().fg(Color::Blue).add_modifier(Modifier::BOLD),

    top_bar_bg: Color::Blue,
    top_bar_active: Style::new()
        .fg(Color::White)
        .bg(Color::Rgb(0, 100, 255))
        .add_modifier(Modifier::BOLD),
    top_bar_inactive: Style::new().fg(Color::Rgb(200, 220, 255)).bg(Color::Blue),
    bg: Color::White,
    bottom_bar_bg: Color::Rgb(240, 240, 240),
    bottom_bar_fg: Color::Rgb(60, 60, 60),
});

// ── Tokyo Night ──
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
        .fg(Color::Rgb(122, 162, 247))
        .add_modifier(Modifier::DIM),
    tab_border: Style::new().fg(Color::Rgb(86, 95, 137)),
    scrollbar_thumb: Style::new().fg(Color::Rgb(169, 177, 214)),
    scrollbar_track: Style::new().dim(),
    border_primary: Style::new().fg(Color::Rgb(122, 162, 247)),
    border_secondary: Style::new().fg(Color::Rgb(100, 109, 151)),
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

    top_bar_bg: Color::Rgb(36, 37, 48),
    top_bar_active: Style::new()
        .fg(Color::Rgb(169, 177, 214))
        .bg(Color::Rgb(56, 58, 79))
        .add_modifier(Modifier::BOLD),
    top_bar_inactive: Style::new()
        .fg(Color::Rgb(122, 162, 247))
        .bg(Color::Rgb(36, 37, 48)),
    bg: Color::Rgb(26, 27, 38),
    bottom_bar_bg: Color::Rgb(36, 37, 48),
    bottom_bar_fg: Color::Rgb(169, 177, 214),
});

// ── Catppuccin ──
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
        .fg(Color::Rgb(137, 180, 250))
        .add_modifier(Modifier::DIM),
    tab_border: Style::new().fg(Color::Rgb(108, 112, 134)),
    scrollbar_thumb: Style::new().fg(Color::Rgb(205, 214, 244)),
    scrollbar_track: Style::new().dim(),
    border_primary: Style::new().fg(Color::Rgb(137, 180, 250)),
    border_secondary: Style::new().fg(Color::Rgb(120, 124, 146)),
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

    top_bar_bg: Color::Rgb(40, 40, 56),
    top_bar_active: Style::new()
        .fg(Color::Rgb(205, 214, 244))
        .bg(Color::Rgb(69, 71, 90))
        .add_modifier(Modifier::BOLD),
    top_bar_inactive: Style::new()
        .fg(Color::Rgb(137, 180, 250))
        .bg(Color::Rgb(40, 40, 56)),
    bg: Color::Rgb(30, 30, 46),
    bottom_bar_bg: Color::Rgb(24, 24, 37),
    bottom_bar_fg: Color::Rgb(205, 214, 244),
});

// ── Gruvbox Dark ──
pub static GRUVBOX: LazyLock<Theme> = LazyLock::new(|| Theme {
    name: "gruvbox",
    header_fg: Color::Rgb(235, 219, 178),
    header_bg: Color::Rgb(40, 40, 40),
    input_fg: Color::Rgb(235, 219, 178),
    input_bg: Color::Rgb(50, 50, 50),
    status_fg: Color::Rgb(235, 219, 178),
    status_bg: Color::Rgb(28, 28, 28),
    table_header_focused: Style::new()
        .fg(Color::Rgb(254, 128, 25))
        .add_modifier(Modifier::BOLD),
    table_header_unfocused: Style::new().fg(Color::Rgb(124, 111, 100)),
    table_row_alt_bg: Color::Rgb(50, 50, 50),
    table_row_num: Style::new().fg(Color::Rgb(124, 111, 100)),
    null_value: Style::new()
        .fg(Color::Rgb(124, 111, 100))
        .add_modifier(Modifier::ITALIC),
    sql_focused: Style::new()
        .fg(Color::Rgb(184, 187, 38))
        .add_modifier(Modifier::BOLD),
    sql_unfocused: Style::new().add_modifier(Modifier::BOLD),
    error: Style::new().fg(Color::Rgb(251, 73, 52)),
    summary: Style::new().fg(Color::Rgb(184, 187, 38)),
    vertical_col: Style::new()
        .fg(Color::Rgb(211, 134, 155))
        .add_modifier(Modifier::BOLD),
    completion_kw: Color::Rgb(131, 165, 152),
    completion_table: Color::Rgb(254, 128, 25),
    completion_column: Color::Rgb(211, 134, 155),
    completion_fn: Color::Rgb(184, 187, 38),
    completion_command: Color::Rgb(235, 219, 178),
    completion_selected: Style::new()
        .bg(Color::Rgb(50, 50, 50))
        .fg(Color::Rgb(235, 219, 178)),
    completion_border: Style::new().fg(Color::Rgb(211, 134, 155)),
    syntax_keyword: Style::new()
        .fg(Color::Rgb(254, 128, 25))
        .add_modifier(Modifier::BOLD),
    syntax_number: Style::new().fg(Color::Rgb(211, 134, 155)),
    syntax_string: Style::new().fg(Color::Rgb(184, 187, 38)),
    syntax_operator: Style::new().fg(Color::Rgb(124, 111, 100)),
    picker_selected: Style::new()
        .fg(Color::Rgb(131, 165, 152))
        .add_modifier(Modifier::BOLD),
    header_style: Style::new()
        .bg(Color::Rgb(40, 40, 40))
        .fg(Color::Rgb(235, 219, 178))
        .add_modifier(Modifier::BOLD),
    schema_browser_border: Style::new().fg(Color::Rgb(131, 165, 152)),
    tab_active: Style::new()
        .fg(Color::Rgb(131, 165, 152))
        .add_modifier(Modifier::BOLD),
    tab_inactive: Style::new()
        .fg(Color::Rgb(131, 165, 152))
        .add_modifier(Modifier::DIM),
    tab_border: Style::new().fg(Color::Rgb(124, 111, 100)),
    scrollbar_thumb: Style::new().fg(Color::Rgb(235, 219, 178)),
    scrollbar_track: Style::new().dim(),
    border_primary: Style::new().fg(Color::Rgb(131, 165, 152)),
    border_secondary: Style::new().fg(Color::Rgb(80, 73, 69)),
    command_palette_selected: Style::new()
        .bg(Color::Rgb(50, 50, 50))
        .fg(Color::Rgb(235, 219, 178)),
    help_key: Style::new()
        .fg(Color::Rgb(131, 165, 152))
        .add_modifier(Modifier::BOLD),
    help_desc: Style::new(),
    help_section: Style::new()
        .fg(Color::Rgb(131, 165, 152))
        .add_modifier(Modifier::BOLD),

    top_bar_bg: Color::Rgb(50, 50, 50),
    top_bar_active: Style::new()
        .fg(Color::Rgb(235, 219, 178))
        .bg(Color::Rgb(80, 73, 69))
        .add_modifier(Modifier::BOLD),
    top_bar_inactive: Style::new()
        .fg(Color::Rgb(131, 165, 152))
        .bg(Color::Rgb(50, 50, 50)),
    bg: Color::Rgb(40, 40, 40),
    bottom_bar_bg: Color::Rgb(28, 28, 28),
    bottom_bar_fg: Color::Rgb(235, 219, 178),
});

// ── Solarized Dark ──
pub static SOLARIZED_DARK: LazyLock<Theme> = LazyLock::new(|| Theme {
    name: "solarized-dark",
    header_fg: Color::Rgb(147, 161, 161),
    header_bg: Color::Rgb(7, 54, 66),
    input_fg: Color::Rgb(131, 148, 150),
    input_bg: Color::Rgb(0, 43, 54),
    status_fg: Color::Rgb(147, 161, 161),
    status_bg: Color::Rgb(7, 54, 66),
    table_header_focused: Style::new()
        .fg(Color::Rgb(42, 161, 152))
        .add_modifier(Modifier::BOLD),
    table_header_unfocused: Style::new().fg(Color::Rgb(88, 110, 117)),
    table_row_alt_bg: Color::Rgb(7, 54, 66),
    table_row_num: Style::new().fg(Color::Rgb(88, 110, 117)),
    null_value: Style::new()
        .fg(Color::Rgb(88, 110, 117))
        .add_modifier(Modifier::ITALIC),
    sql_focused: Style::new()
        .fg(Color::Rgb(133, 153, 0))
        .add_modifier(Modifier::BOLD),
    sql_unfocused: Style::new().add_modifier(Modifier::BOLD),
    error: Style::new().fg(Color::Rgb(220, 50, 47)),
    summary: Style::new().fg(Color::Rgb(133, 153, 0)),
    vertical_col: Style::new()
        .fg(Color::Rgb(211, 54, 130))
        .add_modifier(Modifier::BOLD),
    completion_kw: Color::Rgb(42, 161, 152),
    completion_table: Color::Rgb(203, 75, 22),
    completion_column: Color::Rgb(211, 54, 130),
    completion_fn: Color::Rgb(133, 153, 0),
    completion_command: Color::Rgb(131, 148, 150),
    completion_selected: Style::new()
        .bg(Color::Rgb(7, 54, 66))
        .fg(Color::Rgb(147, 161, 161)),
    completion_border: Style::new().fg(Color::Rgb(42, 161, 152)),
    syntax_keyword: Style::new()
        .fg(Color::Rgb(42, 161, 152))
        .add_modifier(Modifier::BOLD),
    syntax_number: Style::new().fg(Color::Rgb(211, 54, 130)),
    syntax_string: Style::new().fg(Color::Rgb(133, 153, 0)),
    syntax_operator: Style::new().fg(Color::Rgb(88, 110, 117)),
    picker_selected: Style::new()
        .fg(Color::Rgb(42, 161, 152))
        .add_modifier(Modifier::BOLD),
    header_style: Style::new()
        .bg(Color::Rgb(7, 54, 66))
        .fg(Color::Rgb(147, 161, 161))
        .add_modifier(Modifier::BOLD),
    schema_browser_border: Style::new().fg(Color::Rgb(42, 161, 152)),
    tab_active: Style::new()
        .fg(Color::Rgb(42, 161, 152))
        .add_modifier(Modifier::BOLD),
    tab_inactive: Style::new()
        .fg(Color::Rgb(42, 161, 152))
        .add_modifier(Modifier::DIM),
    tab_border: Style::new().fg(Color::Rgb(88, 110, 117)),
    scrollbar_thumb: Style::new().fg(Color::Rgb(147, 161, 161)),
    scrollbar_track: Style::new().dim(),
    border_primary: Style::new().fg(Color::Rgb(42, 161, 152)),
    border_secondary: Style::new().fg(Color::Rgb(30, 70, 80)),
    command_palette_selected: Style::new()
        .bg(Color::Rgb(7, 54, 66))
        .fg(Color::Rgb(147, 161, 161)),
    help_key: Style::new()
        .fg(Color::Rgb(42, 161, 152))
        .add_modifier(Modifier::BOLD),
    help_desc: Style::new(),
    help_section: Style::new()
        .fg(Color::Rgb(42, 161, 152))
        .add_modifier(Modifier::BOLD),

    top_bar_bg: Color::Rgb(7, 54, 66),
    top_bar_active: Style::new()
        .fg(Color::Rgb(147, 161, 161))
        .bg(Color::Rgb(30, 70, 80))
        .add_modifier(Modifier::BOLD),
    top_bar_inactive: Style::new()
        .fg(Color::Rgb(42, 161, 152))
        .bg(Color::Rgb(7, 54, 66)),
    bg: Color::Rgb(0, 43, 54),
    bottom_bar_bg: Color::Rgb(7, 54, 66),
    bottom_bar_fg: Color::Rgb(147, 161, 161),
});

// ── One Dark ──
pub static ONE_DARK: LazyLock<Theme> = LazyLock::new(|| Theme {
    name: "one-dark",
    header_fg: Color::Rgb(171, 178, 191),
    header_bg: Color::Rgb(40, 44, 52),
    input_fg: Color::Rgb(171, 178, 191),
    input_bg: Color::Rgb(30, 33, 39),
    status_fg: Color::Rgb(171, 178, 191),
    status_bg: Color::Rgb(22, 24, 28),
    table_header_focused: Style::new()
        .fg(Color::Rgb(97, 175, 239))
        .add_modifier(Modifier::BOLD),
    table_header_unfocused: Style::new().fg(Color::Rgb(92, 99, 112)),
    table_row_alt_bg: Color::Rgb(40, 44, 52),
    table_row_num: Style::new().fg(Color::Rgb(92, 99, 112)),
    null_value: Style::new()
        .fg(Color::Rgb(92, 99, 112))
        .add_modifier(Modifier::ITALIC),
    sql_focused: Style::new()
        .fg(Color::Rgb(152, 195, 121))
        .add_modifier(Modifier::BOLD),
    sql_unfocused: Style::new().add_modifier(Modifier::BOLD),
    error: Style::new().fg(Color::Rgb(224, 108, 117)),
    summary: Style::new().fg(Color::Rgb(152, 195, 121)),
    vertical_col: Style::new()
        .fg(Color::Rgb(198, 120, 221))
        .add_modifier(Modifier::BOLD),
    completion_kw: Color::Rgb(97, 175, 239),
    completion_table: Color::Rgb(209, 154, 102),
    completion_column: Color::Rgb(198, 120, 221),
    completion_fn: Color::Rgb(152, 195, 121),
    completion_command: Color::Rgb(171, 178, 191),
    completion_selected: Style::new()
        .bg(Color::Rgb(40, 44, 52))
        .fg(Color::Rgb(171, 178, 191)),
    completion_border: Style::new().fg(Color::Rgb(97, 175, 239)),
    syntax_keyword: Style::new()
        .fg(Color::Rgb(198, 120, 221))
        .add_modifier(Modifier::BOLD),
    syntax_number: Style::new().fg(Color::Rgb(209, 154, 102)),
    syntax_string: Style::new().fg(Color::Rgb(152, 195, 121)),
    syntax_operator: Style::new().fg(Color::Rgb(92, 99, 112)),
    picker_selected: Style::new()
        .fg(Color::Rgb(97, 175, 239))
        .add_modifier(Modifier::BOLD),
    header_style: Style::new()
        .bg(Color::Rgb(40, 44, 52))
        .fg(Color::Rgb(171, 178, 191))
        .add_modifier(Modifier::BOLD),
    schema_browser_border: Style::new().fg(Color::Rgb(97, 175, 239)),
    tab_active: Style::new()
        .fg(Color::Rgb(97, 175, 239))
        .add_modifier(Modifier::BOLD),
    tab_inactive: Style::new()
        .fg(Color::Rgb(97, 175, 239))
        .add_modifier(Modifier::DIM),
    tab_border: Style::new().fg(Color::Rgb(92, 99, 112)),
    scrollbar_thumb: Style::new().fg(Color::Rgb(171, 178, 191)),
    scrollbar_track: Style::new().dim(),
    border_primary: Style::new().fg(Color::Rgb(97, 175, 239)),
    border_secondary: Style::new().fg(Color::Rgb(60, 66, 77)),
    command_palette_selected: Style::new()
        .bg(Color::Rgb(40, 44, 52))
        .fg(Color::Rgb(171, 178, 191)),
    help_key: Style::new()
        .fg(Color::Rgb(97, 175, 239))
        .add_modifier(Modifier::BOLD),
    help_desc: Style::new(),
    help_section: Style::new()
        .fg(Color::Rgb(97, 175, 239))
        .add_modifier(Modifier::BOLD),

    top_bar_bg: Color::Rgb(40, 44, 52),
    top_bar_active: Style::new()
        .fg(Color::Rgb(171, 178, 191))
        .bg(Color::Rgb(60, 66, 77))
        .add_modifier(Modifier::BOLD),
    top_bar_inactive: Style::new()
        .fg(Color::Rgb(97, 175, 239))
        .bg(Color::Rgb(40, 44, 52)),
    bg: Color::Rgb(30, 33, 39),
    bottom_bar_bg: Color::Rgb(22, 24, 28),
    bottom_bar_fg: Color::Rgb(171, 178, 191),
});

// ── Rose Pine ──
pub static ROSE_PINE: LazyLock<Theme> = LazyLock::new(|| Theme {
    name: "rose-pine",
    header_fg: Color::Rgb(224, 222, 244),
    header_bg: Color::Rgb(31, 29, 46),
    input_fg: Color::Rgb(224, 222, 244),
    input_bg: Color::Rgb(42, 39, 63),
    status_fg: Color::Rgb(224, 222, 244),
    status_bg: Color::Rgb(25, 23, 36),
    table_header_focused: Style::new()
        .fg(Color::Rgb(156, 207, 216))
        .add_modifier(Modifier::BOLD),
    table_header_unfocused: Style::new().fg(Color::Rgb(110, 106, 134)),
    table_row_alt_bg: Color::Rgb(42, 39, 63),
    table_row_num: Style::new().fg(Color::Rgb(110, 106, 134)),
    null_value: Style::new()
        .fg(Color::Rgb(110, 106, 134))
        .add_modifier(Modifier::ITALIC),
    sql_focused: Style::new()
        .fg(Color::Rgb(62, 143, 176))
        .add_modifier(Modifier::BOLD),
    sql_unfocused: Style::new().add_modifier(Modifier::BOLD),
    error: Style::new().fg(Color::Rgb(235, 111, 146)),
    summary: Style::new().fg(Color::Rgb(62, 143, 176)),
    vertical_col: Style::new()
        .fg(Color::Rgb(196, 167, 231))
        .add_modifier(Modifier::BOLD),
    completion_kw: Color::Rgb(156, 207, 216),
    completion_table: Color::Rgb(246, 193, 119),
    completion_column: Color::Rgb(196, 167, 231),
    completion_fn: Color::Rgb(62, 143, 176),
    completion_command: Color::Rgb(224, 222, 244),
    completion_selected: Style::new()
        .bg(Color::Rgb(42, 39, 63))
        .fg(Color::Rgb(224, 222, 244)),
    completion_border: Style::new().fg(Color::Rgb(235, 111, 146)),
    syntax_keyword: Style::new()
        .fg(Color::Rgb(235, 111, 146))
        .add_modifier(Modifier::BOLD),
    syntax_number: Style::new().fg(Color::Rgb(246, 193, 119)),
    syntax_string: Style::new().fg(Color::Rgb(62, 143, 176)),
    syntax_operator: Style::new().fg(Color::Rgb(110, 106, 134)),
    picker_selected: Style::new()
        .fg(Color::Rgb(156, 207, 216))
        .add_modifier(Modifier::BOLD),
    header_style: Style::new()
        .bg(Color::Rgb(31, 29, 46))
        .fg(Color::Rgb(224, 222, 244))
        .add_modifier(Modifier::BOLD),
    schema_browser_border: Style::new().fg(Color::Rgb(156, 207, 216)),
    tab_active: Style::new()
        .fg(Color::Rgb(156, 207, 216))
        .add_modifier(Modifier::BOLD),
    tab_inactive: Style::new()
        .fg(Color::Rgb(156, 207, 216))
        .add_modifier(Modifier::DIM),
    tab_border: Style::new().fg(Color::Rgb(110, 106, 134)),
    scrollbar_thumb: Style::new().fg(Color::Rgb(224, 222, 244)),
    scrollbar_track: Style::new().dim(),
    border_primary: Style::new().fg(Color::Rgb(235, 111, 146)),
    border_secondary: Style::new().fg(Color::Rgb(65, 60, 90)),
    command_palette_selected: Style::new()
        .bg(Color::Rgb(42, 39, 63))
        .fg(Color::Rgb(224, 222, 244)),
    help_key: Style::new()
        .fg(Color::Rgb(156, 207, 216))
        .add_modifier(Modifier::BOLD),
    help_desc: Style::new(),
    help_section: Style::new()
        .fg(Color::Rgb(156, 207, 216))
        .add_modifier(Modifier::BOLD),

    top_bar_bg: Color::Rgb(42, 39, 63),
    top_bar_active: Style::new()
        .fg(Color::Rgb(224, 222, 244))
        .bg(Color::Rgb(65, 60, 90))
        .add_modifier(Modifier::BOLD),
    top_bar_inactive: Style::new()
        .fg(Color::Rgb(156, 207, 216))
        .bg(Color::Rgb(42, 39, 63)),
    bg: Color::Rgb(31, 29, 46),
    bottom_bar_bg: Color::Rgb(25, 23, 36),
    bottom_bar_fg: Color::Rgb(224, 222, 244),
});

// ── Everforest ──
pub static EVERFOREST: LazyLock<Theme> = LazyLock::new(|| Theme {
    name: "everforest",
    header_fg: Color::Rgb(211, 198, 170),
    header_bg: Color::Rgb(45, 53, 59),
    input_fg: Color::Rgb(211, 198, 170),
    input_bg: Color::Rgb(35, 42, 47),
    status_fg: Color::Rgb(211, 198, 170),
    status_bg: Color::Rgb(31, 37, 42),
    table_header_focused: Style::new()
        .fg(Color::Rgb(167, 192, 128))
        .add_modifier(Modifier::BOLD),
    table_header_unfocused: Style::new().fg(Color::Rgb(125, 134, 118)),
    table_row_alt_bg: Color::Rgb(56, 64, 70),
    table_row_num: Style::new().fg(Color::Rgb(125, 134, 118)),
    null_value: Style::new()
        .fg(Color::Rgb(125, 134, 118))
        .add_modifier(Modifier::ITALIC),
    sql_focused: Style::new()
        .fg(Color::Rgb(167, 192, 128))
        .add_modifier(Modifier::BOLD),
    sql_unfocused: Style::new().add_modifier(Modifier::BOLD),
    error: Style::new().fg(Color::Rgb(230, 126, 128)),
    summary: Style::new().fg(Color::Rgb(167, 192, 128)),
    vertical_col: Style::new()
        .fg(Color::Rgb(219, 188, 127))
        .add_modifier(Modifier::BOLD),
    completion_kw: Color::Rgb(131, 192, 178),
    completion_table: Color::Rgb(219, 188, 127),
    completion_column: Color::Rgb(182, 148, 170),
    completion_fn: Color::Rgb(167, 192, 128),
    completion_command: Color::Rgb(211, 198, 170),
    completion_selected: Style::new()
        .bg(Color::Rgb(56, 64, 70))
        .fg(Color::Rgb(211, 198, 170)),
    completion_border: Style::new().fg(Color::Rgb(131, 192, 178)),
    syntax_keyword: Style::new()
        .fg(Color::Rgb(131, 192, 178))
        .add_modifier(Modifier::BOLD),
    syntax_number: Style::new().fg(Color::Rgb(219, 188, 127)),
    syntax_string: Style::new().fg(Color::Rgb(167, 192, 128)),
    syntax_operator: Style::new().fg(Color::Rgb(125, 134, 118)),
    picker_selected: Style::new()
        .fg(Color::Rgb(131, 192, 178))
        .add_modifier(Modifier::BOLD),
    header_style: Style::new()
        .bg(Color::Rgb(45, 53, 59))
        .fg(Color::Rgb(211, 198, 170))
        .add_modifier(Modifier::BOLD),
    schema_browser_border: Style::new().fg(Color::Rgb(131, 192, 178)),
    tab_active: Style::new()
        .fg(Color::Rgb(131, 192, 178))
        .add_modifier(Modifier::BOLD),
    tab_inactive: Style::new()
        .fg(Color::Rgb(131, 192, 178))
        .add_modifier(Modifier::DIM),
    tab_border: Style::new().fg(Color::Rgb(125, 134, 118)),
    scrollbar_thumb: Style::new().fg(Color::Rgb(211, 198, 170)),
    scrollbar_track: Style::new().dim(),
    border_primary: Style::new().fg(Color::Rgb(131, 192, 178)),
    border_secondary: Style::new().fg(Color::Rgb(65, 76, 72)),
    command_palette_selected: Style::new()
        .bg(Color::Rgb(56, 64, 70))
        .fg(Color::Rgb(211, 198, 170)),
    help_key: Style::new()
        .fg(Color::Rgb(131, 192, 178))
        .add_modifier(Modifier::BOLD),
    help_desc: Style::new(),
    help_section: Style::new()
        .fg(Color::Rgb(131, 192, 178))
        .add_modifier(Modifier::BOLD),

    top_bar_bg: Color::Rgb(55, 63, 69),
    top_bar_active: Style::new()
        .fg(Color::Rgb(211, 198, 170))
        .bg(Color::Rgb(75, 86, 82))
        .add_modifier(Modifier::BOLD),
    top_bar_inactive: Style::new()
        .fg(Color::Rgb(131, 192, 178))
        .bg(Color::Rgb(55, 63, 69)),
    bg: Color::Rgb(45, 53, 59),
    bottom_bar_bg: Color::Rgb(31, 37, 42),
    bottom_bar_fg: Color::Rgb(211, 198, 170),
});

// ── Ayu Dark ──
pub static AYU_DARK: LazyLock<Theme> = LazyLock::new(|| Theme {
    name: "ayu-dark",
    header_fg: Color::Rgb(191, 199, 213),
    header_bg: Color::Rgb(14, 18, 24),
    input_fg: Color::Rgb(191, 199, 213),
    input_bg: Color::Rgb(22, 27, 36),
    status_fg: Color::Rgb(191, 199, 213),
    status_bg: Color::Rgb(10, 13, 18),
    table_header_focused: Style::new()
        .fg(Color::Rgb(57, 186, 230))
        .add_modifier(Modifier::BOLD),
    table_header_unfocused: Style::new().fg(Color::Rgb(96, 105, 120)),
    table_row_alt_bg: Color::Rgb(28, 34, 44),
    table_row_num: Style::new().fg(Color::Rgb(96, 105, 120)),
    null_value: Style::new()
        .fg(Color::Rgb(96, 105, 120))
        .add_modifier(Modifier::ITALIC),
    sql_focused: Style::new()
        .fg(Color::Rgb(185, 211, 114))
        .add_modifier(Modifier::BOLD),
    sql_unfocused: Style::new().add_modifier(Modifier::BOLD),
    error: Style::new().fg(Color::Rgb(255, 97, 97)),
    summary: Style::new().fg(Color::Rgb(185, 211, 114)),
    vertical_col: Style::new()
        .fg(Color::Rgb(162, 122, 204))
        .add_modifier(Modifier::BOLD),
    completion_kw: Color::Rgb(57, 186, 230),
    completion_table: Color::Rgb(255, 143, 64),
    completion_column: Color::Rgb(162, 122, 204),
    completion_fn: Color::Rgb(185, 211, 114),
    completion_command: Color::Rgb(191, 199, 213),
    completion_selected: Style::new()
        .bg(Color::Rgb(28, 34, 44))
        .fg(Color::Rgb(191, 199, 213)),
    completion_border: Style::new().fg(Color::Rgb(57, 186, 230)),
    syntax_keyword: Style::new()
        .fg(Color::Rgb(255, 143, 64))
        .add_modifier(Modifier::BOLD),
    syntax_number: Style::new().fg(Color::Rgb(162, 122, 204)),
    syntax_string: Style::new().fg(Color::Rgb(185, 211, 114)),
    syntax_operator: Style::new().fg(Color::Rgb(96, 105, 120)),
    picker_selected: Style::new()
        .fg(Color::Rgb(57, 186, 230))
        .add_modifier(Modifier::BOLD),
    header_style: Style::new()
        .bg(Color::Rgb(14, 18, 24))
        .fg(Color::Rgb(191, 199, 213))
        .add_modifier(Modifier::BOLD),
    schema_browser_border: Style::new().fg(Color::Rgb(57, 186, 230)),
    tab_active: Style::new()
        .fg(Color::Rgb(57, 186, 230))
        .add_modifier(Modifier::BOLD),
    tab_inactive: Style::new()
        .fg(Color::Rgb(57, 186, 230))
        .add_modifier(Modifier::DIM),
    tab_border: Style::new().fg(Color::Rgb(96, 105, 120)),
    scrollbar_thumb: Style::new().fg(Color::Rgb(191, 199, 213)),
    scrollbar_track: Style::new().dim(),
    border_primary: Style::new().fg(Color::Rgb(57, 186, 230)),
    border_secondary: Style::new().fg(Color::Rgb(45, 53, 65)),
    command_palette_selected: Style::new()
        .bg(Color::Rgb(28, 34, 44))
        .fg(Color::Rgb(191, 199, 213)),
    help_key: Style::new()
        .fg(Color::Rgb(57, 186, 230))
        .add_modifier(Modifier::BOLD),
    help_desc: Style::new(),
    help_section: Style::new()
        .fg(Color::Rgb(57, 186, 230))
        .add_modifier(Modifier::BOLD),

    top_bar_bg: Color::Rgb(22, 27, 36),
    top_bar_active: Style::new()
        .fg(Color::Rgb(191, 199, 213))
        .bg(Color::Rgb(45, 53, 65))
        .add_modifier(Modifier::BOLD),
    top_bar_inactive: Style::new()
        .fg(Color::Rgb(57, 186, 230))
        .bg(Color::Rgb(22, 27, 36)),
    bg: Color::Rgb(14, 18, 24),
    bottom_bar_bg: Color::Rgb(10, 13, 18),
    bottom_bar_fg: Color::Rgb(191, 199, 213),
});
