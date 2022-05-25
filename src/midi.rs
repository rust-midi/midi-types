//! This module contains data types to represent the different messages that can be sent over MIDI.

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

const NOTE_OFF_END: u8 = NOTE_OFF + 0x0F;
const NOTE_ON_END: u8 = NOTE_ON + 0x0F;
const KEY_PRESSURE_END: u8 = KEY_PRESSURE + 0x0F;
const CONTROL_CHANGE_END: u8 = CONTROL_CHANGE + 0x0F;
const PITCH_BEND_CHANGE_END: u8 = PITCH_BEND_CHANGE + 0x0F;
const PROGRAM_CHANGE_END: u8 = PROGRAM_CHANGE + 0x0F;
const CHANNEL_PRESSURE_END: u8 = CHANNEL_PRESSURE + 0x0F;

use core::convert::TryFrom;

use status::*;

/// An enum with variants for all possible Midi messages.
#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
/// Errors rendering
pub enum RenderError {
    ///Input buffer wasn't long enough to render message
    BufferTooShort,
}

#[derive(Debug, PartialEq, Clone)]
/// Errors parsing
pub enum ParseError {
    ///Input buffer wasn't long enough to parse anything
    BufferTooShort,
    ///Couldn't find a valid message
    MessageNotFound,
    ///Partial valid message but
    PartialData,
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

    //helper to render 3 byte messages
    fn chan3byte<T0: Into<u8> + Copy, T1: Into<u8> + Copy, C: Into<u8> + Copy>(
        buf: &mut [u8],
        status: u8,
        chan: &C,
        d0: &T0,
        d1: &T1,
    ) -> Result<usize, RenderError> {
        if buf.len() >= 3 {
            let chan: u8 = (*chan).into();
            let status = status | chan;
            for (o, i) in buf.iter_mut().zip(&[status, (*d0).into(), (*d1).into()]) {
                *o = *i;
            }
            Ok(3)
        } else {
            Err(RenderError::BufferTooShort)
        }
    }

    //helper to render 2 byte messages
    fn chan2byte<T0: Into<u8> + Copy, C: Into<u8> + Copy>(
        buf: &mut [u8],
        status: u8,
        chan: &C,
        d0: &T0,
    ) -> Result<usize, RenderError> {
        if buf.len() >= 2 {
            let chan: u8 = (*chan).into();
            let status = status | chan;
            for (o, i) in buf.iter_mut().zip(&[status, (*d0).into()]) {
                *o = *i;
            }
            Ok(2)
        } else {
            Err(RenderError::BufferTooShort)
        }
    }

    //helper to render 1 byte messages
    fn chan1byte(buf: &mut [u8], status: u8) -> Result<usize, RenderError> {
        if buf.len() >= 1 {
            buf[0] = status;
            Ok(1)
        } else {
            Err(RenderError::BufferTooShort)
        }
    }

    /// Render into a raw byte buffer, return the number of bytes rendered
    pub fn render(&self, buf: &mut [u8]) -> Result<usize, RenderError> {
        match self {
            Self::NoteOff(c, n, v) => Self::chan3byte(buf, NOTE_OFF, c, n, v),
            Self::NoteOn(c, n, v) => Self::chan3byte(buf, NOTE_ON, c, n, v),
            Self::KeyPressure(c, n, v) => Self::chan3byte(buf, KEY_PRESSURE, c, n, v),
            Self::ControlChange(c, n, v) => Self::chan3byte(buf, CONTROL_CHANGE, c, n, v),
            Self::PitchBendChange(c, v) => {
                let (v0, v1): (u8, u8) = (*v).into();
                Self::chan3byte(buf, PITCH_BEND_CHANGE, c, &v0, &v1)
            }
            Self::SongPositionPointer(v) => {
                let (v0, v1): (u8, u8) = (*v).into();
                Self::chan3byte(buf, SONG_POSITION_POINTER, &0, &v0, &v1)
            }
            Self::ProgramChange(c, p) => Self::chan2byte(buf, PROGRAM_CHANGE, c, p),
            Self::ChannelPressure(c, p) => Self::chan2byte(buf, CHANNEL_PRESSURE, c, p),
            Self::QuarterFrame(q) => Self::chan2byte(buf, QUARTER_FRAME, &0, q),
            Self::SongSelect(s) => Self::chan2byte(buf, SONG_SELECT, &0, s),
            Self::TuneRequest => Self::chan1byte(buf, TUNE_REQUEST),
            Self::TimingClock => Self::chan1byte(buf, TIMING_CLOCK),
            Self::Start => Self::chan1byte(buf, START),
            Self::Continue => Self::chan1byte(buf, CONTINUE),
            Self::Stop => Self::chan1byte(buf, STOP),
            Self::ActiveSensing => Self::chan1byte(buf, ACTIVE_SENSING),
            Self::Reset => Self::chan1byte(buf, RESET),
        }
    }

