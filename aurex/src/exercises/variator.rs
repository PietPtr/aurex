// use crate::{
//     midi,
//     sequence::{NoteWithDuration, Play, Rhythm, Sequence},
//     staccato,
//     theory::{
//         scales::{scale, set_of_pitch_classes},
//         Interval,
//     },
// };

// use super::Exercise;

// pub struct VariatorExercise {
//     pub bpm: u64,
//     pub loops: u32,
//     pub root: wmidi::Note,
//     pub scale: Vec<Interval>,
//     pub melody: Vec<NoteWithDuration>,
// }

// impl Exercise for VariatorExercise {
//     fn play(self) {
//         let scale_as_notes = scale(self.root, &self.scale);
//         let pitch_classes = set_of_pitch_classes(scale_as_notes);

//         let replacements = vec![
//             Replacement {
//                 pattern: vec![Rhythm::Quarter],
//                 replacement: Box::new(|note| {
//                     let mut new_first_note = note.get(0).cloned().unwrap();
//                     let mut new_second_note = note.get(0).cloned().unwrap();

//                     new_first_note.duration = staccato!(Rhythm::Eighth);
//                     new_second_note.duration = Rhythm::Eighth;

//                     vec![new_first_note, new_second_note]
//                 }),
//             },
//             Replacement {
//                 pattern: vec![Rhythm::Quarter],
//                 replacement: Box::new(|note| {
//                     // TODO: getting extremely cumbersome, first refactor theory library
//                     // let mut note = note.get(0).cloned().unwrap();
//                     // let note_id = note.note;
//                     // let mut new_note = vec![];
//                     // loop {}
//                     todo!()
//                 }),
//             },
//         ];

//         let mut conn = midi::open_midi_connection("128:0");
//         midi::set_instrument(&mut conn, wmidi::Channel::Ch1, midi::FINGERED_BASS);

//         let sequence = Sequence::new(self.bpm);

//         for _ in 0..self.loops {

//             // take a random note in the melody
//             // do a rhythm replacement
//             // or a note replacement
//         }
//         // possible replacements:
//         // easy:
//         // change a quarter note in a staccato(eight) + eigth
//         // move a note by one step in the scale
//         // move a note somewhere else in the scale
//         // hard:
//         // move a note chromatically
//         // change an eigth into two staccato sixteenths
//         // change a quarter into a dotted eigth + sixteenth
//         // change a quarter into triplets
//     }
// }

// pub struct Replacement {
//     pattern: Vec<Rhythm>,
//     replacement: Box<dyn FnMut(Vec<NoteWithDuration>) -> Vec<NoteWithDuration>>,
// }
