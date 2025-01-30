use aurex::sequence::{NoteDuration, NoteWithDuration, Play, Sequence};
use wmidi::{Channel, Note};

// TODO: drum loops
// TODO: a first practice sequence: drums + a fourth or fifth above the given root

/// Plays a C major chord, then either the C above or the C below that chord twice.
#[test]
fn first_sequence() {
    let mut conn = aurex::midi::open_midi_connection("128:0");

    let mut sequence = Sequence::new(60);
    sequence.add_to_end(NoteWithDuration {
        note: Play::c(4),
        duration: NoteDuration::Half,
        channel: Channel::Ch1,
    });
    sequence.add_beat_offset(
        -1.0,
        NoteWithDuration {
            note: Play::e(4),
            duration: NoteDuration::Half,
            channel: Channel::Ch1,
        },
    );
    sequence.add_beat_offset(
        -1.0,
        NoteWithDuration {
            note: Play::g(4),
            duration: NoteDuration::Quarter,
            channel: Channel::Ch1,
        },
    );
    sequence.add_to_end(NoteWithDuration {
        note: Play::RandomNote(vec![Note::C4, Note::C5]),
        duration: NoteDuration::Quarter,
        channel: Channel::Ch1,
    });
    sequence.add_to_end(NoteWithDuration {
        note: Play::ClosureNote(Box::new(|notes| notes.last().copied())),
        duration: NoteDuration::Quarter,
        channel: Channel::Ch1,
    });

    sequence.play(&mut conn);
}
