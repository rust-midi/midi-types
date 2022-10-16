//! The MIDI note type represent midi note numbers

/// Represents a midi note number
///
/// # Note
/// * 12-tone english named note constants are calculated with 0 corresponding to C-2 and 127 to
/// G8, C4 is 72
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Note(u8);

#[allow(non_upper_case_globals)]
impl Note {
    pub const C2m: Self = Self::new(0 * 12 + 0);
    pub const Cs2m: Self = Self::new(0 * 12 + 1);
    pub const D2m: Self = Self::new(0 * 12 + 2);
    pub const Ds2m: Self = Self::new(0 * 12 + 3);
    pub const E2m: Self = Self::new(0 * 12 + 4);
    pub const F2m: Self = Self::new(0 * 12 + 5);
    pub const Fs2m: Self = Self::new(0 * 12 + 6);
    pub const G2m: Self = Self::new(0 * 12 + 7);
    pub const Gs2m: Self = Self::new(0 * 12 + 8);
    pub const A2m: Self = Self::new(0 * 12 + 9);
    pub const As2m: Self = Self::new(0 * 12 + 10);
    pub const B2m: Self = Self::new(0 * 12 + 11);

    pub const C1m: Self = Self::new(1 * 12 + 0);
    pub const Cs1m: Self = Self::new(1 * 12 + 1);
    pub const D1m: Self = Self::new(1 * 12 + 2);
    pub const Ds1m: Self = Self::new(1 * 12 + 3);
    pub const E1m: Self = Self::new(1 * 12 + 4);
    pub const F1m: Self = Self::new(1 * 12 + 5);
    pub const Fs1m: Self = Self::new(1 * 12 + 6);
    pub const G1m: Self = Self::new(1 * 12 + 7);
    pub const Gs1m: Self = Self::new(1 * 12 + 8);
    pub const A1m: Self = Self::new(1 * 12 + 9);
    pub const As1m: Self = Self::new(1 * 12 + 10);
    pub const B1m: Self = Self::new(1 * 12 + 11);

    pub const C0: Self = Self::new(2 * 12 + 0);
    pub const Cs0: Self = Self::new(2 * 12 + 1);
    pub const D0: Self = Self::new(2 * 12 + 2);
    pub const Ds0: Self = Self::new(2 * 12 + 3);
    pub const E0: Self = Self::new(2 * 12 + 4);
    pub const F0: Self = Self::new(2 * 12 + 5);
    pub const Fs0: Self = Self::new(2 * 12 + 6);
    pub const G0: Self = Self::new(2 * 12 + 7);
    pub const Gs0: Self = Self::new(2 * 12 + 8);
    pub const A0: Self = Self::new(2 * 12 + 9);
    pub const As0: Self = Self::new(2 * 12 + 10);
    pub const B0: Self = Self::new(2 * 12 + 11);

    pub const C1: Self = Self::new(3 * 12 + 0);
    pub const Cs1: Self = Self::new(3 * 12 + 1);
    pub const D1: Self = Self::new(3 * 12 + 2);
    pub const Ds1: Self = Self::new(3 * 12 + 3);
    pub const E1: Self = Self::new(3 * 12 + 4);
    pub const F1: Self = Self::new(3 * 12 + 5);
    pub const Fs1: Self = Self::new(3 * 12 + 6);
    pub const G1: Self = Self::new(3 * 12 + 7);
    pub const Gs1: Self = Self::new(3 * 12 + 8);
    pub const A1: Self = Self::new(3 * 12 + 9);
    pub const As1: Self = Self::new(3 * 12 + 10);
    pub const B1: Self = Self::new(3 * 12 + 11);

    pub const C2: Self = Self::new(4 * 12 + 0);
    pub const Cs2: Self = Self::new(4 * 12 + 1);
    pub const D2: Self = Self::new(4 * 12 + 2);
    pub const Ds2: Self = Self::new(4 * 12 + 3);
    pub const E2: Self = Self::new(4 * 12 + 4);
    pub const F2: Self = Self::new(4 * 12 + 5);
    pub const Fs2: Self = Self::new(4 * 12 + 6);
    pub const G2: Self = Self::new(4 * 12 + 7);
    pub const Gs2: Self = Self::new(4 * 12 + 8);
    pub const A2: Self = Self::new(4 * 12 + 9);
    pub const As2: Self = Self::new(4 * 12 + 10);
    pub const B2: Self = Self::new(4 * 12 + 11);

