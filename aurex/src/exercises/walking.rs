pub struct WalkingBassExercise {
    pub bpm: u64,
    pub loops: u32,
    pub root: wmidi::Note,
    pub chord_loop: Vec<Vec<wmidi::Note>>,
}
