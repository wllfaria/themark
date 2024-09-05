use anathema::component::{Component, KeyCode, KeyEvent, MouseEvent, MouseState};
use anathema::default_widgets::Overflow;
use anathema::state::{List, State, Value};

use crate::inner_token::InnerToken;

#[derive(State, Debug, Default)]
pub struct ViewerState {
    tokens: Value<List<InnerToken>>,
    loading_document: Value<bool>,
    total_tokens: Value<usize>,
    has_error: Value<bool>,
}

impl ViewerState {
    pub fn new(tokens: impl Iterator<Item = InnerToken>) -> Self {
        let tokens = tokens.collect::<Vec<_>>();
        Self {
            loading_document: Value::new(tokens.is_empty()),
            has_error: Value::new(false),
            total_tokens: Value::new(tokens.len()),
            tokens: List::from_iter(tokens),
        }
    }
}

#[derive(Default)]
pub struct Viewer;

impl Component for Viewer {
    type Message = String;
    type State = ViewerState;

    fn message(
        &mut self,
        message: Self::Message,
        state: &mut Self::State,
        _: anathema::widgets::Elements<'_, '_>,
        _: anathema::prelude::Context<'_, Self::State>,
    ) {
        state.loading_document.set(true);
        match themark_fs::load_markdown(message) {
            Ok(tokens) => {
                let tok: Vec<InnerToken> = tokens.into_iter().map(Into::into).collect::<Vec<_>>();
                state.total_tokens.set(tok.len());
                state.tokens = List::from_iter(tok);
                state.loading_document.set(false);
            }
            Err(_) => state.has_error.set(true),
        };
    }

    fn on_mouse(
        &mut self,
        mouse: MouseEvent,
        _: &mut Self::State,
        mut elements: anathema::widgets::Elements<'_, '_>,
        _: anathema::prelude::Context<'_, Self::State>,
    ) {
        elements.by_tag("overflow").first(|el, _| {
            let overflow = el.to::<Overflow>();
            match mouse.state {
                MouseState::ScrollUp => overflow.scroll_up_by(3),
                MouseState::ScrollDown => overflow.scroll_down_by(3),
                _ => {}
            }
        });
    }

    fn on_key(
        &mut self,
        key: anathema::component::KeyEvent,
        _: &mut Self::State,
        mut elements: anathema::widgets::Elements<'_, '_>,
        context: anathema::prelude::Context<'_, Self::State>,
    ) {
        let height = context.viewport.size().height;
        elements.by_tag("overflow").first(|el, _| {
            let overflow = el.to::<Overflow>();
            let KeyEvent { code, .. } = key;
            match code {
                KeyCode::Char('j') | KeyCode::Down => overflow.scroll_down(),
                KeyCode::Char('k') | KeyCode::Up => overflow.scroll_up(),
                KeyCode::PageUp => overflow.scroll_up_by(height.div_ceil(2) as i32),
                KeyCode::PageDown => overflow.scroll_down_by(height.div_ceil(2) as i32),
                _ => {}
            }
        });
    }
}
