use anathema::component::{Component, KeyCode, KeyEvent, MouseEvent, MouseState};
use anathema::default_widgets::Overflow;
use anathema::state::{List, State, Value};
use themark_parser::Token;

use crate::inner_token::InnerToken;

#[derive(State, Debug)]
pub struct IndexState {
    tokens: Value<List<InnerToken>>,
}

impl IndexState {
    pub fn new(tokens: Vec<Token>) -> IndexState {
        IndexState {
            tokens: tokens.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Default)]
pub struct Index;

impl Component for Index {
    type Message = ();
    type State = IndexState;

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
        _state: &mut Self::State,
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
