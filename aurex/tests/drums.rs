use aurex::{
    drums::{basic_backbeat, count_off},
    sequence::{Rhythm, Sequence},
};

#[test]
fn test_drum_channel() {
    let mut conn = aurex::midi::open_midi_connection("128:0");
    let mut sequence = Sequence::new(78.);

    for _ in 0..10 {
        sequence.add_to_end(aurex::drums::METRONOME_EMPH.with_duration(Rhythm::Quarter));
    }

    sequence.play(&mut conn);
}

#[test]
fn test_count_off_and_backbeat() {
    let mut conn = aurex::midi::open_midi_connection("128:0");
    let bpm = 125.;

    let mut count_off = count_off(bpm).combine_at_end(basic_backbeat(bpm).r#loop(4));

    count_off.play(&mut conn);
}

// TODO: define a few warming ups, e.g. scales / arpeggios
