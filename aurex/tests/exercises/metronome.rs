use rand::Rng;

use aurex::metronome::metronomes::{TickEveryBeatMetronome, TwoAndFourMetronome};
use aurex::{
    drums::metronome_emphasis,
    exercises::{metronome::MetronomeExercise, play},
    midi,
};

/// Clicks on 2 and 4 at a random tempo between 55 and 68 BPM
#[test]
fn warmup_metronome() {
    let mut rng = rand::rng();
    let bpm: f64 = rng.random_range(55f64..=68f64);

    println!("Two and Four metronome at ~{bpm:.0}BPM");

    let exercise = MetronomeExercise {
        metronome: TwoAndFourMetronome {},
        countoff: true,
        loops: 1000,
        bpm,
    };

    play(exercise);
}

#[test]
fn metronome() {
    let bpm = 65.;

    let exercise = MetronomeExercise {
        // metronome: EmphasisOneMetronome {},
        metronome: TickEveryBeatMetronome {},
        countoff: false,
        loops: 1000,
        bpm,
    };

    play(exercise);
}

#[test]
fn concentration_metronome() {
    let bpm: f64 = rand::random_range(23f64..29f64);

    let exercise = MetronomeExercise {
        metronome: TickEveryBeatMetronome {},
        countoff: false,
        loops: 1000,
        bpm,
    };

    play(exercise);
}

#[test]
fn count_in_practice() {
    let mut conn = midi::open_midi_connection("128:0");

    for _ in 0..10 {
        std::thread::sleep(std::time::Duration::from_millis(
            rand::rng().random_range(900..1500),
        ));

        let bpm: f64 = rand::rng().random_range(120f64..=140.);
        println!("BPM: {bpm}");

        let mut sequence = metronome_emphasis(bpm).r#loop(5);

        sequence.play(&mut conn);
    }
}
