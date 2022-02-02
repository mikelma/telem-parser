use std::error::Error;
use std::fmt;

use crate::constants::*;

#[derive(Debug)]
pub enum TelemError {
    /// The number of fields of the packet is greater than the maximum number of fields.
    /// Contains the number of fields of the packet.
    InvalidNumberOfFields(usize),
    InvalidFieldId(usize, usize),
    MissingBOP,
    MissingEOP,
    MissingBytes(usize),
    LengthError,
    Base40Decode,
    CrcError,
    CobsError,
}

impl fmt::Display for TelemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            TelemError::InvalidNumberOfFields(n) 
                => write!(f, "Invalid number of fields. Max. fields is {} and got {}.", 
                          TELEMETRY_MAX_FIELDS, n),
            TelemError::InvalidFieldId(id, max) 
                => write!(f, "Invalid field id: max field id is {} and got {}.", 
                          max, id),
            TelemError::MissingBOP => write!(f, "Missing starter marker in the beginning \
                                             of the package"),
            TelemError::MissingEOP => write!(f, "Missing end marker in the end \
                                             of the package"),
            TelemError::LengthError => write!(f, "Real length of the packet and number of fields do not match"),
            TelemError::CrcError => write!(f, "Computed CRC and received CRC codes do not match"),
            TelemError::Base40Decode => write!(f, "Failed to decode bytes as base40 string"),
            TelemError::MissingBytes(n) => write!(f, "Package has missing bytes. Minimum number \
                                                  of bytes is {} and the package length is {}", 
                                                  TELEMETRY_MIN_BYTES, n),
            TelemError::CobsError => write!(f, "COBS error, failed to decode bytes"),
        }
    }
}

impl Error for TelemError {}
