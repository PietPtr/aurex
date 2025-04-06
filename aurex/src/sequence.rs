use std::{
    collections::VecDeque,
    fmt,
    time::{Duration, Instant, SystemTime},
};

use chrono::Timelike;
use midir::MidiOutputConnection;
use rand::seq::IndexedRandom;
use wmidi::{MidiMessage, U7};

use crate::theory::intervals::Interval;

#[derive(Debug, Clone)]
pub struct Sequence {
    /// A collection of sequenced notes with monotonically increasing time field
    notes: VecDeque<SequencedNote>,
    pub bpm: f64, // TODO: turn into f64
    end_time: Duration,
}

impl Sequence {
    pub fn new(bpm: f64) -> Self {
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

    pub fn length_in_beats(&self) -> f64 {
        (self.end_time.as_secs_f64() / 60.) * self.bpm
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

    pub fn add_simultaneous(&mut self, note: NoteWithDuration) -> Duration {
        let last_note_time = self
            .notes
            .back()
            .map_or(Duration::from_millis(0), |n| n.time);

        let note = note.with_time(last_note_time);

        self.end_time = last_note_time + note.duration.time(self.bpm);

        self.notes.push_back(note);

        self.end_time
    }

    pub fn add_chord(
        &mut self,
        notes: &[Play],
        duration: Rhythm,
        channel: wmidi::Channel,
    ) -> Duration {
        for note in notes {
            let sequenced_note = SequencedNote {
                time: self.end_time,
                note: note.clone(),
                duration: duration.clone(),
                channel,
            };

            self.notes.push_back(sequenced_note);
        }

        self.end_time += duration.time(self.bpm);
        self.end_time
    }

    /// Add a note to the sequence at the last end time + the given beat offset, which is a number representing amount of beats.
    /// May be negative.
    pub fn add_beat_offset(&mut self, beat_offset: f64, note: NoteWithDuration) -> Duration {
        let ns_per_beat = 60_000_000_000. / self.bpm;
        let time = self.end_time.as_nanos() as i64 + (ns_per_beat * beat_offset) as i64;
        let time = Duration::from_nanos(time as u64);

        self.end_time = self.end_time.max(time + note.duration.time(self.bpm));

        let note = note.with_time(time);
        self.notes.push_back(note);

        self.end_time
    }

    /// Adds the given sequence to the current one, such that they play simultaneously.
    pub fn combine_simultaneous(mut self, mut other: Sequence) -> Self {
        while let Some(note) = other.notes.pop_front() {
            self.notes.push_back(note);
        }

        self.end_time = self.end_time.max(other.end_time);

        self
    }

    /// Adds the given sequence after the current one.
    pub fn combine_at_end(mut self, mut other: Sequence) -> Self {
        while let Some(mut note) = other.notes.pop_front() {
            note.time += self.end_time;
            self.notes.push_back(note);
        }

        self.end_time += other.end_time;

        self
    }

    /// Takes the current notes in the sequence and loops them a given amount of times.
    /// The end time is updated to reflect the looping.
    pub fn r#loop(mut self, loops: usize) -> Self {
        let mut notes = VecDeque::with_capacity(self.notes.len() * loops);

        for iteration in 0..loops {
            for note in self.notes.iter() {
                let mut note = note.clone();
                note.time += self.end_time * iteration as u32;
                notes.push_back(note);
            }
        }

        self.notes = notes;

        self.end_time *= loops as u32;

        self
    }

    /// Final function to call to play the constructed sequence over the given midi connection
    /// TODO: if qsynth not running, run it?
    pub fn play(&mut self, conn: &mut MidiOutputConnection) {
        println!("Playing sequence with {} notes.", self.notes.len());
        let end_date = SystemTime::now() + self.end_time;
        let datetime: chrono::DateTime<chrono::Local> = end_date.into();
        println!(
            "Done at {:02}:{:02}:{:02} ({:.0?})",
            datetime.hour(),
            datetime.minute(),
            datetime.second(),
            self.end_time
        );

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
                    time: sequenced_note.time + sequenced_note.duration.play_time(self.bpm),
                    message: note_off,
                });
            }
        }

        actions.sort_by(|a, b| a.time.cmp(&b.time));

        let start_time = Instant::now();
        for action in actions {
            // Busy wait until it's time for the next action
            while start_time.elapsed() < action.time {}

            conn.send(&action.message.to_vec()).unwrap();
        }
    }
}

#[derive(Debug)]
pub struct SequenceAction<'a> {
    time: Duration,
    message: MidiMessage<'a>,
}

pub struct ChannelNote {
    pub note: Play,
    pub channel: wmidi::Channel,
}

impl ChannelNote {
    pub fn with_duration(self, duration: Rhythm) -> NoteWithDuration {
        NoteWithDuration {
            note: self.note,
            duration,
            channel: self.channel,
        }
    }
}

#[derive(Debug, Clone)]
pub struct NoteWithDuration {
    pub note: Play,
    pub duration: Rhythm,
    pub channel: wmidi::Channel,
}

