use aurex::{
    exercises::{chords::DiatonicChordExercise, Exercise},
    theory::scales,
};
use wmidi::Note;

#[test]
fn diatonic_chords() {
    let exercise = DiatonicChordExercise {
        bpm: 75,
        loops: 10,
        root: Note::C3,
        scale: scales::MAJOR.to_vec(),
    };

    exercise.play();
}
