use aurex::{
    drums::{basic_backbeat, count_off},
    sequence::{NoteWithDuration, Play, Sequence},
};

#[test]
fn test_drum_channel() {
    let mut conn = aurex::midi::open_midi_connection("128:0");
    let mut sequence = Sequence::new(78);

    for _ in 0..10 {
        sequence.add_to_end(NoteWithDuration {
            note: Play::b(1),
            duration: aurex::sequence::NoteDuration::Quarter,
            channel: wmidi::Channel::Ch10,
        });
    }

    sequence.play(&mut conn);
}

#[test]
fn test_count_off_and_backbeat() {
    let mut conn = aurex::midi::open_midi_connection("128:0");
    let bpm = 125;

    let mut count_off = count_off(bpm).combine_at_end(basic_backbeat(bpm).r#loop(4));

    count_off.play(&mut conn);
}
