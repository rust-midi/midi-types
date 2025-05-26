//! Types for representing midi messages

#![no_std]
#![warn(
    missing_debug_implementations,
    missing_docs,
    clippy::cargo,
    clippy::nursery,
    clippy::cargo
)]

mod message;
mod note;

pub use message::*;
pub use note::*;
