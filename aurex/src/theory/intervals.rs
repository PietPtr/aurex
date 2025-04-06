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
