#[derive(Debug, Clone, Copy)]
pub enum Interval {
    Unison,
    MinorSecond,
    MajorSecond,
    MinorThird,
    MajorThird,
    PerfectFourth,
    AugmentedFourth,
    Tritone,
    DiminishedFifth,
    PerfectFifth,
    MinorSixth,
    MajorSixth,
    MinorSeventh,
    MajorSeventh,
    Octave,
    MinorNinth,
    MajorNinth,
    OctaveAndMinorSecond,
    OctaveAndMajorSecond,
    OctaveAndMinorThird,
    OctaveAndMajorThird,
    OctaveAndPerfectFourth,
    OctaveAndAugmentedFourth,
    OctaveAndTritone,
    OctaveAndDiminishedFifth,
    OctaveAndPerfectFifth,
    OctaveAndMinorSixth,
    OctaveAndMajorSixth,
    OctaveAndMinorSeventh,
    OctaveAndMajorSeventh,
    TwoOctaves,
    /// A generic interval that consists of any amount of semitones
    Interval {
        semitones: u8,
    },
}

#[derive(Hash, Debug, Clone, Copy)]
pub enum NoteName {
    As,
    A,
    Ais,
    Bes,
    B,
    Bis,
    Ces,
    C,
    Cis,
    Des,
    D,
    Dis,
    Es,
    E,
    Eis,
    Fes,
    F,
    Fis,
    Ges,
    G,
    Gis,
}

impl NoteName {
    pub fn from_note_sharp(note: wmidi::Note) -> Self {
        let note = (note as u8) % 12;

        match note {
            0 => Self::C,
            1 => Self::Cis,
            2 => Self::D,
            3 => Self::Dis,
            4 => Self::E,
            5 => Self::F,
            6 => Self::Fis,
            7 => Self::G,
            8 => Self::Gis,
            9 => Self::A,
            10 => Self::Ais,
            11 => Self::B,
            n => panic!("mod 12 does not produce {n}"),
        }
    }

    pub fn from_note_flat(note: wmidi::Note) -> Self {
        let note = (note as u8) % 12;

        match note {
            0 => Self::C,
            1 => Self::Des,
            2 => Self::D,
            3 => Self::Es,
            4 => Self::E,
            5 => Self::F,
            6 => Self::Ges,
            7 => Self::G,
            8 => Self::As,
            9 => Self::A,
            10 => Self::Bes,
            11 => Self::B,
            n => panic!("mod 12 does not produce {n}"),
        }
    }

    pub fn with_octave_sharp(note: wmidi::Note) -> (Self, i8) {
        let octave = (note as i8) / 12 - 1;
        let note = Self::from_note_sharp(note);
        (note, octave)
    }

    pub fn with_octave_flat(note: wmidi::Note) -> (Self, i8) {
        let octave = (note as i8) / 12 - 1;
        let note = Self::from_note_flat(note);
        (note, octave)
    }
}

impl PartialEq for NoteName {
    fn eq(&self, other: &Self) -> bool {
        use NoteName::*;
        matches!(
            (self, other),
            (As, Gis) | (Bes, Ais) | (Ces, B) | (Des, Cis) | (Es, Dis) | (Fes, E) | (Ges, Fis)
        ) || matches!(
            (self, other),
            (Gis, As) | (Ais, Bes) | (B, Ces) | (Cis, Des) | (Dis, Es) | (E, Fes) | (Fis, Ges)
        )
    }
}

// Did not check if this is correct
impl Eq for NoteName {}

impl Interval {
    pub fn semitones(self) -> u8 {
        match self {
            Interval::Unison => 0,
            Interval::MinorSecond => 1,
            Interval::MajorSecond => 2,
            Interval::MinorThird => 3,
            Interval::MajorThird => 4,
            Interval::PerfectFourth => 5,
            Interval::AugmentedFourth => 6,
            Interval::Tritone => 6,
            Interval::DiminishedFifth => 6,
            Interval::PerfectFifth => 7,
            Interval::MinorSixth => 8,
            Interval::MajorSixth => 9,
            Interval::MinorSeventh => 10,
            Interval::MajorSeventh => 11,
            Interval::Octave => 12,
            Interval::MinorNinth => 13,
            Interval::MajorNinth => 14,
            Interval::OctaveAndMinorSecond => 12 + 1,
            Interval::OctaveAndMajorSecond => 12 + 2,
            Interval::OctaveAndMinorThird => 12 + 3,
            Interval::OctaveAndMajorThird => 12 + 4,
            Interval::OctaveAndPerfectFourth => 12 + 5,
            Interval::OctaveAndAugmentedFourth => 12 + 6,
            Interval::OctaveAndTritone => 12 + 6,
            Interval::OctaveAndDiminishedFifth => 12 + 6,
            Interval::OctaveAndPerfectFifth => 12 + 7,
            Interval::OctaveAndMinorSixth => 12 + 8,
            Interval::OctaveAndMajorSixth => 12 + 9,
            Interval::OctaveAndMinorSeventh => 12 + 10,
            Interval::OctaveAndMajorSeventh => 12 + 11,
            Interval::TwoOctaves => 12 + 12,
            Interval::Interval { semitones } => semitones,
        }
    }

    pub fn isemitones(self) -> isize {
        self.semitones() as isize
    }

    pub fn offset(self, note: wmidi::Note) -> wmidi::Note {
        wmidi::Note::from_u8_lossy((note as u8) + self.semitones())
    }
}

pub mod chords {
    use super::Interval as I;

    pub const MAJOR: &[I; 2] = &[I::MajorThird, I::PerfectFifth];
    pub const MAJOR_SEVENTH: &[I; 3] = &[I::MajorThird, I::PerfectFifth, I::MajorSeventh];
    pub const MINOR: &[I; 2] = &[I::MinorThird, I::PerfectFifth];
    pub const MINOR_SEVENTH: &[I; 3] = &[I::MinorThird, I::PerfectFifth, I::MinorSeventh];
    pub const DOMINANT_SEVENTH: &[I; 3] = &[I::MajorThird, I::PerfectFifth, I::MinorSeventh];
    pub const HALF_DIMINISHED: &[I; 2] = &[I::MinorThird, I::DiminishedFifth];
    pub const HALF_DIMINISHED_SEVENTH: &[I; 3] =
        &[I::MinorThird, I::DiminishedFifth, I::MinorSeventh];
}

pub mod scales {
    use std::collections::HashSet;

    use super::{Interval as I, NoteName};

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
}
