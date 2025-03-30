use aurex::{
    exercises::{melody::MelodyExercise, play},
    metronome::beat::BeatMetronome,
    player::playonce::PlayOnce,
    random::RandomThings,
    sequence::Rhythm,
    theory::scales,
};

#[test]
fn sixteenths() {
    let exercise = PlayOnce {
        exercise: MelodyExercise {
            bpm: 120,
            root: wmidi::Note::A1,
            scale: scales::MAJOR.to_vec(),
            steps: RandomThings {
                things: [0],
                weights: [1],
            },
            rhythms: RandomThings {
                things: [
                    vec![
                        Rhythm::Staccato(Box::new(Rhythm::Sixteenth)),
                        Rhythm::Staccato(Box::new(Rhythm::Sixteenth)),
                        Rhythm::Staccato(Box::new(Rhythm::Sixteenth)),
                        Rhythm::Staccato(Box::new(Rhythm::Sixteenth)),
                    ],
                    vec![
                        Rhythm::Staccato(Box::new(Rhythm::Eighth)),
                        Rhythm::Staccato(Box::new(Rhythm::Sixteenth)),
                        Rhythm::Staccato(Box::new(Rhythm::Sixteenth)),
                    ],
                    vec![
                        Rhythm::Staccato(Box::new(Rhythm::Sixteenth)),
                        Rhythm::Staccato(Box::new(Rhythm::Eighth)),
                        Rhythm::Staccato(Box::new(Rhythm::Sixteenth)),
                    ],
                    vec![
                        Rhythm::Staccato(Box::new(Rhythm::Sixteenth)),
                        Rhythm::Staccato(Box::new(Rhythm::Sixteenth)),
                        Rhythm::Staccato(Box::new(Rhythm::Eighth)),
                    ],
                ],
                weights: [50, 20, 20, 20],
            },
            amount_of_beats: 1.,
            rest_probability: 0.2,
        },
        metronome: BeatMetronome {},
        loops: 10,
    };

    play(exercise);
}
