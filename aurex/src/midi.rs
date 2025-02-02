pub fn open_midi_connection(id: &str) -> midir::MidiOutputConnection {
    let midi_out =
        midir::MidiOutput::new("Rust MIDI Output").expect("Failed to create MIDI output");
    let out_ports = midi_out.ports();
    println!("Looking for {id} in {} ports.", out_ports.len());

    for port in out_ports.into_iter() {
        println!(
            "MIDI device={} {}",
            port.id(),
            midi_out.port_name(&port).unwrap_or("".to_string())
        );
        if port.id().eq(id) {
            let conn = midi_out
                .connect(&port, "MIDI Connection")
                .expect("Failed to connect to MIDI port");
            return conn;
        }
    }

    panic!("Did not find MIDI device {id}.");
}

pub const GRAND_PIANO: wmidi::U7 = wmidi::U7::from_u8_lossy(0);
pub const FINGERED_BASS: wmidi::U7 = wmidi::U7::from_u8_lossy(33);
pub const PICKED_BASS: wmidi::U7 = wmidi::U7::from_u8_lossy(34);

pub fn set_instrument(
    conn: &mut midir::MidiOutputConnection,
    channel: wmidi::Channel,
    instrument: wmidi::U7,
) {
    let message = wmidi::MidiMessage::ProgramChange(channel, instrument);
    conn.send(&message.to_vec()).unwrap();
}
