use aurex::{
    drums::{metronome_backbeat, metronome_emphasis},
    midi,
};
use rand::Rng;

/// Clicks on 2 and 4 at a random tempo between 55 and 68 BPM
/// TODO: maybe put this in an Exercise implementing struct with BPM as setting
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
