pub mod syntax;

use pulldown_cmark::{CodeBlockKind, Event, HeadingLevel, Options, Parser, Tag, TagEnd};

#[derive(Debug)]
pub enum Token {
    Text(String),
    Paragraph {
        parts: Vec<Token>,
    },
    Table {
        headings: Vec<String>,
        cols: Vec<Vec<String>>,
    },
    Image {
        uri: String,
        text: String,
    },
    Heading {
        level: InnerHeadingLevel,
        content: String,
    },
    List {
        items: Vec<Token>,
    },
    Link {
        uri: String,
        label: String,
    },
    CodeBlock {
        content: Vec<syntax::CodeLine>,
        language: String,
    },
    Code(String),
    ListItem {
        parts: Vec<Token>,
    },
}

#[derive(Debug)]
pub struct InnerHeadingLevel(u8);

impl From<HeadingLevel> for InnerHeadingLevel {
    fn from(value: HeadingLevel) -> Self {
        match value {
            HeadingLevel::H1 => InnerHeadingLevel(1),
            HeadingLevel::H2 => InnerHeadingLevel(2),
            HeadingLevel::H3 => InnerHeadingLevel(3),
            HeadingLevel::H4 => InnerHeadingLevel(4),
            HeadingLevel::H5 => InnerHeadingLevel(5),
            HeadingLevel::H6 => InnerHeadingLevel(6),
        }
    }
}

impl From<InnerHeadingLevel> for u8 {
    fn from(value: InnerHeadingLevel) -> Self {
        value.0
    }
}

fn take_heading<'a>(iter: &mut impl Iterator<Item = Event<'a>>, level: HeadingLevel) -> Token {
    let mut content = String::new();

    loop {
        match iter.next() {
            Some(Event::Text(t)) => content.push_str(&t),
            Some(Event::End(TagEnd::Heading(_))) => break,
            _ => {}
        }
    }

    Token::Heading {
        level: level.into(),
        content,
    }
}

fn take_image<'a>(iter: &mut impl Iterator<Item = Event<'a>>, event: Event<'a>) -> Token {
    let Event::Start(Tag::Image { dest_url, .. }) = event else {
        unreachable!();
    };
    let mut link_text = String::new();

    loop {
        let Some(next) = iter.next() else { break };

        match next {
            Event::Text(text) => link_text.push_str(&text),
            Event::End(TagEnd::Image) => break,
            _ => {}
        }
    }

    Token::Image {
        uri: dest_url.to_string(),
        text: link_text,
    }
}

fn take_table_headings<'a>(iter: &mut impl Iterator<Item = Event<'a>>) -> Vec<String> {
    assert!(matches!(iter.next(), Some(Event::Start(Tag::TableHead))));

    let mut headings = vec![];

    loop {
        match iter.next() {
            Some(Event::Text(text)) => headings.push(text.to_string()),
            Some(Event::End(TagEnd::TableHead)) => break,
            _ => (),
        }
    }

    headings
}

fn take_table_row<'a>(iter: &mut impl Iterator<Item = Event<'a>>, columns: &mut [Vec<String>]) {
    let mut idx = 0;
    let mut column = String::new();
    loop {
        let next = iter.next();
        match next {
            Some(Event::Text(text)) => {
                column.push_str(text.as_ref());
            }
            Some(Event::Code(code)) => {
                column.push_str(code.as_ref());
            }
            Some(Event::End(TagEnd::TableCell)) => {
                columns[idx].push(column.clone());
                column = String::new();
                idx += 1;
            }
            Some(Event::End(TagEnd::TableRow)) => {
                break;
            }
            _ => (),
        }
    }
}

