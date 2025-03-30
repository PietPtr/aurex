use aurex::{
    exercises::{melody::MelodyExercise, play},
    metronome::backbeat::BackbeatMetronome,
    player::playonce::PlayOnce,
    random::RandomThings,
    sequence::Rhythm,
    staccato,
    theory::{scales, Interval},
};

#[test]
fn melody() {
    let mut scale = scales::TWO_OCTAVE_MAJOR_PENTATONIC.to_vec();
    scale.push(Interval::Octave);

    let exercise = PlayOnce {
        exercise: MelodyExercise {
            bpm: 65.,
            root: wmidi::Note::A1,
            scale,
            steps: RandomThings {
                things: [-2, -1, 0, 1, 2, 3],
                weights: [3, 10, 10, 40, 40, 0],
            },
            rhythms: RandomThings {
                things: [
                    vec![Rhythm::Quarter],
                    vec![Rhythm::Eighth],
                    vec![Rhythm::Eighth, Rhythm::Eighth],
                    vec![Rhythm::DottedQuarter],
                    vec![
                        Rhythm::QuarterTriplet,
                        Rhythm::QuarterTriplet,
                        Rhythm::QuarterTriplet,
                    ],
                ],
                weights: [50, 50, 15, 2, 5],
            },
            amount_of_beats: 2.5,
            ..MelodyExercise::default()
        },
        metronome: BackbeatMetronome {},
        loops: 10,
    };

    play(exercise);
}

#[test]
fn fast_no_leaps() {
    let exercise = PlayOnce {
        exercise: MelodyExercise {
            bpm: 100.,
            root: wmidi::Note::F1,
            scale: scales::TWO_OCTAVE_MAJOR.to_vec(),
            steps: RandomThings {
                things: [0, 1],
                weights: [40, 70],
            },
            rhythms: RandomThings {
                things: [
                    vec![staccato!(Rhythm::Eighth), staccato!(Rhythm::Eighth)],
                    vec![staccato!(Rhythm::Quarter)],
                    vec![
                        staccato!(Rhythm::QuarterTriplet),
                        staccato!(Rhythm::QuarterTriplet),
                        staccato!(Rhythm::QuarterTriplet),
                    ],
                ],
                weights: [50, 5, 5],
            },
            amount_of_beats: 3.,
            ..MelodyExercise::default()
        },
        metronome: BackbeatMetronome {},
        loops: 10,
    };

    play(exercise);
}

// TODO: most of these exercises don't include the octave
// TODO: these exercises don't set the instrument at the start

#[test]
fn short_ascending() {
    let exercise = PlayOnce {
        exercise: MelodyExercise {
            bpm: 120.,
            root: wmidi::Note::F1,
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
        },
        metronome: BackbeatMetronome {},
        loops: 10,
    };

    play(exercise);
}

// TODO: four beat walking bass practice given a chord loop?

#[test]
fn four_beats() {
    let exercise = PlayOnce {
        exercise: MelodyExercise {
            bpm: 70.,
            root: wmidi::Note::C2,
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
        },
        metronome: BackbeatMetronome {},
        loops: 10,
    };

    play(exercise);
}
