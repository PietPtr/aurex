use std::{os::unix::raw::gid_t, rc::Rc};

use aurex::sequence::{NoteDuration, NoteWithDuration, Play, Sequence};
use wmidi::{Channel, Note};

// TODO: drum loops
// TODO: a first practice sequence: drums + a fourth or fifth above the given root

/// Plays a C major chord, then either the C above or the C below that chord twice.
#[test]
fn first_sequence() {
    let mut conn = aurex::midi::open_midi_connection("128:0");
    let mut sequence = Sequence::new(120);

    sequence.add_to_end(NoteWithDuration {
        note: Play::c(4),
        duration: NoteDuration::Half,
        channel: Channel::Ch1,
    });
    sequence.add_beat_offset(
        -NoteDuration::Quarter.beats(),
        NoteWithDuration {
            note: Play::e(4),
            duration: NoteDuration::Half,
            channel: Channel::Ch1,
        },
    );
    sequence.add_beat_offset(
        -NoteDuration::Quarter.beats(),
        NoteWithDuration {
            note: Play::g(4),
            duration: NoteDuration::Quarter,
            channel: Channel::Ch1,
        },
    );
    sequence.add_to_end(NoteWithDuration {
        note: Play::RandomNote(vec![Note::C4, Note::C5]),
        duration: NoteDuration::Eigth,
        channel: Channel::Ch1,
    });
    sequence.add_beat_offset(
        NoteDuration::Eigth.beats(),
        NoteWithDuration {
            note: Play::ClosureNote(Rc::new(|notes| notes.last().copied())),
            duration: NoteDuration::Half,
            channel: Channel::Ch1,
        },
    );

    sequence.play(&mut conn);
}

#[test]
fn add_simultaneous() {
    let mut conn = aurex::midi::open_midi_connection("128:0");
    let mut sequence = Sequence::new(90);

    sequence.add_simultaneous(NoteWithDuration {
        note: Play::c(4),
        duration: NoteDuration::Half,
        channel: Channel::Ch1,
    });

    sequence.add_simultaneous(NoteWithDuration {
        note: Play::e(4),
        duration: NoteDuration::Half,
        channel: Channel::Ch1,
    });

    sequence.add_simultaneous(NoteWithDuration {
        note: Play::g(4),
        duration: NoteDuration::Half,
        channel: Channel::Ch1,
    });

    sequence.play(&mut conn);
}

#[test]
fn add_chords() {
    let mut conn = aurex::midi::open_midi_connection("128:0");
    let mut sequence = Sequence::new(78);

    let c_major = vec![Play::c(4), Play::e(4), Play::g(4)];
    let d_minor = vec![Play::d(4), Play::f(4), Play::a(4)];
    let e_minor = vec![Play::e(4), Play::g(4), Play::b(4)];
    let f_major = vec![Play::f(4), Play::a(4), Play::c(5)];

    sequence.add_chord(&c_major, NoteDuration::Half, Channel::Ch1);
    sequence.add_chord(&c_major, NoteDuration::Eigth, Channel::Ch1);
    sequence.add_chord(&d_minor, NoteDuration::Eigth, Channel::Ch1);
    sequence.add_chord(&e_minor, NoteDuration::Eigth, Channel::Ch1);
    sequence.add_chord(&f_major, NoteDuration::Half, Channel::Ch1);
    sequence.add_chord(&f_major, NoteDuration::Eigth, Channel::Ch1);
    sequence.add_chord(&e_minor, NoteDuration::Eigth, Channel::Ch1);
    sequence.add_chord(&d_minor, NoteDuration::Eigth, Channel::Ch1);
    sequence.add_chord(&c_major, NoteDuration::Whole, Channel::Ch1);

    sequence.play(&mut conn);
}
