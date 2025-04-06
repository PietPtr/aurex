// TODO: chord progressions? too generic? too specific?

use super::notes::NoteName;

/// Utility to define chord progressions given a root note and roman numeral notation
pub struct ChordProgression {
    chords: Vec<Vec<wmidi::Note>>,
}

impl ChordProgression {
    pub fn construct(root: wmidi::Note, chords: Vec<RomanNumeralChord>) {}
}

type RomanNumeralChord = (RomanNumerals, ChordExtensions);

#[allow(non_camel_case_types)]
pub enum RomanNumerals {
    I,
    II,
    III,
    IV,
    V,
    VI,
    VII,
    i,
    ii,
    iii,
    iv,
    v,
    vi,
    vii,
}

#[allow(non_camel_case_types)]
pub enum ChordExtensions {
    None,
    maj7,
    dom7,
    dim7,
    min7,
    aug,
    dim,
}
