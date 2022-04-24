use std::io::Read;

/// Get the indentation level in a MuON file.
///
/// Always returns 2, 3 or 4
fn indentation(data: &str) -> Option<u8> {
    // Limit to 1024 lines
    for line in data.lines().take(1024) {
        if !line.starts_with("  ") {
            continue;
        }
        if !line.get(2..)?.starts_with(" ") {
            return Some(2);
        } else if !line.get(3..)?.starts_with(" ") {
            return Some(3);
        } else if !line.get(4..)?.starts_with(" ") {
            return Some(4);
        } else {
            return None;
        }
    }
    None
}

#[derive(Debug)]
pub struct Time(String);

#[derive(Debug)]
pub struct Date(String);

#[derive(Debug)]
pub struct DateTime(Date, Time);

/// An arbitrarily sized integer.
#[derive(Debug)]
pub enum Int {
    /// Signed Byte
    ByteS(i8),
    /// Unsigned Byte
    ByteU(u8),
    /// Signed Half-word (2 bytes)
    HalfS(i16),
    /// Unsigned Half-word (2 bytes)
    HalfU(u16),
    /// Signed Word (4 bytes)
    WordS(i32),
    /// Unsigned Word (4 bytes)
    WordU(u32),
    /// Signed Long Word
    LongS(i64),
    /// Unsigned Long Word
    LongU(u64),
    /// Big (Signed) Int
    Big(Box<Vec<i64>>),
}

/// A MuON value
#[derive(Debug)]
pub enum Value {
    /// Text value
    Text(String),
    /// Boolean value
    Bool(bool),
    /// Arbitrary-Length Integer value
    Int(Int),
    /// 64-Bit Number value
    Number(f64),
    /// Date and time with offset
    DateTime(DateTime),
    /// Date with no time or offset
    Date(Date),
    /// Time with no date or offset
    Time(Time),
    /// Record value
    Record(Vec<(String, Value)>),
    /// Dictionary value
    Dictionary(Vec<(Value, Value)>),
    /// Any value
    Any(Box<Value>),
    /// Optional value
    Optional(Option<Box<Value>>),
    /// List of values
    List(Vec<Value>),
}

pub struct Parser<R: Read> {
    reader: R,
}

impl<R: Read> Parser<R> {
    fn new(reader: R) -> Result<Self> {
        Ok(Self {
            reader
        })
    }
}

impl<R: Read> Iterator for Parser<R> {
    type Item = (String, Value);

    fn next(&mut self) -> Option<(String, Value)> {
        None
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub enum Error {
    /// Wrapped I/O Error.
    Io(std::io::Error),
}
