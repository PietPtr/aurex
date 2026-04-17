use aurex::{
    exercises::{arpeggios::OneOctaveArpeggios, play},
    metronome::metronomes::EmphasisOneMetronome,
    player::playonce::PlayOnce,
};
use wmidi::Note;

const ROOT: Note = Note::B1;

#[test]
fn arpeggios() {
    let exercise = PlayOnce {
        exercise: OneOctaveArpeggios {
            bpm: 60.,
            root: ROOT,
        },
        metronome: EmphasisOneMetronome {},
        loops: 2,
    };

    play(exercise);
}
