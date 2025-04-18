use aurex::{
    exercises::{chords::DiatonicChordExercise, play},
    metronome::drummer::BackbeatDrummer,
    player::playonce::PlayOnce,
    theory::scales,
};
use wmidi::Note;

#[test]
fn diatonic_chords() {
    let exercise = PlayOnce {
        exercise: DiatonicChordExercise {
            bpm: 75.,
            loops: 10,
            root: Note::C3,
            scale: scales::MAJOR.to_vec(),
        },
        metronome: BackbeatDrummer {},
        loops: 10,
    };

    play(exercise);
}
