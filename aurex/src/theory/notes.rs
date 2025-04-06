use std::hash::Hash;

#[derive(Debug, Clone, Copy)]
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

impl Hash for NoteName {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            NoteName::As | NoteName::Gis => 0u8.hash(state),
            NoteName::A => 1u8.hash(state),
            NoteName::Ais | NoteName::Bes => 2u8.hash(state),
            NoteName::B | NoteName::Ces => 3u8.hash(state),
            NoteName::Bis | NoteName::C => 4u8.hash(state),
            NoteName::Cis | NoteName::Des => 5u8.hash(state),
            NoteName::D => 6u8.hash(state),
            NoteName::Dis | NoteName::Es => 7u8.hash(state),
            NoteName::E | NoteName::Fes => 8u8.hash(state),
            NoteName::Eis | NoteName::F => 9u8.hash(state),
            NoteName::Fis | NoteName::Ges => 10u8.hash(state),
            NoteName::G => 11u8.hash(state),
        }
    }
}

// Did not check if this is correct
impl Eq for NoteName {}
