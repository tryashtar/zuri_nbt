//! See [Writer].
use std::io::Write;

use crate::err::{ErrorPath, PathPart, WriteError};

/// A short notation for the result type used in the [Writer].
pub type Res = Result<(), ErrorPath<WriteError>>;

/// A trait that can be implemented to alter how basic NBT types are written.
///
/// All the implemented methods must not panic.
pub trait Writer {
    /// Writes an 8-bit unsigned integer.
    fn write_u8(&mut self, buf: &mut impl Write, x: u8) -> Res;
    /// Writes an 8-bit signed integer.
    fn write_i8(&mut self, buf: &mut impl Write, x: i8) -> Res;
    /// Writes a 16-bit signed integer.
    fn write_i16(&mut self, buf: &mut impl Write, x: i16) -> Res;
    /// Writes a 32-bit signed integer.
    fn write_i32(&mut self, buf: &mut impl Write, x: i32) -> Res;
    /// Writes a 64-bit signed integer.
    fn write_i64(&mut self, buf: &mut impl Write, x: i64) -> Res;
    /// Writes a 32-bit floating point number.
    fn write_f32(&mut self, buf: &mut impl Write, x: f32) -> Res;
    /// Writes a 64-bit floating point number.
    fn write_f64(&mut self, buf: &mut impl Write, x: f64) -> Res;

    /// Writes the NBT `end` tag, which indicates the end of a compound tag.
    fn write_end(&mut self, buf: &mut impl Write) -> Res {
        self.write_u8(buf, 0)
    }

    /// Writes a variable-length string.
    fn write_string(&mut self, buf: &mut impl Write, x: &str) -> Res {
        let modified_bytes = cesu8::to_java_cesu8(x);
        if modified_bytes.len() > i16::MAX as usize {
            return Err(ErrorPath::new(WriteError::SeqLengthViolation(
                i16::MAX as usize,
                modified_bytes.len(),
            )));
        }

        self.write_i16(buf, modified_bytes.len() as i16)?;
        for (i, b) in modified_bytes.iter().enumerate() {
            self.write_u8(buf, *b)
                .map_err(|err| err.prepend(PathPart::Element(i)))?;
        }
        Ok(())
    }

    /// Writes variable-length array of 8-bit signed integers.
    fn write_i8_vec(&mut self, buf: &mut impl Write, x: &[i8]) -> Res {
        if x.len() > i32::MAX as usize {
            return Err(ErrorPath::new(WriteError::SeqLengthViolation(
                i32::MAX as usize,
                x.len(),
            )));
        }
        self.write_i32(buf, x.len() as i32)?;
        for (i, v) in x.iter().enumerate() {
            self.write_i8(buf, *v)
                .map_err(|err| err.prepend(PathPart::Element(i)))?;
        }
        Ok(())
    }

    /// Writes variable-length array of 8-bit unsigned integers.
    fn write_u8_vec(&mut self, buf: &mut impl Write, x: &[u8]) -> Res {
        if x.len() > i32::MAX as usize {
            return Err(ErrorPath::new(WriteError::SeqLengthViolation(
                i32::MAX as usize,
                x.len(),
            )));
        }
        self.write_i32(buf, x.len() as i32)?;
        for (i, v) in x.iter().enumerate() {
            self.write_u8(buf, *v)
                .map_err(|err| err.prepend(PathPart::Element(i)))?;
        }
        Ok(())
    }

    /// Writes variable-length array of 32-bit signed integers.
    fn write_i32_vec(&mut self, buf: &mut impl Write, x: &[i32]) -> Res {
        if x.len() > i32::MAX as usize {
            return Err(ErrorPath::new(WriteError::SeqLengthViolation(
                i32::MAX as usize,
                x.len(),
            )));
        }
        self.write_i32(buf, x.len() as i32)?;
        for (i, v) in x.iter().enumerate() {
            self.write_i32(buf, *v)
                .map_err(|err| err.prepend(PathPart::Element(i)))?;
        }
        Ok(())
    }

    /// Writes variable-length array of 64-bit signed integers.
    fn write_i64_vec(&mut self, buf: &mut impl Write, x: &[i64]) -> Res {
        if x.len() > i32::MAX as usize {
            return Err(ErrorPath::new(WriteError::SeqLengthViolation(
                i32::MAX as usize,
                x.len(),
            )));
        }
        self.write_i32(buf, x.len() as i32)?;
        for (i, v) in x.iter().enumerate() {
            self.write_i64(buf, *v)
                .map_err(|err| err.prepend(PathPart::Element(i)))?;
        }
        Ok(())
    }
}
