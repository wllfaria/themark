use anathema::component::Component;
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
}
