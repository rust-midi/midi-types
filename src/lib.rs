#![no_std]
#[warn(missing_debug_implementations, missing_docs)]
mod midi;

pub use midi::{status, Channel, Control, MidiMessage, Note, Program};
