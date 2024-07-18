use mnomer::{
    frequency_relative_semitone_equal_temperament, BeatPattern, BeatPatternType, BeatPlayer,
    ToneConfiguration,
};
use test_engine::{
    refs::Weak,
    ui::{view, Anchor::Bot, Button, HasText, TextField, ViewBase, ViewData, ViewSetup},
};

#[view]
pub struct NomeView {
    player: BeatPlayer,

    #[init]
    bmp_field:    TextField,
    start_button: Button,
}

impl Default for NomeView {
    fn default() -> Self {
        // Create the tone configurations for the beatplayer
        let freq = 440.0;
        let normal_beat = ToneConfiguration {
            frequency:   freq,
            sample_rate: 48000.0, // may be changed by the beatplayer to match the audio device
            length:      0.05,    // 50 ms
            overtones:   1,
            channels:    1,
        };

        // accentuated beat is 5 semitones higher than the normal beat
        let accentuated_beat = ToneConfiguration {
            frequency: frequency_relative_semitone_equal_temperament(freq, 5.0),
            ..normal_beat
        };

        // beatplayer takes care of generating the beat and its playback
        let player = BeatPlayer::new(
            100,
            4,
            normal_beat,
            accentuated_beat,
            BeatPattern(vec![
                BeatPatternType::Accent,
                BeatPatternType::Beat,
                BeatPatternType::Beat,
                BeatPatternType::Beat,
            ]),
        );

        Self {
            __view_base: ViewBase::default(),
            player,
            bmp_field: Weak::default(),
            start_button: Weak::default(),
        }
    }
}

impl NomeView {
    fn on_start(&mut self) {
        if self.player.is_playing() {
            self.start_button.set_text("Start");
            self.player.stop();
        } else {
            self.start_button.set_text("Stop");
            self.player.play_beat().unwrap();
        }
    }
}

impl ViewSetup for NomeView {
    fn setup(mut self: Weak<Self>) {
        self.start_button.set_text("Start").place().size(200, 200).center();
        self.start_button.on_tap(move || self.on_start());

        self.bmp_field.set_text("100");
        self.bmp_field
            .place()
            .size(400, 100)
            .center_x()
            .anchor(Bot, self.start_button, 10);

        self.bmp_field.editing_ended.val(move |bmp| {
            let bmp: u16 = bmp.parse().unwrap();
            self.player.set_bpm(bmp);
        });
    }
}
