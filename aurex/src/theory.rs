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
}

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
    use super::Interval as I;

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

    pub const MINOR: &[I; 7] = AEOLIAN;

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

    pub fn scale(root: wmidi::Note, intervals: &[I]) -> Vec<wmidi::Note> {
        let mut notes = vec![];

        for interval in intervals {
            notes.push(wmidi::Note::from_u8_lossy(
                root as u8 + interval.semitones(),
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
}
