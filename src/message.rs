//! This module contains data types to represent the different messages that can be sent over MIDI.

use crate::Note;

/// An enum with variants for all possible Midi messages.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MidiMessage {
    // Channel voice messages
    /// Note Off message
    NoteOff(Channel, Note, Value7),

    /// Note on message
    NoteOn(Channel, Note, Value7),

    /// KeyPressure message for polyphonic aftertouch
    KeyPressure(Channel, Note, Value7),

    /// Control change message
    ControlChange(Channel, Control, Value7),

    /// Program change message
    ProgramChange(Channel, Program),

    /// Channel pressure message for channel aftertouch
    ChannelPressure(Channel, Value7),

    /// Pitch bend message
    PitchBendChange(Channel, Value14),

    // System common messages
    /// System exclusive message starts
    // SystemExclusive {
    //     /// The system exclusive manufacturer id, this is either a 1 byte or 3 byte number
    //     manufacturer_id: u32,
    // },

    /// System exclusive data is received
    // SystemExclusiveData (Value7),

    /// Signals the end of the system exclusive block
    // EndOfExclusive,

    /// Midi time code quarter frame
    QuarterFrame(QuarterFrame),

    /// Set the song position pointer
    SongPositionPointer(Value14),

    /// Specifies which sequence or song is to be played
    SongSelect(Value7),

    /// Tune all oscillators
    TuneRequest,

    // System real time messages
    /// Timing tick message
    TimingClock,

    /// Start message
    Start,

    /// Continue message
    Continue,

    /// Stop message
    Stop,

    /// Active sensing message
    ActiveSensing,

    /// Reset message
    Reset,
}

impl MidiMessage {
    /// The length of the rendered data, including the status
    pub fn len(&self) -> usize {
        match self {
            Self::NoteOff(..)
            | Self::NoteOn(..)
            | Self::KeyPressure(..)
            | Self::ControlChange(..)
            | Self::PitchBendChange(..)
            | Self::SongPositionPointer(..) => 3,
            Self::ProgramChange(..)
            | Self::ChannelPressure(..)
            | Self::QuarterFrame(..)
            | Self::SongSelect(..) => 2,
            Self::TuneRequest
            | Self::TimingClock
            | Self::Start
            | Self::Continue
            | Self::Stop
            | Self::ActiveSensing
            | Self::Reset => 1,
        }
    }
}

#[allow(missing_docs)]
/// Status byte constants
pub mod status {
    pub const NOTE_OFF: u8 = 0x80;
    pub const NOTE_ON: u8 = 0x90;
    pub const KEY_PRESSURE: u8 = 0xA0;
    pub const CONTROL_CHANGE: u8 = 0xB0;
    pub const PITCH_BEND_CHANGE: u8 = 0xE0;
    pub const SONG_POSITION_POINTER: u8 = 0xF2;
    pub const PROGRAM_CHANGE: u8 = 0xC0;
    pub const CHANNEL_PRESSURE: u8 = 0xD0;
    pub const QUARTER_FRAME: u8 = 0xF1;
    pub const SONG_SELECT: u8 = 0xF3;
    pub const TUNE_REQUEST: u8 = 0xF6;
    pub const TIMING_CLOCK: u8 = 0xF8;
    pub const START: u8 = 0xFA;
    pub const CONTINUE: u8 = 0xFB;
    pub const STOP: u8 = 0xFC;
    pub const ACTIVE_SENSING: u8 = 0xFE;
    pub const RESET: u8 = 0xFF;

    pub const SYSEX_START: u8 = 0xF0;
    pub const SYSEX_END: u8 = 0xF7;
}

/// Represents a Midi channel, Midi channels can range from 0 to 15, but are represented as 1 based
/// values Channel 1 to 16
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Channel(u8);

impl Channel {
    /// Create a new `Channel`
    ///
    /// # Arguments
    /// * `val` - the 0 based channel value
    ///
    /// # Note
    /// * The `val` will be clamped so it is in the 0..15 valid range.
    ///
    pub const fn new(val: u8) -> Self {
        Self(if val > 15 { 15 } else { val })
    }
}

impl From<u8> for Channel {
    fn from(channel: u8) -> Self {
        Channel(channel.min(15))
    }
}

