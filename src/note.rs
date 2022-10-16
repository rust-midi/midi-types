//! The MIDI note type represent midi note numbers

/// Represents a midi note number where 0 corresponds to C-2 and 127 corresponds to G8,
/// C4 is 72
#[derive(Debug, PartialEq, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Note(u8);

impl From<u8> for Note {
    fn from(note: u8) -> Self {
        Note(note.min(127))
    }
}

impl Into<u8> for Note {
    fn into(self) -> u8 {
        self.0
    }
}
