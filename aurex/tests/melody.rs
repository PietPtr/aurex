use aurex::{
    exercises::{melody::MelodyExercise, Exercise},
    random::RandomThings,
    sequence::Rhythm,
    theory::scales,
};

#[test]
fn melody() {
    let exercise = MelodyExercise {
        bpm: 100,
        loops: 10,
        root: wmidi::Note::C2,
        scale: scales::MAJOR.to_vec(),
        steps: RandomThings {
            things: [-2, -1, 0, 1, 2],
            weights: [5, 10, 10, 40, 15],
        },
        rhythms: RandomThings {
            things: [
                vec![Rhythm::Quarter],
                vec![Rhythm::Eigth],
                vec![
                    Rhythm::QuarterTriplet,
                    Rhythm::QuarterTriplet,
                    Rhythm::QuarterTriplet,
                ],
                vec![Rhythm::DottedEighth, Rhythm::Sixteenth],
            ],
            weights: [50, 45, 5, 5],
        },
        rest_probability: 0.1,
        amount_of_beats: 3.,
        ..MelodyExercise::default()
    };

    exercise.play();
}

#[test]
fn sixteenths_melody() {
    let exercise = MelodyExercise {
        bpm: 70,
        loops: 10,
        root: wmidi::Note::C2,
        scale: scales::MAJOR.to_vec(),
        steps: RandomThings {
            things: [-5, -3, 0, 3, 5],
            weights: [5, 5, 80, 5, 5],
        },
        rhythms: RandomThings {
            things: [
                vec![
                    Rhythm::Sixteenth,
                    Rhythm::Sixteenth,
                    Rhythm::Sixteenth,
                    Rhythm::Sixteenth,
                ],
                vec![Rhythm::Eigth, Rhythm::Sixteenth, Rhythm::Sixteenth],
                vec![Rhythm::Sixteenth, Rhythm::Eigth, Rhythm::Sixteenth],
                vec![Rhythm::Sixteenth, Rhythm::Sixteenth, Rhythm::Eigth],
            ],
            weights: [50, 10, 10, 10],
        },
        amount_of_beats: 1.,
        rest_probability: 0.3,
        ..MelodyExercise::default()
    };

    exercise.play();
}

#[test]
fn fast_no_leaps() {
    let exercise = MelodyExercise {
        bpm: 120,
        loops: 10,
        root: wmidi::Note::C4,
        scale: scales::MAJOR.to_vec(),
        steps: RandomThings {
            things: [0, 1],
            weights: [40, 60],
        },
        rhythms: RandomThings {
            things: [
                vec![Rhythm::Eigth, Rhythm::Eigth],
                vec![Rhythm::Quarter],
                vec![
                    Rhythm::QuarterTriplet,
                    Rhythm::QuarterTriplet,
                    Rhythm::QuarterTriplet,
                ],
            ],
            weights: [50, 5, 20],
        },
        amount_of_beats: 3.,
        rest_probability: 0.0,
        ..MelodyExercise::default()
    };

    exercise.play();
}