impl From<Channel> for u8 {
    fn from(channel: Channel) -> u8 {
        channel.0
    }
}

/// A Midi controller number
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Control(u8);

impl Control {
    /// Create a new `Control`
    ///
    /// # Arguments
    /// * `val` - the control number value
    ///
    /// # Note
    /// * The `val` will be clamped so it is in the 0..127 valid range
    ///
    pub const fn new(val: u8) -> Self {
        Self(if val > 127 { 127 } else { val })
    }
}

impl From<u8> for Control {
    fn from(control: u8) -> Self {
        Control(control.min(127))
    }
}

impl From<Control> for u8 {
    fn from(control: Control) -> u8 {
        control.0
    }
}

/// A Midi program number, these usually correspond to presets on Midi devices
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Program(u8);

impl Program {
    /// Create a new `Program`
    ///
    /// # Arguments
    /// * `val` - the program number value
    ///
    /// # Note
    /// * The `val` will be clamped so it is in the 0..127 valid range
    ///
    pub const fn new(val: u8) -> Self {
        Self(if val > 127 { 127 } else { val })
    }
}

impl From<u8> for Program {
    fn from(value: u8) -> Self {
        assert!(value <= 127);
        Program(value)
    }
}

impl From<Program> for u8 {
    fn from(program: Program) -> u8 {
        program.0
    }
}

/// A 7 bit Midi data value stored in an unsigned 8 bit integer, the msb is always 0
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Value7(u8);

impl Value7 {
    /// Create a new `Value7`
    ///
    /// # Arguments
    /// * `val` - the value
    ///
    /// # Note
    /// * The `val` will be clamped so it is in the 0..127 valid range
    ///
    pub const fn new(val: u8) -> Self {
        Self(if val > 127 { 127 } else { val })
    }
}

impl From<u8> for Value7 {
    fn from(value: u8) -> Self {
        Value7(value.min(127))
    }
}

impl From<Value7> for u8 {
    fn from(value: Value7) -> u8 {
        value.0
    }
}

/// A 14 bit Midi value stored as two 7 bit Midi data values, where the msb is always 0 to signify
/// that this is a data value.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Value14(u8, u8);

impl Value14 {
    /// Create a new `Value14`
    ///
    /// # Arguments
    /// * `val` - the value
    ///
    /// # Note
    /// * The `val` will be clamped so it is in the 0..127 valid range
    ///
    pub const fn new(val: i16) -> Self {
        let value = if val <= -8192i16 {
            -8192i16
        } else if val >= 8191i16 {
            8191i16
        } else {
            val
        }
        .saturating_add(8192i16) as u16;
        Value14(((value & 0x3f80) >> 7) as u8, (value & 0x007f) as u8)
    }
}

impl From<(u8, u8)> for Value14 {
    fn from(value: (u8, u8)) -> Self {
        Value14(value.0.min(127), value.1.min(127))
    }
}

impl From<Value14> for (u8, u8) {
    fn from(value: Value14) -> (u8, u8) {
        (value.0, value.1)
    }
}

impl From<u16> for Value14 {
    fn from(value: u16) -> Self {
        Value14(((value & 0x3f80) >> 7) as u8, (value & 0x007f) as u8)
    }
}

impl From<Value14> for u16 {
    fn from(value: Value14) -> u16 {
        (value.0 as u16) * 128 + value.1 as u16
    }
}

///Convert from -8192i16..8191i16
impl From<i16> for Value14 {
    fn from(value: i16) -> Self {
        Self::new(value)
    }
}

///Convert into -8192i16..8191i16
impl From<Value14> for i16 {
    fn from(value: Value14) -> i16 {
        let v: u16 = value.into();
        (v as i16) - 8192i16
    }
}

///Convert from -1.0..1.0
impl From<f32> for Value14 {
    fn from(value: f32) -> Self {
        Self::from((value * if value > 0.0 { 8191.0 } else { 8192.0 }) as i16)
    }
}

///Convert into -1.0..1.0
impl From<Value14> for f32 {
    fn from(value: Value14) -> f32 {
        let v: i16 = value.into();
        let v = v as f32 / if v > 0 { 8191.0 } else { 8192.0 };
        v.clamp(-1.0, 1.0)
    }
}

