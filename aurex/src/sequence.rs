use std::{collections::VecDeque, fmt, thread, time::Duration};

use midir::MidiOutputConnection;
use rand::seq::IndexedRandom;
use wmidi::{MidiMessage, U7};

#[derive(Debug)]
pub struct Sequence {
    /// A collection of sequenced notes with monotonically increasing time field
    notes: VecDeque<SequencedNote>,
    bpm: u64,
    end_time: Duration,
}

impl Sequence {
    pub fn new(bpm: u64) -> Self {
        Self {
            notes: VecDeque::new(),
            bpm,
            end_time: Duration::from_millis(0),
        }
    }

    /// Adds the given note to the sequence and returns at which timestamp the sequence now ends.
    pub fn add(&mut self, note: SequencedNote) -> Duration {
        self.end_time += note.duration.time(self.bpm);
        self.notes.push_back(note);
        self.end_time
    }

    pub fn add_to_end(&mut self, note: NoteWithDuration) -> Duration {
        let note = SequencedNote {
            time: self.end_time,
            note: note.note,
            duration: note.duration,
            channel: note.channel,
        };

        self.end_time += note.duration.time(self.bpm);

        self.notes.push_back(note);

        self.end_time
    }

    /// Add a note to the sequence at the last end time + the given beat offset, which is a number representing amount of beats.
    /// May be negative.
    pub fn add_beat_offset(&mut self, beat_offset: f64, note: NoteWithDuration) -> Duration {
        let ns_per_beat = 60_000_000_000 / self.bpm;
        let time = self.end_time.as_nanos() as i64 + (ns_per_beat as f64 * beat_offset) as i64;
        let time = Duration::from_nanos(time as u64);

        self.end_time = self.end_time.max(time + note.duration.time(self.bpm));

        let note = SequencedNote {
            time,
            note: note.note,
            duration: note.duration,
            channel: note.channel,
        };
        self.notes.push_back(note);

        self.end_time
    }

    pub fn play(&mut self, conn: &mut MidiOutputConnection) {
        println!("Playing sequence with {} notes.", self.notes.len());

        let mut actions = Vec::<SequenceAction>::new();
        let mut notes = Vec::with_capacity(self.notes.len());

        while let Some(sequenced_note) = self.notes.pop_front() {
            let resolved_note = sequenced_note.note.note(&notes);

            if let Some(note) = resolved_note {
                notes.push(note);

                let velocity = U7::from_u8_lossy(50);
                let note_on = MidiMessage::NoteOn(sequenced_note.channel, note, velocity);
                let note_off = MidiMessage::NoteOff(sequenced_note.channel, note, velocity);

                actions.push(SequenceAction {
                    time: sequenced_note.time,
                    message: note_on,
                });

                actions.push(SequenceAction {
                    time: sequenced_note.time + sequenced_note.duration.time(self.bpm),
                    message: note_off,
                });
            }
        }

        actions.sort_by(|a, b| a.time.cmp(&b.time));

        let mut time = Duration::from_millis(0);
        for action in actions {
            println!("{:?}", action);
            dbg!(action.time, time);
            let sleep_time = action.time - time;
            thread::sleep(sleep_time);
            conn.send(&action.message.to_vec()).unwrap();
            time = action.time;
        }
    }
}

#[derive(Debug)]
pub struct SequenceAction<'a> {
    time: Duration,
    message: MidiMessage<'a>,
}

pub struct NoteWithDuration {
    pub note: Play,
    pub duration: NoteDuration,
    pub channel: wmidi::Channel,
}

#[derive(Debug)]
pub struct SequencedNote {
    pub time: Duration,
    pub note: Play,
    pub duration: NoteDuration,
    pub channel: wmidi::Channel,
}

impl SequencedNote {}

pub enum Play {
    Note(wmidi::Note),
    RandomNote(Vec<wmidi::Note>),
    ClosureNote(Box<dyn Fn(&Vec<wmidi::Note>) -> Option<wmidi::Note>>),
}

macro_rules! note {
    ($octave:expr, $note:ident) => {
        Self::Note(wmidi::Note::from_u8_lossy(
            ((wmidi::Note::$note as i8).saturating_add(12 * $octave)) as u8,
        ))
    };
}

impl Play {
    pub fn note(&self, notes: &Vec<wmidi::Note>) -> Option<wmidi::Note> {
        match self {
            Play::Note(note) => Some(*note),
            Play::RandomNote(vec) => vec.choose(&mut rand::rng()).copied(),
            Play::ClosureNote(function) => function(notes),
        }
    }

    pub fn a_flat(octave: i8) -> Self {
        note!(octave, Ab0)
    }

    pub fn a(octave: i8) -> Self {
        note!(octave, A0)
    }

    pub fn a_sharp(octave: i8) -> Self {
        note!(octave, ASharp0)
    }

    pub fn b_flat(octave: i8) -> Self {
        note!(octave, Bb0)
    }

    pub fn b(octave: i8) -> Self {
        note!(octave, B0)
    }

    pub fn c(octave: i8) -> Self {
        note!(octave, C0)
    }

    pub fn c_sharp(octave: i8) -> Self {
        note!(octave, CSharp0)
    }

    pub fn d_flat(octave: i8) -> Self {
        note!(octave, Db0)
    }

    pub fn d(octave: i8) -> Self {
        note!(octave, D0)
    }

    pub fn d_sharp(octave: i8) -> Self {
        note!(octave, DSharp0)
    }

    pub fn e_flat(octave: i8) -> Self {
        note!(octave, Eb0)
    }

    pub fn e(octave: i8) -> Self {
        note!(octave, E0)
    }

    pub fn f(octave: i8) -> Self {
        note!(octave, F0)
    }

    pub fn f_sharp(octave: i8) -> Self {
        note!(octave, FSharp0)
    }

    pub fn g_flat(octave: i8) -> Self {
        note!(octave, Gb0)
    }

    pub fn g(octave: i8) -> Self {
        note!(octave, G0)
    }

    pub fn g_sharp(octave: i8) -> Self {
        note!(octave, GSharp0)
    }
}

impl fmt::Debug for Play {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Play::Note(note) => f.debug_tuple("Note").field(note).finish(),
            Play::RandomNote(notes) => f.debug_tuple("RandomNote").field(notes).finish(),
            Play::ClosureNote(_) => f.write_str("ClosureNote(<function>)"),
        }
    }
}

#[derive(Debug)]
pub enum NoteDuration {
    Whole,
    Half,
    Quarter,
    Eigth,
    QuarterTriplet,
    Sixteenth,
}

impl NoteDuration {
    pub fn time(&self, bpm: u64) -> Duration {
        let ns_per_beat = 60_000_000_000 / bpm;
        Duration::from_nanos(match self {
            NoteDuration::Whole => 4 * ns_per_beat,
            NoteDuration::Half => 2 * ns_per_beat,
            NoteDuration::Quarter => 1 * ns_per_beat,
            NoteDuration::Eigth => ns_per_beat / 2,
            NoteDuration::QuarterTriplet => ns_per_beat / 3,
            NoteDuration::Sixteenth => ns_per_beat / 4,
        })
    }
}
