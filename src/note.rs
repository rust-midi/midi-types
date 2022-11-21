//! The MIDI note type represent midi note numbers

/// Represents a midi note number where 0 corresponds to C-2 and 127 corresponds to G8,
/// C4 is 72
#[derive(Debug, PartialEq, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Note(u8);

impl Note {
    /// Create a new `Note`
    ///
    /// # Arguments
    /// * `val` - the note number value
    ///
    /// # Note
    /// * The `val` will be clamped so it is in the 0..127 valid range
    ///
    pub const fn new(val: u8) -> Self {
        Self(if val > 127 { 127 } else { val })
    }
}

impl From<u8> for Note {
    fn from(note: u8) -> Self {
        debug_assert!(note <= 127);
        Self::from(note)
    }
}

impl Into<u8> for Note {
    fn into(self) -> u8 {
        self.0
    }
}
