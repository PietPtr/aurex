use aurex::{
    sequence::{NoteWithDuration, Play, Rhythm, Sequence},
    theory::chords,
};
use wmidi::{Channel, Note};

/// Plays a C major chord, then either the C above or the C below that chord.
#[test]
fn first_sequence() {
    let mut conn = aurex::midi::open_midi_connection("128:0");
    let mut sequence = Sequence::new(120);

    sequence.add_to_end(NoteWithDuration {
        note: Play::c(4),
        duration: Rhythm::Half,
        channel: Channel::Ch1,
    });
    sequence.add_beat_offset(
        -Rhythm::Quarter.beats(),
        NoteWithDuration {
            note: Play::e(4),
            duration: Rhythm::Half,
            channel: Channel::Ch1,
        },
    );
    sequence.add_beat_offset(
        -Rhythm::Quarter.beats(),
        NoteWithDuration {
            note: Play::g(4),
            duration: Rhythm::Quarter,
            channel: Channel::Ch1,
        },
    );
    sequence.add_to_end(NoteWithDuration {
        note: Play::RandomNote(vec![Note::C4, Note::C5]),
        duration: Rhythm::Eighth,
        channel: Channel::Ch1,
    });

    sequence.play(&mut conn);
}

#[test]
fn add_simultaneous() {
    let mut conn = aurex::midi::open_midi_connection("128:0");
    let mut sequence = Sequence::new(90);

    sequence.add_simultaneous(NoteWithDuration {
        note: Play::c(4),
        duration: Rhythm::Half,
        channel: Channel::Ch1,
    });

    sequence.add_simultaneous(NoteWithDuration {
        note: Play::e(4),
        duration: Rhythm::Half,
        channel: Channel::Ch1,
    });

    sequence.add_simultaneous(NoteWithDuration {
        note: Play::g(4),
        duration: Rhythm::Half,
        channel: Channel::Ch1,
    });

    sequence.play(&mut conn);
}

#[test]
fn add_chords() {
    let mut conn = aurex::midi::open_midi_connection("128:0");
    let mut sequence = Sequence::new(78);

    let c_major = Play::c(4).chord(chords::MAJOR);
    let d_minor = Play::d(4).chord(chords::MINOR);
    let e_minor = Play::e(4).chord(chords::MINOR);
    let f_major = Play::f(4).chord(chords::MAJOR);

    sequence.add_chord(&c_major, Rhythm::Half, Channel::Ch1);
    sequence.add_chord(&c_major, Rhythm::Eighth, Channel::Ch1);
    sequence.add_chord(&d_minor, Rhythm::Eighth, Channel::Ch1);
    sequence.add_chord(&e_minor, Rhythm::Eighth, Channel::Ch1);
    sequence.add_chord(&f_major, Rhythm::Half, Channel::Ch1);
    sequence.add_chord(&f_major, Rhythm::Eighth, Channel::Ch1);
    sequence.add_chord(&e_minor, Rhythm::Eighth, Channel::Ch1);
    sequence.add_chord(&d_minor, Rhythm::Eighth, Channel::Ch1);
    sequence.add_chord(&c_major, Rhythm::Whole, Channel::Ch1);

    sequence.play(&mut conn);
}
