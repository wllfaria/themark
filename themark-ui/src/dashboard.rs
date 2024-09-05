use crate::MarkdownDocument;
use anathema::component::{Component, ComponentId, KeyCode, KeyEvent};
use anathema::default_widgets::Overflow;
use anathema::state::{List, State, Value};

#[derive(Debug, State)]
pub struct MarkdownDocumentState {
    file_name: Value<String>,
    path: Value<String>,
    size: Value<String>,
}

impl From<MarkdownDocument> for MarkdownDocumentState {
    fn from(document: MarkdownDocument) -> Self {
        Self {
            file_name: document.file_name.into(),
            path: document.path.to_string_lossy().to_string().into(),
            size: document.size.into(),
        }
    }
}

#[derive(Debug, State)]
pub struct DashboardState {
    documents: Value<List<MarkdownDocumentState>>,
    selected_idx: Value<usize>,
    navigate_to: Value<String>,
}

impl Default for DashboardState {
    fn default() -> Self {
        Self {
            documents: Default::default(),
            selected_idx: Value::new(0),
            navigate_to: Value::new("viewer".into()),
        }
    }
}

impl DashboardState {
    pub fn new(documents: impl Iterator<Item = MarkdownDocumentState>) -> Self {
        Self {
            documents: List::from_iter(documents),
            selected_idx: Value::new(0),
            navigate_to: Value::new("viewer".into()),
        }
    }
}

#[derive(Debug)]
pub struct Dashboard {
    viewer: ComponentId<String>,
}

impl Dashboard {
    pub fn new(viewer_id: ComponentId<String>) -> Self {
        Self { viewer: viewer_id }
    }
}

impl Component for Dashboard {
    type Message = ();
    type State = DashboardState;

    fn on_key(
        &mut self,
        key: anathema::component::KeyEvent,
        state: &mut Self::State,
        mut elements: anathema::widgets::Elements<'_, '_>,
        mut context: anathema::prelude::Context<'_, Self::State>,
    ) {
        let overflow_offset = 8;

        let KeyEvent { code, .. } = key;
        let selected_idx = state.selected_idx.copy_value();
        let max_idx = state.documents.len() - 1;
        match code {
            KeyCode::Char('j') | KeyCode::Down => state
                .selected_idx
                .set(usize::min(selected_idx + 1, max_idx)),
            KeyCode::Char('k') | KeyCode::Up => {
                state.selected_idx.set(selected_idx.saturating_sub(1))
            }
            KeyCode::Enter | KeyCode::Char(' ') => {
                if let Some(val) = state
                    .documents
                    .to_ref()
                    .iter()
                    .nth(state.selected_idx.copy_value())
                {
                    let content = val.to_ref().path.to_ref().clone();
                    context.emit(self.viewer, content);
                    context.publish("navigate", |state| &state.navigate_to);
                }
            }
            _ => {}
        }

        elements.by_attribute("id", "scrollview").first(|el, _| {
            let overflow = el.to::<Overflow>();
            let height = context.viewport.size().height;
            let height = height - overflow_offset;
            let offset = state.selected_idx.copy_value();
            let offset = offset as i32 - overflow.offset().y;

            if offset > height as i32 {
                let difference = offset - height as i32;
                overflow.scroll_down_by(difference);
            } else if offset <= 0 {
                let difference = i32::abs(offset);
                overflow.scroll_up_by(difference);
            }
        });
    }
}
