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
        .enable_mouse()
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
            "code_block",
            "templates/components/code_block.aml",
            Empty::default,
            Default::default,
        )
        .unwrap();

    runtime
        .register_prototype(
            "image",
            "templates/components/image.aml",
            Empty::default,
            Default::default,
        )
        .unwrap();

    runtime
        .register_prototype(
            "table",
            "templates/components/table.aml",
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

    let mut runtime = runtime.finish().unwrap();
    runtime.fps = 60;
    runtime.run();
}
