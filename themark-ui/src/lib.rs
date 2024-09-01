mod components;
mod index;
mod inner_token;

use components::Empty;
use index::{Index, IndexState};

use themark_parser::Token;

use anathema::backend::tui::TuiBackend;
use anathema::runtime::Runtime;
use anathema::templates::Document;

pub fn setup(tokens: Vec<Token>) {
    let doc = Document::new("@index");

    let backend = TuiBackend::builder()
        .enable_alt_screen()
        .hide_cursor()
        .enable_raw_mode()
        .finish()
        .unwrap();

    let mut runtime = Runtime::builder(doc, backend);

    runtime
        .register_component(
            "index",
            "templates/index.aml",
            Index,
            IndexState::new(tokens),
        )
        .unwrap();

    runtime
        .register_prototype(
            "list",
            "templates/components/list.aml",
            Empty::default,
            Default::default,
        )
        .unwrap();

    runtime
        .register_prototype(
            "link",
            "templates/components/link.aml",
            Empty::default,
            Default::default,
        )
        .unwrap();

    runtime
        .register_prototype(
            "heading",
            "templates/components/heading.aml",
            Empty::default,
            Default::default,
        )
        .unwrap();

    runtime
        .register_prototype(
            "paragraph",
            "templates/components/paragraph.aml",
            Empty::default,
            Default::default,
        )
        .unwrap();

    runtime.finish().unwrap().run();
}