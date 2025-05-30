//! See [Reader].
use crate::err::{ErrorPath, PathPart, ReadError};
use std::io::Read;

/// A short notation for the result type used in the [Reader].
pub type Res<T> = Result<T, ErrorPath<ReadError>>;

/// A trait that can be implemented to alter how basic NBT types are read.
///
/// All the implemented methods must not panic.
pub trait Reader {
    /// Reads an 8-bit unsigned integer.
    fn u8(buf: &mut impl Read) -> Res<u8>;
    /// Reads an 8-bit signed integer.
    fn i8(buf: &mut impl Read) -> Res<i8>;
    /// Reads a 16-bit signed integer.
    fn i16(buf: &mut impl Read) -> Res<i16>;
    /// Reads a 32-bit signed integer.
    fn i32(buf: &mut impl Read) -> Res<i32>;
    /// Reads a 64-bit signed integer.
    fn i64(buf: &mut impl Read) -> Res<i64>;
    /// Reads a 32-bit floating point number.
    fn f32(buf: &mut impl Read) -> Res<f32>;
    /// Reads a 64-bit floating point number.
    fn f64(buf: &mut impl Read) -> Res<f64>;

    /// Reads the NBT `end` tag, which indicates the end of a compound tag.
    fn end(buf: &mut impl Read) -> Res<()> {
        let t = Self::u8(buf)?;
        if t != 0 {
            return Err(ErrorPath::new(ReadError::UnexpectedTag(0, t)));
        }
        Ok(())
    }

    /// Reads a variable-length string.
    fn string(buf: &mut impl Read) -> Res<String> {
        let len = Self::i16(buf)?;
        if len < 0 {
            return Err(ErrorPath::new(ReadError::SeqLengthViolation(
                i16::MAX as usize,
                len as usize,
            )));
        }

        let mut str_buf = Vec::with_capacity(len as usize);
        for i in 0..len {
            str_buf.push(Self::u8(buf).map_err(|err| err.prepend(PathPart::Element(i as usize)))?);
        }
        match cesu8::from_java_cesu8(&str_buf) {
            Ok(str) => Ok(str.into_owned()),
            Err(_) => Err(ErrorPath::new(ReadError::InvalidString(str_buf))),
        }
    }

    /// Reads variable-length array of 8-bit unsigned integers.
    fn u8_vec(buf: &mut impl Read) -> Res<Vec<u8>> {
        let len = Self::i32(buf)?;
        if len < 0 {
            return Err(ErrorPath::new(ReadError::SeqLengthViolation(
                i32::MAX as usize,
                len as usize,
            )));
        }

        let mut vec_buf = Vec::with_capacity(len as usize);
        for i in 0..len {
            vec_buf.push(Self::u8(buf).map_err(|err| err.prepend(PathPart::Element(i as usize)))?);
        }

        Ok(vec_buf)
    }

    /// Reads variable-length array of 8-bit signed integers.
    fn i8_vec(buf: &mut impl Read) -> Res<Vec<i8>> {
        let len = Self::i32(buf)?;
        if len < 0 {
            return Err(ErrorPath::new(ReadError::SeqLengthViolation(
                i32::MAX as usize,
                len as usize,
            )));
        }

        let mut vec_buf = Vec::with_capacity(len as usize);
        for i in 0..len {
            vec_buf.push(Self::i8(buf).map_err(|err| err.prepend(PathPart::Element(i as usize)))?);
        }

        Ok(vec_buf)
    }

    /// Reads variable-length array of 32-bit signed integers.
    fn i32_vec(buf: &mut impl Read) -> Res<Vec<i32>> {
        let len = Self::i32(buf)?;
        if len < 0 {
            return Err(ErrorPath::new(ReadError::SeqLengthViolation(
                i32::MAX as usize,
                len as usize,
            )));
        }

        let mut vec_buf = Vec::with_capacity(len as usize);
        for i in 0..len {
            vec_buf.push(Self::i32(buf).map_err(|err| err.prepend(PathPart::Element(i as usize)))?);
        }

        Ok(vec_buf)
    }

    /// Reads variable-length array of 64-bit signed integers.
    fn i64_vec(buf: &mut impl Read) -> Res<Vec<i64>> {
        let len = Self::i32(buf)?;
        if len < 0 {
            return Err(ErrorPath::new(ReadError::SeqLengthViolation(
                i32::MAX as usize,
                len as usize,
            )));
        }

        let mut vec_buf = Vec::with_capacity(len as usize);
        for i in 0..len {
            vec_buf.push(Self::i64(buf).map_err(|err| err.prepend(PathPart::Element(i as usize)))?);
        }

        Ok(vec_buf)
    }
}
