use syntect::easy::HighlightLines;
use syntect::highlighting::{FontStyle, Style, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

#[derive(Debug, PartialEq)]
pub struct CodeLine {
    pub parts: Vec<CodeToken>,
}

impl From<Vec<CodeToken>> for CodeLine {
    fn from(value: Vec<CodeToken>) -> Self {
        Self { parts: value }
    }
}

impl From<CodeToken> for CodeLine {
    fn from(value: CodeToken) -> Self {
        Self { parts: vec![value] }
    }
}

#[derive(Debug, PartialEq)]
pub struct CodeToken {
    pub source: String,
    pub fg: (u8, u8, u8),
    pub bold: bool,
}

impl<'a> From<(Style, &'a str)> for CodeToken {
    fn from((style, src): (Style, &'a str)) -> Self {
        Self {
            source: src.into(),
            fg: (style.foreground.r, style.foreground.g, style.foreground.b),
            bold: style.font_style.contains(FontStyle::BOLD),
        }
    }
}

impl From<String> for CodeToken {
    fn from(value: String) -> Self {
        Self {
            source: value,
            fg: (255, 255, 255),
            bold: false,
        }
    }
}

pub fn highlight_code(code: String, language: &str) -> Vec<CodeLine> {
    let ps = SyntaxSet::load_defaults_newlines();
    let theme = ThemeSet::get_theme("themes/ayu-mirage.stTheme").unwrap();

    let syntax = match ps.find_syntax_by_extension(language) {
        Some(syntax) => syntax,
        None => {
            return code
                .lines()
                .map(|l| format!("{l}\n"))
                .map(CodeToken::from)
                .map(CodeLine::from)
                .collect::<Vec<_>>()
        }
    };
    let mut h = HighlightLines::new(syntax, &theme);

    LinesWithEndings::from(&code)
        .map(|line| {
            h.highlight_line(line, &ps)
                .unwrap()
                .into_iter()
                .map(CodeToken::from)
                .collect::<Vec<_>>()
        })
        .map(CodeLine::from)
        .collect::<Vec<_>>()
}
