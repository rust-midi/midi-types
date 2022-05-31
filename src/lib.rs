#![no_std]
#[warn(missing_debug_implementations, missing_docs)]
mod midi;
mod parser;

pub use {midi::*, parser::*};
