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
