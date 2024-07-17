use level::Sprite;
use refs::Weak;
use ui::{AlertErr, TextField, ViewCallbacks, ViewData, ViewSetup};
use ui_proc::view;

use crate as test_engine;

#[view]
pub struct SpriteView {
    #[educe(Debug(ignore))]
    sprite: Weak<dyn Sprite>,

    #[init]
    position: TextField,
    rotation: TextField,
    z_pos:    TextField,
}

impl ViewSetup for SpriteView {
    fn setup(mut self: Weak<Self>) {
        self.place().all_ver();

        self.z_pos.editing_ended.val(move |z_text| {
            let Some(z) = z_text.parse::<f32>().alert_err() else {
                return;
            };
            self.sprite.z_position = z;
        });
    }
}

impl ViewCallbacks for SpriteView {
    fn update(&mut self) {
        let Some(sprite) = self.sprite.get() else {
            return;
        };

        self.position.set_text(sprite.position());
        self.rotation.set_text(sprite.rotation().to_string());

        if !self.z_pos.is_editing() {
            self.z_pos.set_text(sprite.z_position.to_string());
        }
    }
}

impl SpriteView {
    pub fn set_sprite(&mut self, sprite: Weak<dyn Sprite>) {
        self.sprite = sprite;
    }
}
