use aurex::{
    exercises::{known_root::KnownRootExercise, Exercise},
    theory::{scales, Interval},
};

#[test]
fn fourth_and_fifths() {
    let root = wmidi::Note::C2;

    let exercise = KnownRootExercise {
        bpm: 78,
        loops: 10,
        root,
        scale: vec![Interval::PerfectFifth, Interval::PerfectFourth],
    };

    exercise.play();
}

#[test]
fn major_pentatonic_known_root() {
    let exercise = KnownRootExercise {
        bpm: 180,
        loops: 100,
        root: wmidi::Note::C2,
        scale: scales::MAJOR_PENTATONIC.to_vec(),
    };

    exercise.play();
}
