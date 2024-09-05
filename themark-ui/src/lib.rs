mod dashboard;
mod inner_token;
mod router;
mod viewer;

use themark_parser::Token;

use anathema::backend::tui::TuiBackend;
use anathema::runtime::Runtime;
use anathema::templates::Document;
use dashboard::{Dashboard, DashboardState};
use router::Router;
use viewer::{Viewer, ViewerState};

#[derive(Debug, PartialEq)]
pub struct MarkdownDocument {
    file_name: String,
    path: std::path::PathBuf,
    size: String,
}

impl MarkdownDocument {
    pub fn new(file_name: String, path: std::path::PathBuf, size: String) -> Self {
        Self {
            file_name,
            path,
            size,
        }
    }
}

impl From<std::fs::DirEntry> for MarkdownDocument {
    fn from(value: std::fs::DirEntry) -> Self {
        Self {
            file_name: value.file_name().to_string_lossy().to_string(),
            path: value.path(),
            size: themark_fs::read_file_size(value),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Entrypoint {
    Dashboard(Vec<MarkdownDocument>),
    Viewer(Vec<Token>),
}

impl std::fmt::Display for Entrypoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Entrypoint::Dashboard(_) => f.write_str("dashboard"),
            Entrypoint::Viewer(_) => f.write_str("viewer"),
        }
    }
}

pub fn setup(entrypoint: Entrypoint) -> Result<(), Box<dyn std::error::Error>> {
    let doc = Document::new("@router");

    let backend = TuiBackend::builder()
        .enable_alt_screen()
        .enable_mouse()
        .hide_cursor()
        .enable_raw_mode()
        .finish()
        .unwrap();

    let mut runtime = Runtime::builder(doc, backend);

    let components = [
        ("dashboard_item", "templates/components/dashboard_item.aml"),
        ("loading_doc", "templates/components/loading_doc.aml"),
        ("error_doc", "templates/components/error_doc.aml"),
        ("link", "templates/components/link.aml"),
        ("heading", "templates/components/heading.aml"),
        ("code_block", "templates/components/code_block.aml"),
        ("image", "templates/components/image.aml"),
        ("table", "templates/components/table.aml"),
        ("paragraph", "templates/components/paragraph.aml"),
        ("list", "templates/components/list.aml"),
    ];

    for (name, path) in components {
        runtime.register_prototype(name, path, || (), || ())?;
    }

    let start_page = entrypoint.to_string();

    match entrypoint {
        Entrypoint::Dashboard(documents) => {
            let viewer_id = runtime.register_default::<Viewer>("viewer", "templates/viewer.aml")?;
            runtime.register_component(
                "dashboard",
                "templates/dashboard.aml",
                Dashboard::new(viewer_id),
                DashboardState::new(documents.into_iter().map(Into::into)),
            )?;
        }
        Entrypoint::Viewer(tokens) => {
            let viewer_id = runtime.register_component(
                "viewer",
                "templates/viewer.aml",
                Viewer,
                ViewerState::new(tokens.into_iter().map(Into::into)),
            )?;
            runtime.register_component(
                "dashboard",
                "templates/dashboard.aml",
                Dashboard::new(viewer_id),
                DashboardState::default(),
            )?;
        }
    }

    Router::builder()
        .add_route("dashboard")
        .add_route("viewer")
        .finish(start_page, &mut runtime)?;

    runtime.finish()?.run();

    Ok(())
}