    //parse helper guard
    fn check_len<F: Fn() -> Result<Self, ParseError>>(
        buf: &[u8],
        len: usize,
        func: F,
    ) -> Result<Self, ParseError> {
        if buf.len() >= len {
            func()
        } else {
            Err(ParseError::BufferTooShort)
        }
    }

    fn parse(buf: &[u8]) -> Result<Self, ParseError> {
        let chan = |status: u8| -> Channel { Channel::from(status & 0x0F) };
        match buf[0] {
            //1 byte
            TUNE_REQUEST => Ok(Self::TuneRequest),
            TIMING_CLOCK => Ok(Self::TimingClock),
            START => Ok(Self::Start),
            CONTINUE => Ok(Self::Continue),
            STOP => Ok(Self::Stop),
            ACTIVE_SENSING => Ok(Self::ActiveSensing),
            RESET => Ok(Self::Reset),

            //2 byte
            s @ PROGRAM_CHANGE..=PROGRAM_CHANGE_END => Self::check_len(buf, 2, || {
                Ok(Self::ProgramChange(chan(s), Program::from(buf[1])))
            }),
            s @ CHANNEL_PRESSURE..=CHANNEL_PRESSURE_END => Self::check_len(buf, 2, || {
                Ok(Self::ChannelPressure(chan(s), Value7::from(buf[1])))
            }),
            QUARTER_FRAME => Self::check_len(buf, 2, || {
                Ok(Self::QuarterFrame(QuarterFrame::from(buf[1])))
            }),
            SONG_SELECT => Self::check_len(buf, 2, || Ok(Self::SongSelect(Value7::from(buf[1])))),

            //3 byte
            s @ NOTE_OFF..=NOTE_OFF_END => Self::check_len(buf, 3, || {
                Ok(Self::NoteOff(
                    chan(s),
                    Note::from(buf[1]),
                    Value7::from(buf[2]),
                ))
            }),
            s @ NOTE_ON..=NOTE_ON_END => Self::check_len(buf, 3, || {
                Ok(Self::NoteOn(
                    chan(s),
                    Note::from(buf[1]),
                    Value7::from(buf[2]),
                ))
            }),
            s @ KEY_PRESSURE..=KEY_PRESSURE_END => Self::check_len(buf, 3, || {
                Ok(Self::KeyPressure(
                    chan(s),
                    Note::from(buf[1]),
                    Value7::from(buf[2]),
                ))
            }),
            s @ CONTROL_CHANGE..=CONTROL_CHANGE_END => Self::check_len(buf, 3, || {
                Ok(Self::ControlChange(
                    chan(s),
                    Control::from(buf[1]),
                    Value7::from(buf[2]),
                ))
            }),
            s @ PITCH_BEND_CHANGE..=PITCH_BEND_CHANGE_END => Self::check_len(buf, 3, || {
                Ok(Self::PitchBendChange(
                    chan(s),
                    Value14::from((buf[1], buf[2])),
                ))
            }),
            SONG_POSITION_POINTER => Self::check_len(buf, 3, || {
                Ok(Self::SongPositionPointer(Value14::from((buf[1], buf[2]))))
            }),

            _ => Err(ParseError::MessageNotFound),
        }
    }
}

