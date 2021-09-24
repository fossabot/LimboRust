#![doc(hidden)]

pub(crate) mod read;
pub(crate) mod write;

use std::string::FromUtf8Error;

pub enum PacketBufferError {
    NoMoreBytes,
    VarI32TooLong,
    VarI64TooLong,
    BadString(FromUtf8Error),
    StringSizeZero,
    StringTooLong,
}

impl From<FromUtf8Error> for PacketBufferError {
    fn from(err: FromUtf8Error) -> Self {
        PacketBufferError::BadString(err)
    }
}

pub fn get_var_i32_size(input: i32) -> usize {
    for i in 1..5 {
        if (input & ((-1 as i32) << i * 7)) == 0 {
            return i;
        }
    }
    5
}

pub fn get_var_i64_size(input: i64) -> usize {
    for i in 1..10 {
        if (input & ((-1 as i64) << i * 7)) == 0 {
            return i;
        }
    }
    10
}