impl NoteWithDuration {
    pub fn with_time(self, time: Duration) -> SequencedNote {
        SequencedNote {
            time,
            note: self.note,
            duration: self.duration,
            channel: self.channel,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SequencedNote {
    pub time: Duration,
    pub note: Play,
    pub duration: Rhythm,
    pub channel: wmidi::Channel,
}

impl SequencedNote {}

#[derive(Clone)]
pub enum Play {
    Rest,
    Note(wmidi::Note),
    RandomNote(Vec<wmidi::Note>),
}

macro_rules! note {
    ($octave:expr, $note:ident) => {
        Self::Note(wmidi::Note::from_u8_lossy(
            ((wmidi::Note::$note as i8).saturating_add(12 * $octave)) as u8,
        ))
    };
}

impl Play {
    pub fn note(&self, notes: &[wmidi::Note]) -> Option<wmidi::Note> {
        match self {
            Play::Rest => None,
            Play::Note(note) => Some(*note),
            Play::RandomNote(vec) => {
                // TODO: ugly
                if vec.len() > 2 {
                    if let Some(note) = notes.last() {
                        let vec_to_choose_from: Vec<_> =
                            vec.iter().filter(|&e| e != note).collect();

                        vec_to_choose_from
                            .choose(&mut rand::rng())
                            .copied()
                            .copied()
                    } else {
                        vec.choose(&mut rand::rng()).copied()
                    }
                } else {
                    vec.choose(&mut rand::rng()).copied()
                }
            }
        }
    }

    /// Build a chord from a given root and the given intervals. Only works for Play::Note.
    /// e.g. to construct a maj7 pass in the intervals MajorThird, MinorThird, MajorThird
    pub fn chord_relative(&self, intervals: &[Interval]) -> Vec<Play> {
        let Self::Note(note) = self else {
            return vec![];
        };

        let note_number = *note as u8;
        let mut new_note_number = note_number;
        let mut notes = vec![];
        // for each interval, the next note is the previous one + that interval
        for interval in intervals {
            new_note_number += note_number + interval.semitones();
            notes.push(Play::Note(wmidi::Note::from_u8_lossy(new_note_number)));
        }

        notes
    }

    /// Build a chord from a given root and the constituent intervals. Only works for Play::Note.
    /// e.g. to construct a maj7 pass in the intervals MajorThird, PerfectFifth, MajorSeventh
    pub fn chord(&self, intervals: &[Interval]) -> Vec<Play> {
        let Self::Note(note) = self else {
            return vec![];
        };

        let note_number = *note as u8;
        let mut new_note_number = note_number;
        let mut notes = vec![];
        // for each interval, the next note is the previous one + that interval
        for interval in intervals {
            new_note_number += note_number + interval.semitones();
            notes.push(Play::Note(wmidi::Note::from_u8_lossy(new_note_number)));
        }

        notes
    }

    /// Add a duration to the note. Defaults the channel to Channel 1
    pub fn with_duration(self, duration: Rhythm) -> NoteWithDuration {
        NoteWithDuration {
            note: self,
            duration,
            channel: wmidi::Channel::Ch1,
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
            Play::Rest => f.write_str("Rest"),
            Play::Note(note) => f.debug_tuple("Note").field(note).finish(),
            Play::RandomNote(notes) => f.debug_tuple("RandomNote").field(notes).finish(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Rhythm {
    DoubleWhole,
    Whole,
    DottedHalf,
    Half,
    DottedQuarter,
    Quarter,
    Eighth,
    DottedEighth,
    QuarterTriplet,
    Sixteenth,
    Beats(f64),
    Staccato(Box<Rhythm>),
}

impl Rhythm {
    pub fn time(&self, bpm: f64) -> Duration {
        let ns_per_beat = (60_000_000_000.0 / bpm) as u64;
        Duration::from_nanos(match self {
            Rhythm::DoubleWhole => 8 * ns_per_beat,
            Rhythm::Whole => 4 * ns_per_beat,
            Rhythm::DottedHalf => 3 * ns_per_beat,
            Rhythm::Half => 2 * ns_per_beat,
            Rhythm::DottedQuarter => (1.5 * ns_per_beat as f64) as u64,
            Rhythm::Quarter => ns_per_beat,
            Rhythm::DottedEighth => (0.75 * ns_per_beat as f64) as u64,
            Rhythm::Eighth => ns_per_beat / 2,
            Rhythm::QuarterTriplet => ns_per_beat / 3,
            Rhythm::Sixteenth => ns_per_beat / 4,
            Rhythm::Beats(beats) => (ns_per_beat as f64 * beats) as u64,
            Rhythm::Staccato(rhythm) => rhythm.time(bpm).as_nanos() as u64,
        })
    }

    pub fn play_time(&self, bpm: f64) -> Duration {
        match self {
            Rhythm::Staccato(rhythm) => {
                Duration::from_nanos(((rhythm.time(bpm).as_nanos() as f64) * 0.8) as u64)
            }
            _ => self.time(bpm),
        }
    }

    pub fn beats(&self) -> f64 {
        match self {
            Rhythm::DoubleWhole => 8.,
            Rhythm::Whole => 4.,
            Rhythm::DottedHalf => 3.,
            Rhythm::Half => 2.,
            Rhythm::DottedQuarter => 1.5,
            Rhythm::Quarter => 1.,
            Rhythm::DottedEighth => 0.75,
            Rhythm::Eighth => 0.5,
            Rhythm::QuarterTriplet => 1. / 3.,
            Rhythm::Sixteenth => 0.25,
            Rhythm::Beats(beats) => *beats,
            Rhythm::Staccato(rhythm) => rhythm.beats(),
        }
    }
}

#[macro_export]
macro_rules! staccato {
    ($rhythm:expr) => {
        Rhythm::Staccato(Box::new($rhythm))
    };
}
