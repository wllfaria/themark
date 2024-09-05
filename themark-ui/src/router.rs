use std::fmt::Display;

use anathema::backend::Backend;
use anathema::component::Component;
use anathema::runtime::{Error, RuntimeBuilder};
use anathema::state::{List, State, Value};
use anathema::templates::ToSourceKind;

static RECEIVE_IDENT: &str = "navigate";

pub struct RouterBuilder {
    routes: Vec<String>,
    path: Option<String>,
}

impl RouterBuilder {
    pub fn add_route(mut self, route: &str) -> RouterBuilder {
        self.routes.push(route.to_string());
        self
    }

    pub fn generate_template(&self) -> String {
        let mut template = String::new();
        for route in &self.routes {
            let component = format!(
                r#"
if active_route == "{route}"
    @{route} ({RECEIVE_IDENT}->{RECEIVE_IDENT})
            "#
            );
            template.push_str(&component);
        }

        template
    }

    pub fn finish<E: Display, T: Backend>(
        self,
        entrypoint: E,
        runtime: &mut RuntimeBuilder<T>,
    ) -> Result<(), Error> {
        let template = self.generate_template();

        let router = Router {};
        let router_state = RouterState {
            routes: List::from_iter(self.routes),
            active_route: entrypoint.to_string().into(),
        };

        if self.path.is_some() {
            std::fs::write(self.path.as_ref().unwrap(), &template).unwrap();
            runtime.register_component("router", self.path.unwrap(), router, router_state)?;
        } else {
            runtime.register_component("router", template.to_template(), router, router_state)?;
        }
        Ok(())
    }
}

pub struct Router;

impl Router {
    pub fn builder() -> RouterBuilder {
        RouterBuilder {
            routes: vec![],
            path: None,
        }
    }
}

#[derive(State)]
pub struct RouterState {
    active_route: Value<String>,
    routes: Value<List<String>>,
}

impl Component for Router {
    type Message = ();
    type State = RouterState;

    fn receive(
        &mut self,
        ident: &str,
        value: anathema::state::CommonVal<'_>,
        state: &mut Self::State,
        _: anathema::widgets::Elements<'_, '_>,
        _: anathema::prelude::Context<'_, Self::State>,
    ) {
        if ident != RECEIVE_IDENT {
            return;
        }

        let route = value.to_string();
        state.active_route.set(route);
    }
}
