//! Text fitting for RPG Maker MZ: lays translated dialogue and descriptions
//! out the way the engine draws them (game font metrics, portraits, box
//! height, page breaks), so long translations never run off the box.
//!
//! Applied at export only: translations in the database stay line-for-line.

pub mod font;
pub mod geometry;
