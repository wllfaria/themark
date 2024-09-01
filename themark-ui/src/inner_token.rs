use anathema::state::{CommonVal, List, Path, State, Subscriber, Value, ValueRef};

use themark_parser::Token;

#[derive(Debug)]
pub enum InnerToken {
    Text(Value<String>),
    Paragraph {
        parts: Value<List<InnerToken>>,
    },
    Code(Value<String>),
    CodeBlock {
        content: Value<String>,
        language: Value<String>,
    },
    Heading {
        level: Value<u8>,
        content: Value<String>,
    },
    List {
        items: Value<List<InnerToken>>,
    },
    Link {
        uri: Value<String>,
        label: Value<String>,
    },
    ListItem {
        parts: Value<List<InnerToken>>,
    },
}

impl From<Token> for InnerToken {
    fn from(value: Token) -> Self {
        match value {
            Token::Paragraph { parts } => InnerToken::Paragraph {
                parts: List::from_iter(parts.into_iter().map(InnerToken::from)),
            },
            Token::Text(t) => InnerToken::Text(Value::from(t)),
            Token::Heading { level, content } => InnerToken::Heading {
                level: Value::from(u8::from(level)),
                content: Value::from(content),
            },
            Token::List { items } => InnerToken::List {
                items: List::from_iter(items.into_iter().map(InnerToken::from)),
            },
            Token::Code(code) => InnerToken::Code(Value::from(code)),
            Token::Link { uri, label } => InnerToken::Link {
                uri: uri.into(),
                label: label.into(),
            },
            Token::ListItem { parts } => InnerToken::ListItem {
                parts: List::from_iter(parts.into_iter().map(InnerToken::from)),
            },
            Token::CodeBlock { content, language } => InnerToken::CodeBlock {
                content: content.into(),
                language: language.into(),
            },
        }
    }
}

impl State for InnerToken {
    fn state_get(&self, path: Path<'_>, sub: Subscriber) -> Option<ValueRef> {
        match (self, path) {
            (InnerToken::Text(v), _) => Some(v.value_ref(sub)),
            (InnerToken::Paragraph { parts }, Path::Key("parts")) => Some(parts.value_ref(sub)),
            (InnerToken::Code(v), _) => Some(v.value_ref(sub)),
            (InnerToken::Link { uri, .. }, Path::Key("uri")) => Some(uri.value_ref(sub)),
            (InnerToken::Link { label, .. }, Path::Key("label")) => Some(label.value_ref(sub)),
            (InnerToken::CodeBlock { language, .. }, Path::Key("language")) => {
                Some(language.value_ref(sub))
            }
            (InnerToken::CodeBlock { content, .. }, Path::Key("content")) => {
                Some(content.value_ref(sub))
            }
            (InnerToken::Heading { level, .. }, Path::Key("level")) => Some(level.value_ref(sub)),
            (InnerToken::Heading { content, .. }, Path::Key("content")) => {
                Some(content.value_ref(sub))
            }
            (InnerToken::List { items }, Path::Key("items")) => Some(items.value_ref(sub)),
            (InnerToken::ListItem { parts }, Path::Key("parts")) => Some(parts.value_ref(sub)),
            _ => None,
        }
    }

    fn to_common(&self) -> Option<CommonVal<'_>> {
        match self {
            InnerToken::Text(_) => Some(CommonVal::Str("text")),
            InnerToken::Paragraph { .. } => Some(CommonVal::Str("paragraph")),
            InnerToken::Heading { .. } => Some(CommonVal::Str("heading")),
            InnerToken::List { .. } => Some(CommonVal::Str("list")),
            InnerToken::Link { .. } => Some(CommonVal::Str("link")),
            InnerToken::Code { .. } => Some(CommonVal::Str("inline_code")),
            InnerToken::CodeBlock { .. } => Some(CommonVal::Str("code_block")),
            InnerToken::ListItem { .. } => Some(CommonVal::Str("list_item")),
        }
    }
}
