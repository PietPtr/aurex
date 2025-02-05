use crate::sequence::{ChannelNote, Play, Rhythm, Sequence};

pub const SNARE: ChannelNote = ChannelNote {
    note: Play::Note(wmidi::Note::D2),
    channel: wmidi::Channel::Ch10,
};

pub const CLOSED_HAT: ChannelNote = ChannelNote {
    note: Play::Note(wmidi::Note::FSharp2),
    channel: wmidi::Channel::Ch10,
};

pub const STICKS: ChannelNote = ChannelNote {
    note: Play::Note(wmidi::Note::G1),
    channel: wmidi::Channel::Ch10,
};

pub const KICK: ChannelNote = ChannelNote {
    note: Play::Note(wmidi::Note::B1),
    channel: wmidi::Channel::Ch10,
};

pub const METRONOME_TICK: ChannelNote = ChannelNote {
    note: Play::Note(wmidi::Note::F5),
    channel: wmidi::Channel::Ch10,
};

pub const METRONOME_EMPH: ChannelNote = ChannelNote {
    note: Play::Note(wmidi::Note::E5),
    channel: wmidi::Channel::Ch10,
};

pub fn count_off(bpm: u64) -> Sequence {
    let mut sequence = Sequence::new(bpm);

    for _ in 0..2 {
        sequence.add_to_end(STICKS.with_duration(Rhythm::Half));
    }

    for _ in 0..4 {
        sequence.add_to_end(STICKS.with_duration(Rhythm::Quarter));
    }

    sequence
}

pub fn basic_backbeat(bpm: u64) -> Sequence {
    let mut hats = Sequence::new(bpm);
    let mut kick = Sequence::new(bpm);
    let mut snare = Sequence::new(bpm);

    for _ in 0..8 {
        hats.add_to_end(CLOSED_HAT.with_duration(Rhythm::Eighth));
    }

    for _ in 0..2 {
        kick.add_to_end(KICK.with_duration(Rhythm::Half));
    }

    snare.add_beat_offset(Rhythm::Quarter.beats(), SNARE.with_duration(Rhythm::Half));
    snare.add_to_end(SNARE.with_duration(Rhythm::Quarter));

    hats.combine_simultaneous(kick).combine_simultaneous(snare)
}

pub fn metronome_emphasis(bpm: u64) -> Sequence {
    let mut sequence = Sequence::new(bpm);

    sequence.add_to_end(METRONOME_EMPH.with_duration(Rhythm::Quarter));

    for _ in 0..3 {
        sequence.add_to_end(METRONOME_TICK.with_duration(Rhythm::Quarter));
    }

    sequence
}
