use mnomer::{
    BeatPattern, BeatPatternType, BeatPlayer, ToneConfiguration,
    frequency_relative_semitone_equal_temperament,
};
use test_engine::{
    refs::Weak,
    ui::{Button, Label, Setup, ViewData, view},
};

use crate::interface::tempo_control::TempoControl;

#[view]
pub struct NomeView {
    #[educe(Default = make_player())]
    player: BeatPlayer,

    #[init]
    tempo_control: TempoControl,
    tempo_label:   Label,
    start_button:  Button,
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

impl Setup for NomeView {
    fn setup(mut self: Weak<Self>) {
        self.tempo_control.place().l(10).t(120).r(10).h(100);
        self.tempo_control.changed.val(move |bpm| {
            let mut current: i16 = self.tempo_label.text().parse().unwrap();
            current += bpm;
            if current < 0 {
                current = 0;
            }
            self.player.set_bpm(current.try_into().unwrap());
            self.tempo_label.set_text(current);
        });

        self.tempo_label
            .set_text_size(80)
            .set_text("100")
            .place()
            .size(400, 200)
            .center();

        self.start_button.set_text_size(64).set_text("Start").place().lrb(10).h(200);
        self.start_button.on_tap(move || self.on_start());
    }
}

fn make_player() -> BeatPlayer {
    let freq = 440.0;
    let normal_beat = ToneConfiguration {
        frequency:   freq,
        sample_rate: 48000.0,
        length:      0.05,
        overtones:   1,
        channels:    1,
    };

    // The accent is five semitones above the normal beat.
    let accentuated_beat = ToneConfiguration {
        frequency: frequency_relative_semitone_equal_temperament(freq, 5.0),
        ..normal_beat
    };

    BeatPlayer::new(
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
    )
}
