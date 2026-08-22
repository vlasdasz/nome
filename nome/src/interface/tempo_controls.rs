use hilen::{
    Event,
    gm::Apply,
    refs::Weak,
    ui::{Button, Setup, UIColor, ViewData, view},
};

const STEPS: [i16; 6] = [-20, -5, -1, 1, 5, 20];

/// The six step buttons, shared by every variant so they all offer exactly the
/// same control and only the look differs.
#[view]
pub struct TempoControls {
    pub changed: Event<i16>,

    #[init]
    m_20: Button,
    m_5:  Button,
    m_1:  Button,
    p_1:  Button,
    p_5:  Button,
    p_20: Button,
}

impl TempoControls {
    pub fn buttons(self: Weak<Self>) -> [Weak<Button>; 6] {
        [self.m_20, self.m_5, self.m_1, self.p_1, self.p_5, self.p_20]
    }

    pub fn set_style(
        self: Weak<Self>,
        color: impl Into<UIColor> + Copy,
        text_color: impl Into<UIColor> + Copy,
        radius: f32,
    ) -> Weak<Self> {
        self.buttons().apply(move |button| {
            button.set_color(color).set_text_color(text_color).set_corner_radius(radius);
        });
        self
    }
}

impl Setup for TempoControls {
    fn setup(self: Weak<Self>) {
        self.place().all_hor().all(6);

        for (button, step) in self.buttons().into_iter().zip(STEPS) {
            button.set_text(format!("{step:+}")).set_text_size(18);
            button.on_tap(move || self.changed.trigger(step));
        }
    }
}