/*
/// The SMPTE type used. This indicates the number of frames per second
#[derive(Debug, PartialEq, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmpteType {
    /// 24 frames per second
    Frames24,

    /// 25 frames per second
    Frames25,

    /// 29.97 frames per second
    DropFrame30,

    /// 30 frames per second
    Frames30,
}

/// The value of the quarter frame message, this message contains a message type and a value. Each
/// of these eight messages encodes a 4 bit part of the midi time code. As one of these is sent
/// every quarter frames, the complete midi time code is sent every two frames.
#[derive(Debug, PartialEq, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum QuarterFrameType {
    /// Frame number low nibble
    FramesLS,

    /// Frame count high nibble
    FramesMS,

    /// Seconds low nibble
    SecondsLS,

    /// Seconds high nibble
    SecondsMS,

    /// Minutes low nibble
    MinutesLS,

    /// Minutes high nibble
    MinutesMS,

    /// Hours low nibble
    HoursLS,

    /// Combined hours high nibble and smpte type (frames per second)
    HoursMS,
}
*/

/// A MIDI Quarter Frame value, used for sync.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct QuarterFrame(u8);

impl QuarterFrame {
    /// Create a new `QuarterFrame`
    ///
    /// # Arguments
    /// * `val` - the value
    ///
    /// # Note
    /// * The `val` will be clamped so it is in the 0..127 valid range
    ///
    pub const fn new(val: u8) -> Self {
        Self(if val > 127 { 127 } else { val })
    }
}

/*
impl QuarterFrame {
    pub fn frame_type(&self) -> QuarterFrameType {
        unimplemented!()
    }

    pub fn value(&self) -> u8 {
        unimplemented!()
    }

    pub fn smpte_type(&self) -> SmpteType {
        unimplemented!()
    }
}
*/

impl From<u8> for QuarterFrame {
    fn from(value: u8) -> Self {
        Self(value.min(127))
    }
}

impl From<QuarterFrame> for u8 {
    fn from(value: QuarterFrame) -> u8 {
        value.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_combine_7_bit_vals_into_14() {
        let val: Value14 = (0b01010101u8, 0b01010101u8).into();
        assert_eq!(0b0010101011010101u16, val.into());
    }

    #[test]
    fn should_split_14_bit_val_into_7() {
        let val: Value14 = 0b0011001100110011u16.into();
        assert_eq!((0b01100110u8, 0b00110011u8), val.into())
    }

    #[test]
    fn conversion_i16_14() {
        let val: Value14 = Value14::from(8191i16);
        assert_eq!((127, 127), val.into());
        assert_eq!(8191i16, val.into());
        assert_eq!(val, Value14::new(8191i16));

        //clamped
        let val: Value14 = Value14::from(8192i16);
        assert_eq!((127, 127), val.into());
        assert_eq!(8191i16, val.into());
        assert_eq!(val, Value14::new(8192i16));

        let val: Value14 = Value14::from(8190i16);
        assert_eq!((127, 126), val.into());
        assert_eq!(8190i16, val.into());

        let val: Value14 = Value14::from(-8192i16);
        assert_eq!((0, 0), val.into());
        assert_eq!(-8192i16, val.into());

        //clamped
        let val: Value14 = Value14::from(-8193i16);
        assert_eq!((0, 0), val.into());
        assert_eq!(-8192i16, val.into());

        let val: Value14 = Value14::from(0i16);
        assert_eq!((64, 0), val.into());
        assert_eq!(0i16, val.into());

        let val: Value14 = Value14::from(1i16);
        assert_eq!((64, 1), val.into());
        assert_eq!(1i16, val.into());
    }

    #[test]
    fn conversion_f32_14() {
        let val: Value14 = Value14::from(0.0f32);
        assert_eq!((64, 0), val.into());
        assert_eq!(0.0f32, val.into());

        let val: Value14 = Value14::from(1.0f32);
        assert_eq!((127, 127), val.into());
        assert_eq!(1.0f32, val.into());

        let val: Value14 = Value14::from(-1.0f32);
        assert_eq!((0, 0), val.into());
        assert_eq!(-1.0f32, val.into());
    }
}