impl TryFrom<&[u8]> for MidiMessage {
    type Error = ParseError;
    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        if buf.len() == 0 {
            Err(ParseError::BufferTooShort)
        } else {
            Self::parse(buf)
        }
    }
}

/// Represents a midi note number where 0 corresponds to C-2 and 127 corresponds to G8,
/// C4 is 72
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Note(u8);

impl From<u8> for Note {
    fn from(note: u8) -> Self {
        assert!(note <= 127);
        Note(note)
    }
}

impl Into<u8> for Note {
    fn into(self) -> u8 {
        self.0
    }
}

/// Represents a Midi channel, Midi channels can range from 0 to 15, but are represented as 1 based
/// values Channel 1 to 16
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Channel(u8);

impl From<u8> for Channel {
    fn from(channel: u8) -> Self {
        assert!(channel <= 15);
        Channel(channel)
    }
}

impl Into<u8> for Channel {
    fn into(self) -> u8 {
        self.0
    }
}

/// A Midi controller number
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Control(u8);

impl From<u8> for Control {
    fn from(control: u8) -> Self {
        assert!(control <= 127);
        Control(control)
    }
}

impl Into<u8> for Control {
    fn into(self) -> u8 {
        self.0
    }
}

/// A Midi program number, these usually correspond to presets on Midi devices
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Program(u8);

impl From<u8> for Program {
    fn from(value: u8) -> Self {
        assert!(value <= 127);
        Program(value)
    }
}

impl Into<u8> for Program {
    fn into(self) -> u8 {
        self.0
    }
}

/// A 7 bit Midi data value stored in an unsigned 8 bit integer, the msb is always 0
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Value7(u8);

impl From<u8> for Value7 {
    fn from(value: u8) -> Self {
        Value7(value)
    }
}

impl Into<u8> for Value7 {
    fn into(self) -> u8 {
        self.0
    }
}

/// A 14 bit Midi value stored as two 7 bit Midi data values, where the msb is always 0 to signify
/// that this is a data value.
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Value14(u8, u8);

impl From<(u8, u8)> for Value14 {
    fn from(value: (u8, u8)) -> Self {
        Value14(value.0, value.1)
    }
}

impl Into<(u8, u8)> for Value14 {
    fn into(self) -> (u8, u8) {
        (self.0, self.1)
    }
}

impl From<u16> for Value14 {
    fn from(value: u16) -> Self {
        Value14(((value & 0x3f80) >> 7) as u8, (value & 0x007f) as u8)
    }
}

impl Into<u16> for Value14 {
    fn into(self) -> u16 {
        (self.0 as u16) * 128 + self.1 as u16
    }
}

/// The SMPTE type used. This indicates the number of frames per second
#[derive(Debug, PartialEq, Copy, Clone)]
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

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct QuarterFrame(u8);

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

