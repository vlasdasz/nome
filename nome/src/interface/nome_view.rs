use mnomer::{
    frequency_relative_semitone_equal_temperament, BeatPattern, BeatPatternType, BeatPlayer,
    ToneConfiguration,
};
use test_engine::{
    refs::{MainLock, Weak},
    ui::{view, Button, HasText, ViewData, ViewSetup},
};

static PLAYER: MainLock<Option<BeatPlayer>> = MainLock::new();

fn player() -> &'static mut BeatPlayer {
    PLAYER.get_mut().as_mut().unwrap()
}

#[view]
pub struct NomeView {
    #[init]
    start_button: Button,
}

impl NomeView {
    #[allow(clippy::unused_self)]
    fn on_start(&mut self) {
        let player = player();
        if player.is_playing() {
            player.stop();
        } else {
            player.play_beat().unwrap();
        }
    }
}

impl ViewSetup for NomeView {
    fn setup(mut self: Weak<Self>) {
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
        let beatplayer = BeatPlayer::new(
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

        PLAYER.get_mut().replace(beatplayer);

        self.start_button.set_text("Start").place().size(200, 200).center();
        self.start_button.on_tap(move || self.on_start());
    }
}