    pub const C3: Self = Self::new(5 * 12 + 0);
    pub const Cs3: Self = Self::new(5 * 12 + 1);
    pub const D3: Self = Self::new(5 * 12 + 2);
    pub const Ds3: Self = Self::new(5 * 12 + 3);
    pub const E3: Self = Self::new(5 * 12 + 4);
    pub const F3: Self = Self::new(5 * 12 + 5);
    pub const Fs3: Self = Self::new(5 * 12 + 6);
    pub const G3: Self = Self::new(5 * 12 + 7);
    pub const Gs3: Self = Self::new(5 * 12 + 8);
    pub const A3: Self = Self::new(5 * 12 + 9);
    pub const As3: Self = Self::new(5 * 12 + 10);
    pub const B3: Self = Self::new(5 * 12 + 11);

    pub const C4: Self = Self::new(6 * 12 + 0);
    pub const Cs4: Self = Self::new(6 * 12 + 1);
    pub const D4: Self = Self::new(6 * 12 + 2);
    pub const Ds4: Self = Self::new(6 * 12 + 3);
    pub const E4: Self = Self::new(6 * 12 + 4);
    pub const F4: Self = Self::new(6 * 12 + 5);
    pub const Fs4: Self = Self::new(6 * 12 + 6);
    pub const G4: Self = Self::new(6 * 12 + 7);
    pub const Gs4: Self = Self::new(6 * 12 + 8);
    pub const A4: Self = Self::new(6 * 12 + 9);
    pub const As4: Self = Self::new(6 * 12 + 10);
    pub const B4: Self = Self::new(6 * 12 + 11);

    pub const C5: Self = Self::new(7 * 12 + 0);
    pub const Cs5: Self = Self::new(7 * 12 + 1);
    pub const D5: Self = Self::new(7 * 12 + 2);
    pub const Ds5: Self = Self::new(7 * 12 + 3);
    pub const E5: Self = Self::new(7 * 12 + 4);
    pub const F5: Self = Self::new(7 * 12 + 5);
    pub const Fs5: Self = Self::new(7 * 12 + 6);
    pub const G5: Self = Self::new(7 * 12 + 7);
    pub const Gs5: Self = Self::new(7 * 12 + 8);
    pub const A5: Self = Self::new(7 * 12 + 9);
    pub const As5: Self = Self::new(7 * 12 + 10);
    pub const B5: Self = Self::new(7 * 12 + 11);

    pub const C6: Self = Self::new(8 * 12 + 0);
    pub const Cs6: Self = Self::new(8 * 12 + 1);
    pub const D6: Self = Self::new(8 * 12 + 2);
    pub const Ds6: Self = Self::new(8 * 12 + 3);
    pub const E6: Self = Self::new(8 * 12 + 4);
    pub const F6: Self = Self::new(8 * 12 + 5);
    pub const Fs6: Self = Self::new(8 * 12 + 6);
    pub const G6: Self = Self::new(8 * 12 + 7);
    pub const Gs6: Self = Self::new(8 * 12 + 8);
    pub const A6: Self = Self::new(8 * 12 + 9);
    pub const As6: Self = Self::new(8 * 12 + 10);
    pub const B6: Self = Self::new(8 * 12 + 11);

    pub const C7: Self = Self::new(9 * 12 + 0);
    pub const Cs7: Self = Self::new(9 * 12 + 1);
    pub const D7: Self = Self::new(9 * 12 + 2);
    pub const Ds7: Self = Self::new(9 * 12 + 3);
    pub const E7: Self = Self::new(9 * 12 + 4);
    pub const F7: Self = Self::new(9 * 12 + 5);
    pub const Fs7: Self = Self::new(9 * 12 + 6);
    pub const G7: Self = Self::new(9 * 12 + 7);
    pub const Gs7: Self = Self::new(9 * 12 + 8);
    pub const A7: Self = Self::new(9 * 12 + 9);
    pub const As7: Self = Self::new(9 * 12 + 10);
    pub const B7: Self = Self::new(9 * 12 + 11);

    pub const C8: Self = Self::new(10 * 12 + 0);
    pub const Cs8: Self = Self::new(10 * 12 + 1);
    pub const D8: Self = Self::new(10 * 12 + 2);
    pub const Ds8: Self = Self::new(10 * 12 + 3);
    pub const E8: Self = Self::new(10 * 12 + 4);
    pub const F8: Self = Self::new(10 * 12 + 5);
    pub const Fs8: Self = Self::new(10 * 12 + 6);
    pub const G8: Self = Self::new(10 * 12 + 7);

    /// The minimum note value
    pub const MIN: Self = Self::C2m;
    /// The maximum note value
    pub const MAX: Self = Self::G8;

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
        Self::new(note)
    }
}

impl From<Note> for u8 {
    fn from(value: Note) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn note_conv() {
        assert_eq!(127u8, Note::G8.into());
        assert_eq!(127u8, Note::MAX.into());
        assert_eq!(0u8, Note::MIN.into());
        assert_eq!(0u8, Note::C2m.into());
    }
}