fn take_table<'a>(iter: &mut impl Iterator<Item = Event<'a>>) -> Token {
    let headings = take_table_headings(iter);
    let mut cols = (0..headings.len())
        .map(|_| vec![])
        .collect::<Vec<Vec<String>>>();

    loop {
        match iter.next() {
            Some(Event::Start(Tag::TableRow)) => take_table_row(iter, &mut cols),
            Some(Event::End(TagEnd::Table)) => break,
            _ => (),
        }
    }

    Token::Table { headings, cols }
}

fn take_paragraph<'a>(iter: &mut impl Iterator<Item = Event<'a>>) -> Token {
    let mut parts = vec![];

    loop {
        let Some(next) = iter.next() else { break };

        match next {
            Event::Text(text) => parts.push(Token::Text(text.to_string())),
            Event::Code(code) => parts.push(Token::Code(code.to_string())),
            Event::Start(Tag::Link { .. }) => parts.push(take_link(iter, next)),
            Event::Start(Tag::Image { .. }) => parts.push(take_image(iter, next)),
            Event::End(TagEnd::Paragraph) => break,
            _ => {}
        }
    }

    Token::Paragraph { parts }
}

fn take_link<'a>(iter: &mut impl Iterator<Item = Event<'a>>, link: Event<'a>) -> Token {
    let Event::Start(Tag::Link { dest_url, .. }) = link else {
        unreachable!();
    };

    let mut link_text = String::new();

    loop {
        let Some(next) = iter.next() else { break };

        match next {
            Event::Text(text) => link_text.push_str(&text),
            Event::End(TagEnd::Link) => break,
            _ => {}
        }
    }

    Token::Link {
        uri: dest_url.to_string(),
        label: link_text,
    }
}

fn take_list<'a>(iter: &mut impl Iterator<Item = Event<'a>>) -> Token {
    let mut items = vec![];

    while let Some(Event::Start(Tag::Item)) = iter.next() {
        items.push(take_list_items(iter));
    }

    Token::List { items }
}

fn take_list_items<'a>(iter: &mut impl Iterator<Item = Event<'a>>) -> Token {
    let mut parts = vec![];

    loop {
        let Some(next) = iter.next() else { break };

        match next {
            Event::Text(text) => parts.push(Token::Text(text.to_string())),
            Event::Code(code) => parts.push(Token::Code(code.to_string())),
            Event::Start(Tag::Link { .. }) => parts.push(take_link(iter, next)),
            Event::Start(Tag::List(_)) => parts.push(take_list(iter)),
            Event::End(TagEnd::Item) => break,
            _ => (),
        }
    }

    Token::ListItem { parts }
}

fn take_code_block<'a>(iter: &mut impl Iterator<Item = Event<'a>>, kind: CodeBlockKind) -> Token {
    let mut content = String::new();

    let language = match kind {
        CodeBlockKind::Indented => String::new(),
        CodeBlockKind::Fenced(lang) => lang.to_string(),
    };

    loop {
        let Some(next) = iter.next() else {
            break;
        };

        match next {
            Event::Text(text) => content.push_str(&text),
            Event::End(TagEnd::CodeBlock) => break,
            _ => {}
        }
    }

    let content = syntax::highlight_code(content, &language);

    Token::CodeBlock { content, language }
}

pub fn parse(input: &str) -> Vec<Token> {
    let parser = Parser::new_ext(input, Options::ENABLE_TABLES);
    let mut iter = parser;
    let mut tokens = vec![];

    loop {
        let Some(event) = iter.next() else {
            break;
        };

        match event {
            Event::Start(Tag::Heading { level, .. }) => tokens.push(take_heading(&mut iter, level)),
            Event::Start(Tag::List(_)) => tokens.push(take_list(&mut iter)),
            Event::Start(Tag::Table(_)) => tokens.push(take_table(&mut iter)),
            Event::Start(Tag::CodeBlock(kind)) => tokens.push(take_code_block(&mut iter, kind)),
            Event::Start(Tag::Paragraph) => tokens.push(take_paragraph(&mut iter)),
            _ => {}
        }
    }

    tokens
}
