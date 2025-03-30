use aurex::{
    exercises::{
        in_scale::{InScaleExercise, InScaleWithRangeExercise},
        known_root::{self, KnownRootExercise},
        play,
    },
    metronome::emphasis_one::EmphasisOneMetronome,
    player::playtwice::PlayTwice,
    theory::{scales, Interval},
};
use wmidi::Note;

const ROOT: Note = Note::Eb2;

#[test]
fn seconds() {
    let exercise = PlayTwice {
        exercise: KnownRootExercise {
            bpm: 78.,
            root: ROOT,
            scale: vec![Interval::MinorSecond, Interval::MajorSecond],
            direction: known_root::Direction::Ascending,
            root_position: known_root::RootPosition::StartOnRoot,
        },
        metronome: EmphasisOneMetronome {},
        loops: 10,
    };

    play(exercise);
}

#[test]
fn thirds() {
    let exercise = PlayTwice {
        exercise: KnownRootExercise {
            bpm: 78.,
            root: ROOT,
            scale: vec![Interval::MinorThird, Interval::MajorThird],
            direction: known_root::Direction::Ascending,
            root_position: known_root::RootPosition::StartOnRoot,
        },
        metronome: EmphasisOneMetronome {},
        loops: 10,
    };

    play(exercise);
}

#[test]
fn thirds_but_theres_other_stuff() {
    let exercise = PlayTwice {
        exercise: KnownRootExercise {
            bpm: 100.,
            root: ROOT,
            scale: vec![
                Interval::MinorThird,
                Interval::MajorThird,
                Interval::PerfectFifth,
                Interval::Octave,
            ],
            direction: known_root::Direction::Ascending,
            root_position: known_root::RootPosition::EndOnRoot,
        },
        metronome: EmphasisOneMetronome {},
        loops: 10,
    };

    play(exercise);
}

#[test]
fn fourth_and_fifths() {
    let exercise = PlayTwice {
        exercise: KnownRootExercise {
            bpm: 160.,
            root: ROOT,
            scale: vec![Interval::PerfectFourth, Interval::PerfectFifth],
            direction: known_root::Direction::Ascending,
            root_position: known_root::RootPosition::EndOnRoot,
        },
        metronome: EmphasisOneMetronome {},
        loops: 10,
    };

    play(exercise);
}

#[test]
fn major_fourth_and_fifths() {
    let exercise = PlayTwice {
        exercise: KnownRootExercise {
            bpm: 78.,
            root: ROOT,
            scale: vec![
                Interval::MajorThird,
                Interval::PerfectFourth,
                Interval::PerfectFifth,
            ],
            direction: known_root::Direction::Ascending,
            root_position: known_root::RootPosition::EndOnRoot,
        },
        metronome: EmphasisOneMetronome {},
        loops: 10,
    };

    play(exercise);
}

#[test]
fn sixths() {
    let exercise = PlayTwice {
        exercise: KnownRootExercise {
            bpm: 78.,
            root: ROOT,
            scale: vec![Interval::MinorSixth, Interval::MajorSixth],
            direction: known_root::Direction::Ascending,
            root_position: known_root::RootPosition::StartOnRoot,
        },
        metronome: EmphasisOneMetronome {},
        loops: 10,
    };

    play(exercise);
}

#[test]
fn sevenths() {
    let exercise = PlayTwice {
        exercise: KnownRootExercise {
            bpm: 78.,
            root: ROOT,
            scale: vec![Interval::MinorSeventh, Interval::MajorSeventh],
            direction: known_root::Direction::Ascending,
            root_position: known_root::RootPosition::StartOnRoot,
        },
        metronome: EmphasisOneMetronome {},
        loops: 10,
    };

    play(exercise);
}

#[test]
fn major_pentatonic_known_root() {
    let exercise = PlayTwice {
        exercise: KnownRootExercise {
            bpm: 100.,
            root: ROOT,
            scale: scales::MAJOR_PENTATONIC.to_vec(),
            direction: known_root::Direction::Ascending,
            root_position: known_root::RootPosition::StartOnRoot,
        },
        metronome: EmphasisOneMetronome {},
        loops: 10,
    };

    play(exercise);
}

#[test]
fn major_pentatonic() {
    let exercise = PlayTwice {
        exercise: InScaleExercise {
            bpm: 100.,
            loops: 10,
            root: ROOT,
            scale: scales::MAJOR_PENTATONIC.to_vec(),
        },
        metronome: EmphasisOneMetronome {},
        loops: 10,
    };

    play(exercise);
}

#[test]
fn major() {
    let exercise = PlayTwice {
        exercise: KnownRootExercise {
            bpm: 100.,
            root: ROOT,
            scale: scales::MAJOR.to_vec(),
            direction: known_root::Direction::Ascending,
            root_position: known_root::RootPosition::StartOnRoot,
        },
        metronome: EmphasisOneMetronome {},
        loops: 10,
    };

    play(exercise);
}

#[test]
fn minor() {
    let exercise = PlayTwice {
        exercise: KnownRootExercise {
            bpm: 90.,
            root: ROOT,
            scale: scales::TWO_OCTAVE_MINOR[0..8].to_vec(),
            direction: known_root::Direction::Ascending,
            root_position: known_root::RootPosition::EndOnRoot,
        },
        metronome: EmphasisOneMetronome {},
        loops: 10,
    };

    play(exercise);
}

#[test]
fn two_octave_major_pentatonic() {
    let exercise = PlayTwice {
        exercise: InScaleWithRangeExercise {
            bpm: 100.,
            loops: 10,
            root: ROOT,
            scale: scales::MAJOR_PENTATONIC.to_vec(),
            range_start: Note::C2,
            range_end: Note::C4,
        },
        metronome: EmphasisOneMetronome {},
        loops: 10,
    };

    play(exercise);
}

#[test]
fn part_of_major() {
    let exercise = PlayTwice {
        exercise: InScaleWithRangeExercise {
            bpm: 100.,
            loops: 10,
            root: ROOT,
            scale: scales::MAJOR.to_vec(),
            range_start: Note::D2,
            range_end: Note::FSharp2,
        },
        metronome: EmphasisOneMetronome {},
        loops: 10,
    };

    play(exercise);
}
