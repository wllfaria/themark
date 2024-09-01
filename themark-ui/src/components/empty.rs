use anathema::component::Component;

#[derive(Default)]
pub struct Empty;

impl Component for Empty {
    type Message = ();
    type State = ();
}
