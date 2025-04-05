use aurex::{
    drums::{self, metronome_emphasis},
    exercises::{play, Exercise},
    metronome::{emphasis_one::EmphasisOneMetronome, two_and_four::TwoAndFourMetronome, Metronome},
    midi,
    sequence::Sequence,
};
use rand::Rng;

// TODO: if no qsynth, start it

pub struct MetronomeExercise<M: Metronome> {
    pub metronome: M,
    pub bpm: f64,
    pub loops: usize,
    pub countoff: bool,
}

impl<M: Metronome> Exercise for MetronomeExercise<M> {
    fn generate(&self) -> aurex::sequence::Sequence {
        let count = if self.countoff {
            drums::count_off(self.bpm)
        } else {
            Sequence::new(self.bpm)
        };

        count.combine_at_end(M::generate(self.bpm).r#loop(self.loops))
    }

    fn instrument(&self) -> wmidi::U7 {
        // TODO: ..channels?
        midi::FINGERED_BASS
    }

    fn bpm(&self) -> f64 {
        self.bpm
    }
}

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
    let bpm = 118.;

    let exercise = MetronomeExercise {
        metronome: EmphasisOneMetronome {},
        countoff: true,
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
