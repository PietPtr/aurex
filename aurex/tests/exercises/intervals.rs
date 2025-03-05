use aurex::{
    exercises::{
        in_scale::{InScaleExercise, InScaleWithRangeExercise},
        known_root::{self, KnownRootExercise},
        Exercise,
    },
    theory::{scales, Interval},
};
use wmidi::Note;

const ROOT: Note = Note::E2;

#[test]
fn seconds() {
    let exercise = KnownRootExercise {
        bpm: 78,
        loops: 10,
        root: ROOT,
        scale: vec![Interval::MinorSecond, Interval::MajorSecond],
        direction: known_root::Direction::Ascending,
        root_position: known_root::RootPosition::StartOnRoot,
    };

    exercise.play();
}

#[test]
fn thirds() {
    let exercise = KnownRootExercise {
        bpm: 78,
        loops: 10,
        root: ROOT,
        scale: vec![Interval::MinorThird, Interval::MajorThird],
        direction: known_root::Direction::Ascending,
        root_position: known_root::RootPosition::StartOnRoot,
    };

    exercise.play();
}

#[test]
fn thirds_but_theres_other_stuff() {
    let exercise = KnownRootExercise {
        bpm: 100,
        loops: 10,
        root: ROOT,
        scale: vec![
            Interval::MinorThird,
            Interval::MajorThird,
            Interval::PerfectFifth,
            Interval::Octave,
        ],
        direction: known_root::Direction::Ascending,
        root_position: known_root::RootPosition::EndOnRoot,
    };

    exercise.play();
}

// TODO: Build a metronome that accents 2 and 4 / can play at a random tempo between range / etc?

#[test]
fn fourth_and_fifths() {
    let exercise = KnownRootExercise {
        bpm: 160,
        loops: 30,
        root: ROOT,
        scale: vec![Interval::PerfectFourth, Interval::PerfectFifth],
        direction: known_root::Direction::Ascending,
        root_position: known_root::RootPosition::EndOnRoot,
    };

    exercise.play();
}

#[test]
fn major_fourth_and_fifths() {
    let exercise = KnownRootExercise {
        bpm: 78,
        loops: 10,
        root: ROOT,
        scale: vec![
            Interval::MajorThird,
            Interval::PerfectFourth,
            Interval::PerfectFifth,
        ],
        direction: known_root::Direction::Ascending,
        root_position: known_root::RootPosition::EndOnRoot,
    };

    exercise.play();
}

#[test]
fn sixths() {
    let exercise = KnownRootExercise {
        bpm: 78,
        loops: 10,
        root: ROOT,
        scale: vec![Interval::MinorSixth, Interval::MajorSixth],
        direction: known_root::Direction::Ascending,
        root_position: known_root::RootPosition::StartOnRoot,
    };

    exercise.play();
}

#[test]
fn sevenths() {
    let exercise = KnownRootExercise {
        bpm: 78,
        loops: 10,
        root: ROOT,
        scale: vec![Interval::MinorSeventh, Interval::MajorSeventh],
        direction: known_root::Direction::Ascending,
        root_position: known_root::RootPosition::StartOnRoot,
    };

    exercise.play();
}

#[test]
fn major_pentatonic_known_root() {
    let exercise = KnownRootExercise {
        bpm: 100,
        loops: 30,
        root: ROOT,
        scale: scales::MAJOR_PENTATONIC.to_vec(),
        direction: known_root::Direction::Ascending,
        root_position: known_root::RootPosition::StartOnRoot,
    };

    exercise.play();
}

#[test]
fn major_pentatonic() {
    let exercise = InScaleExercise {
        bpm: 100,
        loops: 10,
        root: ROOT,
        scale: scales::MAJOR_PENTATONIC.to_vec(),
    };

    exercise.play();
}

#[test]
fn major() {
    let exercise = KnownRootExercise {
        bpm: 100,
        loops: 10,
        root: ROOT,
        scale: scales::MAJOR.to_vec(),
        direction: known_root::Direction::Ascending,
        root_position: known_root::RootPosition::StartOnRoot,
    };

    exercise.play();
}

#[test]
fn minor() {
    let exercise = KnownRootExercise {
        bpm: 90,
        loops: 10,
        root: ROOT,
        scale: scales::TWO_OCTAVE_MINOR[0..8].to_vec(),
        direction: known_root::Direction::Ascending,
        root_position: known_root::RootPosition::EndOnRoot,
    };

    exercise.play();
}

#[test]
fn two_octave_major_pentatonic() {
    let exercise = InScaleWithRangeExercise {
        bpm: 100,
        loops: 10,
        root: ROOT,
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
        root: ROOT,
        scale: scales::MAJOR.to_vec(),
        range_start: Note::D2,
        range_end: Note::FSharp2,
    };

    exercise.play();
}
