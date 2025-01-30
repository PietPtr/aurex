use std::io;

use aurex::sequence::Play;
use wmidi::{Channel, MidiMessage, Note, U7};

#[test]
fn wmidi_example() {
    let midi_msg =
        MidiMessage::NoteOn(wmidi::Channel::Ch11, wmidi::Note::C4, U7::from_u8_lossy(50));

    println!("{:?}", midi_msg);
    let mut bytes = vec![0u8; midi_msg.bytes_size()];
    midi_msg.copy_to_slice(bytes.as_mut_slice()).unwrap();
    println!("{:?}", bytes);

    let mut conn = aurex::midi::open_midi_connection("128:0");

    for note in 0..128 {
        println!("note={note}");

        bytes[1] = note;
        conn.send(&bytes).unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
    }
}

#[test]
fn midir_example() {
    let mut conn = aurex::midi::open_midi_connection("128:0");

    conn.send(&[0x90, 60, 0x64]).unwrap();
}

#[test]
fn chord_test() {
    let mut conn = aurex::midi::open_midi_connection("128:0");

    let _: Vec<_> = [Note::C4, Note::E4, Note::G4]
        .into_iter()
        .map(|n| MidiMessage::NoteOn(Channel::Ch1, n, U7::from_u8_lossy(50)))
        .map(|m| conn.send(&m.to_vec()))
        .collect();
}

#[test]
fn note_ids() {
    let note = Note::A0;
    dbg!(note, note as usize);

    let play = Play::a(8);
    dbg!(play);
}