impl From<u8> for QuarterFrame {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl Into<u8> for QuarterFrame {
    fn into(self) -> u8 {
        self.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_1BYTE: [MidiMessage; 7] = [
        MidiMessage::TuneRequest,
        MidiMessage::TimingClock,
        MidiMessage::Start,
        MidiMessage::Continue,
        MidiMessage::Stop,
        MidiMessage::ActiveSensing,
        MidiMessage::Reset,
    ];
    const TEST_2BYTE: [MidiMessage; 4] = [
        MidiMessage::ProgramChange(Channel(0), Program(0)),
        MidiMessage::ChannelPressure(Channel(1), Value7(2)),
        MidiMessage::QuarterFrame(QuarterFrame(23)),
        MidiMessage::SongSelect(Value7(3)),
    ];
    const TEST_3BYTE: [MidiMessage; 6] = [
        MidiMessage::NoteOff(Channel(2), Note(3), Value7(1)),
        MidiMessage::NoteOn(Channel(3), Note(120), Value7(120)),
        MidiMessage::KeyPressure(Channel(3), Note(120), Value7(1)),
        MidiMessage::ControlChange(Channel(5), Control(23), Value7(23)),
        MidiMessage::PitchBendChange(Channel(15), Value14(23, 23)),
        MidiMessage::SongPositionPointer(Value14(0, 0)),
    ];

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
    fn render_err() {
        let mut buf0: [u8; 0] = [];
        let mut buf1 = [0];
        let mut buf2 = [0, 0];
        for v in TEST_1BYTE.iter() {
            assert_eq!(
                Err(RenderError::BufferTooShort),
                v.render(&mut buf0),
                "{:?}",
                v
            );
        }

        for v in TEST_2BYTE {
            assert_eq!(
                Err(RenderError::BufferTooShort),
                v.render(&mut buf0),
                "{:?}",
                v
            );
            assert_eq!(
                Err(RenderError::BufferTooShort),
                v.render(&mut buf1),
                "{:?}",
                v
            );
        }

        for v in TEST_3BYTE {
            assert_eq!(
                Err(RenderError::BufferTooShort),
                v.render(&mut buf0),
                "{:?}",
                v
            );
            assert_eq!(
                Err(RenderError::BufferTooShort),
                v.render(&mut buf1),
                "{:?}",
                v
            );
            assert_eq!(
                Err(RenderError::BufferTooShort),
                v.render(&mut buf2),
                "{:?}",
                v
            );
        }
    }

    #[test]
    fn render_ok() {
        let mut buf1 = [0];
        let mut buf2 = [0, 0];
        let mut buf3 = [0, 0, 0];
        let mut buf100 = [0; 100];
        for v in TEST_1BYTE.iter() {
            assert_eq!(Ok(1), v.render(&mut buf1), "{:?}", v);
            assert_eq!(Ok(1), v.render(&mut buf2), "{:?}", v);
            assert_eq!(Ok(1), v.render(&mut buf100), "{:?}", v);
        }

        for v in TEST_2BYTE {
            assert_eq!(Ok(2), v.render(&mut buf2), "{:?}", v);
            assert_eq!(Ok(2), v.render(&mut buf100), "{:?}", v);
        }

        for v in TEST_3BYTE {
            assert_eq!(Ok(3), v.render(&mut buf3), "{:?}", v);
            assert_eq!(Ok(3), v.render(&mut buf100), "{:?}", v);
        }
    }

    #[test]
    fn parse_rendered() {
        let mut buf1 = [0];
        let mut buf2 = [0, 0];
        let mut buf3 = [0, 0, 0];
        let mut buf100 = [0; 100];
        for v in TEST_1BYTE.iter() {
            assert_eq!(Ok(1), v.render(&mut buf1), "{:?}", v);
            assert_eq!(Ok(v.clone()), MidiMessage::try_from(buf1.as_slice()));
            assert_eq!(Ok(1), v.render(&mut buf2), "{:?}", v);
            assert_eq!(Ok(v.clone()), MidiMessage::try_from(buf2.as_slice()));
            assert_eq!(Ok(1), v.render(&mut buf100), "{:?}", v);
            assert_eq!(Ok(v.clone()), MidiMessage::try_from(buf100.as_slice()));
        }

        for v in TEST_2BYTE {
            assert_eq!(Ok(2), v.render(&mut buf2), "{:?}", v);
            assert_eq!(Ok(v.clone()), MidiMessage::try_from(buf2.as_slice()));
            assert_eq!(Ok(2), v.render(&mut buf100), "{:?}", v);
            assert_eq!(Ok(v.clone()), MidiMessage::try_from(buf100.as_slice()));
        }

        for v in TEST_3BYTE {
            assert_eq!(Ok(3), v.render(&mut buf3), "{:?}", v);
            assert_eq!(Ok(v.clone()), MidiMessage::try_from(buf3.as_slice()));
            assert_eq!(Ok(3), v.render(&mut buf100), "{:?}", v);
            assert_eq!(Ok(v.clone()), MidiMessage::try_from(buf100.as_slice()));
        }
    }
}
