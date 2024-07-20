use test_engine::{
    gm::Apply,
    refs::Weak,
    ui::{view, Button, HasText, ViewData, ViewSetup},
    Event,
};

#[view]
#[derive(Default)]
pub struct TempoControl {
    pub changed: Event<i16>,

    #[init]
    m_20: Button,
    m_5:  Button,
    m_1:  Button,
    p_1:  Button,
    p_5:  Button,
    p_20: Button,
}

impl ViewSetup for TempoControl {
    fn setup(mut self: Weak<Self>) {
        self.place().all_hor();

        self.m_20.set_text("-20");
        self.m_5.set_text("-5");
        self.m_1.set_text("-1");
        self.p_1.set_text("+1");
        self.p_5.set_text("+5");
        self.p_20.set_text("+20");

        [self.m_20, self.m_5, self.m_1, self.p_1, self.p_5, self.p_20].apply(|mut button| {
            button.set_text_size(60);
            button.on_tap(move || self.changed.trigger(button.text().parse().unwrap()));
        });
    }
}
