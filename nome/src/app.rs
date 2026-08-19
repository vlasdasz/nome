use test_engine::{
    App,
    refs::Own,
    ui::{Setup, View},
};

use crate::interface::nome_view::NomeView;

#[derive(Default)]
pub struct NomeApp;

impl App for NomeApp {
    fn make_root_view(&self) -> Own<dyn View> {
        NomeView::new()
    }
}
