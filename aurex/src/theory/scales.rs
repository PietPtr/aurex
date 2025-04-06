use std::collections::HashSet;

use super::{intervals::Interval as I, notes::NoteName};

pub const IONIAN: &[I; 7] = &[
    I::Unison,
    I::MajorSecond,
    I::MajorThird,
    I::PerfectFourth,
    I::PerfectFifth,
    I::MajorSixth,
    I::MajorSeventh,
];

pub const DORIAN: &[I; 7] = &[
    I::Unison,
    I::MajorSecond,
    I::MinorThird,
    I::PerfectFourth,
    I::PerfectFifth,
    I::MajorSixth,
    I::MinorSeventh,
];

pub const PHRYGIAN: &[I; 7] = &[
    I::Unison,
    I::MinorSecond,
    I::MinorThird,
    I::PerfectFourth,
    I::PerfectFifth,
    I::MinorSixth,
    I::MinorSeventh,
];

pub const LYDIAN: &[I; 7] = &[
    I::Unison,
    I::MajorSecond,
    I::MajorThird,
    I::AugmentedFourth,
    I::PerfectFifth,
    I::MajorSixth,
    I::MajorSeventh,
];

pub const MIXOLYDIAN: &[I; 7] = &[
    I::Unison,
    I::MajorSecond,
    I::MajorThird,
    I::PerfectFourth,
    I::PerfectFifth,
    I::MajorSixth,
    I::MinorSeventh,
];

pub const AEOLIAN: &[I; 7] = &[
    I::Unison,
    I::MajorSecond,
    I::MinorThird,
    I::PerfectFourth,
    I::PerfectFifth,
    I::MinorSixth,
    I::MinorSeventh,
];

pub const LOCRIAN: &[I; 7] = &[
    I::Unison,
    I::MinorSecond,
    I::MinorThird,
    I::PerfectFourth,
    I::DiminishedFifth,
    I::MinorSixth,
    I::MinorSeventh,
];

pub const MAJOR: &[I; 7] = IONIAN;

pub const TWO_OCTAVE_MAJOR: &[I; 14] = &[
    I::Unison,
    I::MajorSecond,
    I::MajorThird,
    I::PerfectFourth,
    I::PerfectFifth,
    I::MajorSixth,
    I::MajorSeventh,
    I::Octave,
    I::OctaveAndMajorSecond,
    I::OctaveAndMajorThird,
    I::OctaveAndPerfectFourth,
    I::OctaveAndPerfectFifth,
    I::OctaveAndMajorSixth,
    I::OctaveAndMajorSeventh,
];

pub const MINOR: &[I; 7] = AEOLIAN;

pub const TWO_OCTAVE_MINOR: &[I; 14] = &[
    I::Unison,
    I::MajorSecond,
    I::MinorThird,
    I::PerfectFourth,
    I::PerfectFifth,
    I::MinorSixth,
    I::MinorSeventh,
    I::Octave,
    I::OctaveAndMajorSecond,
    I::OctaveAndMinorThird,
    I::OctaveAndPerfectFourth,
    I::OctaveAndPerfectFifth,
    I::OctaveAndMinorSixth,
    I::OctaveAndMinorSeventh,
];

pub const HARMONIC_MINOR: &[I; 8] = &[
    I::Unison,
    I::MajorSecond,
    I::MinorThird,
    I::PerfectFourth,
    I::PerfectFifth,
    I::MinorSixth,
    I::MajorSeventh,
    I::Octave, // TODO: octave or not everywhere...?
];

pub const MAJOR_PENTATONIC: &[I; 5] = &[
    I::Unison,
    I::MajorSecond,
    I::MajorThird,
    I::PerfectFifth,
    I::MajorSixth,
];

pub const TWO_OCTAVE_MAJOR_PENTATONIC: &[I; 10] = &[
    I::Unison,
    I::MajorSecond,
    I::MajorThird,
    I::PerfectFifth,
    I::MajorSixth,
    I::Octave,
    I::OctaveAndMajorSecond,
    I::OctaveAndMajorThird,
    I::OctaveAndPerfectFifth,
    I::OctaveAndMajorSixth,
];

pub const MINOR_PENTATONIC: &[I; 5] = &[
    I::Unison,
    I::MinorThird,
    I::PerfectFourth,
    I::PerfectFifth,
    I::MinorSeventh,
];

// TODO: make a Scale trait

pub fn set_of_pitch_classes(notes: Vec<wmidi::Note>) -> HashSet<NoteName> {
    HashSet::from_iter(notes.iter().map(|n| NoteName::from_note_sharp(*n)))
}

pub fn scale(root: wmidi::Note, intervals: &[I]) -> Vec<wmidi::Note> {
    let mut notes = vec![];

    for interval in intervals {
        notes.push(wmidi::Note::from_u8_lossy(
            root as u8 + interval.semitones(),
        ));
    }

    notes
}

pub fn descending_scale(root: wmidi::Note, intervals: &[I]) -> Vec<wmidi::Note> {
    let mut notes = vec![];

    for interval in intervals {
        notes.push(wmidi::Note::from_u8_lossy(
            root as u8 - interval.semitones(),
        ));
    }

    notes
}

pub fn scale_range(
    root: wmidi::Note,
    intervals: &[I],
    start: wmidi::Note,
    end: wmidi::Note,
) -> Vec<wmidi::Note> {
    let scale = scale(root, intervals)
        .into_iter()
        .map(|n| (n as u8) % 12)
        .collect::<Vec<_>>();

    let mut notes = vec![];

    for note in (start as u8)..(end as u8) {
        if scale.contains(&(note % 12)) {
            notes.push(wmidi::Note::from_u8_lossy(note));
        }
    }

    notes
}

pub fn scale_range_offset(
    root: wmidi::Note,
    intervals: &[I],
    start: wmidi::Note,
    end: u8, // In semitones
) -> Vec<wmidi::Note> {
    scale_range(
        root,
        intervals,
        start,
        wmidi::Note::from_u8_lossy(start as u8 + end),
    )
}
