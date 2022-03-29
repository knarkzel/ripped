#![feature(test)]

extern crate test;

// Mods
pub mod replay;

// Error handling
pub use anyhow::Error;
pub use fehler::throws;

// Other
pub use replay::Replay;
pub use std::path::PathBuf;

// Parallell
use rayon::prelude::*;
