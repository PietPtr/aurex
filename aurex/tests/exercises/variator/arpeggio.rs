// use aurex::{
//     exercises::{play, variator::VariatorExercise, Exercise},
//     metronome::backbeat::BackbeatMetronome,
//     player::playtwice::PlayTwice,
//     random::RandomThings,
//     sequence::{ChannelNote, Play, Rhythm},
//     theory::{
//         scales::{scale_range, scale_range_offset, MAJOR},
//         Interval,
//     },
// };
// use wmidi::Note;

// #[test]
// fn variator_arpeggio() {
//     // TODO: A random arpeggio generator? convenient!
//     let root = Note::C2;
//     let base_scale = MAJOR.to_vec();
//     let extended_scale =
//         scale_range_offset(root, &base_scale, root, Interval::TwoOctaves.semitones());

//     let mut initial_melody = vec![];

//     // pick a random note to start the arpeggio on
//     let mut note_index = rand::random_range(0..base_scale.len());

//     for _ in 0..4 {
//         let note = *extended_scale.get(note_index).unwrap();
//         note_index += 2;
//         initial_melody.push(
//             ChannelNote {
//                 note: Play::Note(note),
//                 channel: wmidi::Channel::Ch1,
//             }
//             .with_duration(Rhythm::Quarter),
//         );
//     }

//     let exercise = PlayTwice {
//         exercise: VariatorExercise {
//             bpm: 78,
//             loops: 10,
//             root,
//             scale: base_scale,
//             melody: initial_melody,
//         },
//         metronome: BackbeatMetronome {},
//         loops: 10,
//     };

//     play(exercise);
// }
