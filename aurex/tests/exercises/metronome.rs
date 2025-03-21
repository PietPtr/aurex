use aurex::{
    drums::{metronome_backbeat, metronome_emphasis},
    midi,
};
use rand::Rng;

// TODO: if no qsynth, start it (and find its port)

/// Clicks on 2 and 4 at a random tempo between 55 and 68 BPM
/// TODO: maybe put this in an Exercise implementing struct with BPM and metronome style and count off as settings
#[test]
fn warmup_metronome() {
    let mut rng = rand::rng();
    let bpm: u64 = rng.random_range(55..=68);

    println!("Backbeat metronome at {bpm}BPM");

    let start = metronome_emphasis(bpm);
    let sequence = metronome_backbeat(bpm).r#loop(1000);

    let mut conn = midi::open_midi_connection("128:0");
    start.combine_at_end(sequence).play(&mut conn);
}

#[test]
fn metronome() {
    // let bpm: u64 = rand::rng().random_range(120..=140);
    let bpm = 118;

    let mut sequence = metronome_emphasis(bpm).r#loop(1000);

    let mut conn = midi::open_midi_connection("128:0");
    sequence.play(&mut conn);
}

#[test]
fn count_in_practice() {
    let mut conn = midi::open_midi_connection("128:0");

    for _ in 0..10 {
        std::thread::sleep(std::time::Duration::from_millis(
            rand::rng().random_range(900..1500),
        ));

        let bpm: u64 = rand::rng().random_range(120..=140);
        println!("BPM: {bpm}");

        let mut sequence = metronome_emphasis(bpm).r#loop(5);

        sequence.play(&mut conn);
    }
}
