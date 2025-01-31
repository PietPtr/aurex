use aurex::{
    drums::{count_off, metronome_emphasis},
    sequence::{NoteDuration, Play, Sequence},
    theory::{scales, Interval},
};

#[test]
fn fourth_and_fifths() {
    let mut conn = aurex::midi::open_midi_connection("128:0");

    let bpm = 78;
    let loops = 10;
    let root = wmidi::Note::C2;

    let fifth = Interval::PerfectFifth.offset(root);
    let fourth = Interval::PerfectFourth.offset(root);

    let count_off = count_off(bpm);
    let metronome = metronome_emphasis(bpm).r#loop(loops);

    let mut sequence = Sequence::new(bpm);
    sequence.add_to_end(Play::Note(root).with_duration(NoteDuration::Quarter));
    sequence.add_to_end(Play::RandomNote(vec![fifth, fourth]).with_duration(NoteDuration::Quarter));
    sequence.add_to_end(Play::Rest.with_duration(NoteDuration::Half));
    let sequence = sequence.r#loop(loops).combine_simultaneous(metronome);

    (count_off.combine_at_end(sequence)).play(&mut conn);
}

// TODO: functions are almost identical, abstract into library.
#[test]
fn major_pentatonic_known_root() {
    let mut conn = aurex::midi::open_midi_connection("128:0");

    let bpm = 78;
    let loops = 10;
    let root = wmidi::Note::C2;

    let count_off = count_off(bpm);
    let metronome = metronome_emphasis(bpm).r#loop(loops);

    let root_note = Play::Note(root);

    let mut sequence = Sequence::new(bpm);
    sequence.add_to_end(root_note.with_duration(NoteDuration::Quarter));
    sequence.add_to_end(
        Play::RandomNote(scales::scale(root, scales::MAJOR_PENTATONIC))
            .with_duration(NoteDuration::Quarter),
    );
    sequence.add_to_end(Play::Rest.with_duration(NoteDuration::Half));

    let sequence = sequence.r#loop(loops).combine_simultaneous(metronome);

    (count_off.combine_at_end(sequence)).play(&mut conn);
}
