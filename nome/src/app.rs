use test_engine::{
    App,
    refs::Own,
    ui::{Setup, Size, View},
};

use crate::interface::nome_view::NomeView;

#[derive(Default)]
pub struct NomeApp;

impl App for NomeApp {
    fn make_root_view(&self) -> Own<dyn View> {
        NomeView::new()
    }

    /// Physical pixels, so this is the portrait shape the screen was designed
    /// at once a retina display halves it.
    fn initial_size(&self) -> Size {
        (960, 1560).into()
    }
}
