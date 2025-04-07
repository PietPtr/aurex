use aurex::{
    exercises::{
        melody::MelodyExercise,
        play,
        rhythm_sweep::{RhythmSweepExercise, RhythmSweepExerciseDef},
    },
    metronome::{
        drummer::{BackbeatDrummer, TwelveEighthDrummer},
        metronomes::{EmphasisOneMetronome, TickEveryBeatMetronome},
    },
    player::{playonce::PlayOnce, playtwice::PlayTwice},
    random::RandomThings,
    sequence::Rhythm,
    theory::scales,
};

#[test]
fn random_sixteenths() {
    let exercise = PlayOnce {
        exercise: MelodyExercise {
            bpm: 120.,
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
            ..MelodyExercise::default()
        },
        metronome: TickEveryBeatMetronome {},
        loops: 10,
    };

    play(exercise);
}

#[test]
fn sweep_sixteenths() {
    let exercise: PlayTwice<RhythmSweepExercise, _> = PlayTwice {
        exercise: RhythmSweepExerciseDef {
            bpm: 105.,
            note: wmidi::Note::A1,
            subdivision_length: Rhythm::Sixteenth,
            sweep_step_length: Rhythm::Sixteenth,
            repeats_per_place: 1,
        }
        .into(),
        metronome: EmphasisOneMetronome {},
        loops: 16,
    };

    play(exercise);
}

#[test]
fn sweep_quarter_triplets() {
    let exercise: PlayTwice<RhythmSweepExercise, _> = PlayTwice {
        exercise: RhythmSweepExerciseDef {
            bpm: 105.,
            note: wmidi::Note::A1,
            subdivision_length: Rhythm::QuarterTriplet,
            sweep_step_length: Rhythm::QuarterTriplet,
            repeats_per_place: 1,
        }
        .into(),
        metronome: TwelveEighthDrummer {},
        loops: 16,
    };

    play(exercise);
}

#[test]
fn sweep_dotted_eights_over_sixteenths() {
    let exercise: PlayTwice<RhythmSweepExercise, BackbeatDrummer> = PlayTwice {
        exercise: RhythmSweepExerciseDef {
            bpm: 70.,
            note: wmidi::Note::A1,
            subdivision_length: Rhythm::DottedEighth,
            sweep_step_length: Rhythm::Sixteenth,
            repeats_per_place: 1,
        }
        .into(),
        metronome: BackbeatDrummer {},
        loops: 16,
    };

    play(exercise);
}
