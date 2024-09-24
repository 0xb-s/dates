// src/lib.rs
//! A comprehensive date and time utility library for parsing, formatting, and manipulating dates.
//! This crate provides extensive functionality similar to popular date libraries, with added features and Rust-specific optimizations.

mod datex;
mod durationx;
mod formatter;
mod locale;
pub mod macros;
mod manipulator;
mod parser;
pub mod recurrence;
pub mod timezone;
pub mod utils;
pub use crate::datex::DateX;
pub use manipulator::DurationUnit;
