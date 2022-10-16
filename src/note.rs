//! The MIDI note type represent midi note numbers

/// Represents a midi note number where 0 corresponds to C-2 and 127 corresponds to G8,
/// C4 is 72
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
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

impl From<Note> for u8 {
    fn from(value: Note) -> Self {
        value.0
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum NoteName {
    C,
    Cs,
    D,
    Ds,
    E,
    F,
    Fs,
    G,
    Gs,
    A,
    As,
    B,
}

impl From<NoteName> for u8 {
    fn from(value: NoteName) -> Self {
        value as u8
    }
}

#[allow(non_upper_case_globals)]
impl NoteName {
    pub const Db: NoteName = NoteName::Cs;
    pub const Eb: NoteName = NoteName::Ds;
    pub const Gb: NoteName = NoteName::Fs;
    pub const Ab: NoteName = NoteName::Gs;
    pub const Bb: NoteName = NoteName::As;
}

impl From<Note> for (NoteName, i8) {
    fn from(note: Note) -> Self {
        let octave: i8 = note.0 as i8 / 12 - 2;
        let note_name = match note.0 % 12 {
            name if name == NoteName::C as u8 => NoteName::C,
            name if name == NoteName::Cs as u8 => NoteName::Cs,
            name if name == NoteName::D as u8 => NoteName::D,
            name if name == NoteName::Ds as u8 => NoteName::Ds,
            name if name == NoteName::E as u8 => NoteName::E,
            name if name == NoteName::F as u8 => NoteName::F,
            name if name == NoteName::Fs as u8 => NoteName::Fs,
            name if name == NoteName::G as u8 => NoteName::G,
            name if name == NoteName::Gs as u8 => NoteName::Gs,
            name if name == NoteName::A as u8 => NoteName::A,
            name if name == NoteName::As as u8 => NoteName::As,
            name if name == NoteName::B as u8 => NoteName::B,
            _ => panic!("This code should be unreachable"), // Out of bound values should not occur because of the %12
        };

        (note_name, octave)
    }
}

pub enum MidiConversionError {
    OutOfBoundsError,
}

impl TryFrom<(NoteName, i8)> for Note {
    type Error = MidiConversionError;

    fn try_from(value: (NoteName, i8)) -> Result<Self, Self::Error> {
        let (name, octave) = value;

        if !(-2i8..=8).contains(&octave) {
            Err(MidiConversionError::OutOfBoundsError)?
        }

        let number = name as u8 + (octave as u8 + 2) * 12;

        if number > 127 {
            Err(MidiConversionError::OutOfBoundsError)?
        }

        Ok(number.into())
    }
}
