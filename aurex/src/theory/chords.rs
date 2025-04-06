use super::intervals::Interval as I;

pub const MAJOR: &[I; 2] = &[I::MajorThird, I::PerfectFifth];
pub const MAJOR_SEVENTH: &[I; 3] = &[I::MajorThird, I::PerfectFifth, I::MajorSeventh];
pub const MINOR: &[I; 2] = &[I::MinorThird, I::PerfectFifth];
pub const MINOR_SEVENTH: &[I; 3] = &[I::MinorThird, I::PerfectFifth, I::MinorSeventh];
pub const DOMINANT_SEVENTH: &[I; 3] = &[I::MajorThird, I::PerfectFifth, I::MinorSeventh];
pub const HALF_DIMINISHED: &[I; 2] = &[I::MinorThird, I::DiminishedFifth];
pub const HALF_DIMINISHED_SEVENTH: &[I; 3] = &[I::MinorThird, I::DiminishedFifth, I::MinorSeventh];
