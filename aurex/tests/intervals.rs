use aurex::{
    exercises::{
        in_scale::{InScaleExercise, InScaleWithRangeExercise},
        known_root::KnownRootExercise,
        Exercise,
    },
    theory::{scales, Interval},
};
use wmidi::Note;

#[test]
fn fourth_and_fifths() {
    let root = Note::C2;

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
        bpm: 100,
        loops: 100,
        root: Note::C2,
        scale: scales::MAJOR_PENTATONIC.to_vec(),
    };

    exercise.play();
}

#[test]
fn major_pentatonic() {
    let exercise = InScaleExercise {
        bpm: 100,
        loops: 10,
        root: Note::C2,
        scale: scales::MAJOR_PENTATONIC.to_vec(),
    };

    exercise.play();
}

#[test]
fn two_octave_major_pentatonic() {
    let exercise = InScaleWithRangeExercise {
        bpm: 100,
        loops: 10,
        root: Note::C2,
        scale: scales::MAJOR_PENTATONIC.to_vec(),
        range_start: Note::C2,
        range_end: Note::C4,
    };

    exercise.play();
}

#[test]
fn part_of_major() {
    let exercise = InScaleWithRangeExercise {
        bpm: 100,
        loops: 10,
        root: Note::C2,
        scale: scales::MAJOR.to_vec(),
        range_start: Note::D2,
        range_end: Note::FSharp2,
    };

    exercise.play();
}
