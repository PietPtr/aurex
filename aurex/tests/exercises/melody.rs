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
        root: wmidi::Note::C4,
        scale: scales::MAJOR.to_vec(),
        steps: RandomThings {
            things: [-2, -1, 0, 1, 2],
            weights: [5, 10, 10, 40, 15],
        },
        rhythms: RandomThings {
            things: [
                vec![Rhythm::Quarter],
                vec![Rhythm::Eighth],
                vec![
                    Rhythm::QuarterTriplet,
                    Rhythm::QuarterTriplet,
                    Rhythm::QuarterTriplet,
                ],
            ],
            weights: [50, 45, 5],
        },
        rest_probability: 0.1,
        amount_of_beats: 3.,
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
        scale: scales::TWO_OCTAVE_MAJOR.to_vec(),
        steps: RandomThings {
            things: [0, 1],
            weights: [40, 60],
        },
        rhythms: RandomThings {
            things: [
                vec![Rhythm::Eighth, Rhythm::Eighth],
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

#[test]
fn short_ascending() {
    let exercise = MelodyExercise {
        bpm: 120,
        loops: 10,
        root: wmidi::Note::C4,
        scale: scales::MAJOR.to_vec(),
        steps: RandomThings {
            things: [1, 2, 3, 4, 5, 6],
            weights: [80, 70, 60, 50, 40, 30],
        },
        rhythms: RandomThings {
            things: [vec![Rhythm::Quarter, Rhythm::Quarter, Rhythm::Quarter]],
            weights: [1],
        },
        amount_of_beats: 3.,
        ..MelodyExercise::default()
    };

    exercise.play();
}

#[test]
fn four_beats() {
    let exercise = MelodyExercise {
        bpm: 120,
        loops: 10,
        root: wmidi::Note::C4,
        scale: scales::MAJOR.to_vec(),
        steps: RandomThings {
            things: [1, 2, 3, 4, 5, 6],
            weights: [80, 30, 20, 10, 10, 2],
        },
        rhythms: RandomThings {
            things: [vec![
                Rhythm::Quarter,
                Rhythm::Quarter,
                Rhythm::Quarter,
                Rhythm::Quarter,
            ]],
            weights: [1],
        },
        amount_of_beats: 4.,
        ..MelodyExercise::default()
    };

    exercise.play();
}
