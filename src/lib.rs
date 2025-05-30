#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

use indexmap::IndexMap;
use std::fmt::Debug;
use std::io::{Read, Write};

use strum_macros::{Display, IntoStaticStr};

use writer::Writer;

use crate::err::{NBTError, Path, PathPart, ReadError, WriteError};
use crate::reader::Reader;
use crate::view::View;

pub mod encoding;
pub mod err;
mod r#impl;
pub mod reader;
#[cfg(feature = "serde")]
pub mod serde;
pub mod tag;
pub mod view;
pub mod writer;

/// An enum representing all possible NBT data.
#[derive(Debug, Clone, PartialEq)]
pub enum NBTTag {
    /// An 8-bit signed integer.
    Byte(tag::Byte),
    /// A 16-bit signed integer.
    Short(tag::Short),
    /// A 32-bit signed integer.
    Int(tag::Int),
    /// A 64-bit signed integer.
    Long(tag::Long),
    /// A 32-bit floating point number.
    Float(tag::Float),
    /// A 64-bit floating point number.
    Double(tag::Double),
    /// A string of characters.
    ///
    /// Should never be larger than [i16::MAX].
    String(tag::String),
    /// A map containing zero or more key-value pairs.
    ///
    /// Each key maps to exactly one [NBTTag] of any type.
    Compound(tag::Compound),
    /// A variable-length list [NBTTag]s of the same type.
    ///
    /// Lists will fail to encode/decode should it contain values of which the type does not match
    /// the type of the first element in the list.
    List(tag::List),
    /// A variable-length array containing 8-bit unsigned integers.
    ByteArray(tag::ByteArray),
    /// A variable-length array containing 32-bit signed integers.
    IntArray(tag::IntArray),
    /// A variable-length array containing 64-bit signed integers.
    LongArray(tag::LongArray),
}

/// An enum representing all possible NBT tag types.
#[allow(missing_docs)]
#[derive(Copy, Clone, Debug, Display, IntoStaticStr, Eq, PartialEq)]
pub enum NBTTagType {
    Byte,
    Short,
    Int,
    Long,
    Float,
    Double,
    String,
    Compound,
    List,
    ByteArray,
    IntArray,
    LongArray,
}

impl NBTTag {
    /// Returns the [NBTTagType] associated with the tag variant contained in the enum.
    pub fn tag_type(&self) -> NBTTagType {
        match self {
            NBTTag::Byte(v) => v.tag_type(),
            NBTTag::Short(v) => v.tag_type(),
            NBTTag::Int(v) => v.tag_type(),
            NBTTag::Long(v) => v.tag_type(),
            NBTTag::Float(v) => v.tag_type(),
            NBTTag::Double(v) => v.tag_type(),
            NBTTag::String(v) => v.tag_type(),
            NBTTag::Compound(v) => v.tag_type(),
            NBTTag::List(v) => v.tag_type(),
            NBTTag::ByteArray(v) => v.tag_type(),
            NBTTag::IntArray(v) => v.tag_type(),
            NBTTag::LongArray(v) => v.tag_type(),
        }
    }

    /// Creates a [View] for the NBT tag for easy reading.
    pub fn view(&self) -> View {
        View::new(self)
    }

    /// Attempts to read the data from a buffer into an NBT value using the specified [Reader]
    /// encoding.
    pub fn read<R: Reader>(buf: &mut impl Read) -> reader::Res<Self> {
        let tag_id = R::u8(buf)?;
        R::string(buf)?;
        Self::read_payload::<R>(tag_id, buf)
    }

    fn read_payload<R: Reader>(tag_id: u8, buf: &mut impl Read) -> reader::Res<Self> {
        match tag_id {
            1 => Ok(NBTTag::Byte(tag::Byte::read_payload::<R>(buf)?)),
            2 => Ok(NBTTag::Short(tag::Short::read_payload::<R>(buf)?)),
            3 => Ok(NBTTag::Int(tag::Int::read_payload::<R>(buf)?)),
            4 => Ok(NBTTag::Long(tag::Long::read_payload::<R>(buf)?)),
            5 => Ok(NBTTag::Float(tag::Float::read_payload::<R>(buf)?)),
            6 => Ok(NBTTag::Double(tag::Double::read_payload::<R>(buf)?)),
            8 => Ok(NBTTag::String(tag::String::read_payload::<R>(buf)?)),
            10 => Ok(NBTTag::Compound(tag::Compound::read_payload::<R>(buf)?)),
            9 => Ok(NBTTag::List(tag::List::read_payload::<R>(buf)?)),
            7 => Ok(NBTTag::ByteArray(tag::ByteArray::read_payload::<R>(buf)?)),
            11 => Ok(NBTTag::IntArray(tag::IntArray::read_payload::<R>(buf)?)),
            12 => Ok(NBTTag::LongArray(tag::LongArray::read_payload::<R>(buf)?)),
            other => Err(NBTError::new(ReadError::UnknownTagType(other))),
        }
    }

    /// Attempts to write the NBT data into a buffer using the specified [Writer] encoding.
    pub fn write<W: Writer>(&self, buf: &mut impl Write) -> writer::Res {
        W::write_u8(buf, self.tag_id())?;
        W::write_string(buf, "")?;
        self.write_payload::<W>(buf)
    }

    fn write_payload<W: Writer>(&self, buf: &mut impl Write) -> writer::Res {
        match self {
            NBTTag::Byte(tag) => tag.write_payload::<W>(buf),
            NBTTag::Short(tag) => tag.write_payload::<W>(buf),
            NBTTag::Int(tag) => tag.write_payload::<W>(buf),
            NBTTag::Long(tag) => tag.write_payload::<W>(buf),
            NBTTag::Float(tag) => tag.write_payload::<W>(buf),
            NBTTag::Double(tag) => tag.write_payload::<W>(buf),
            NBTTag::String(tag) => tag.write_payload::<W>(buf),
            NBTTag::Compound(tag) => tag.write_payload::<W>(buf),
            NBTTag::List(tag) => tag.write_payload::<W>(buf),
            NBTTag::ByteArray(tag) => tag.write_payload::<W>(buf),
            NBTTag::IntArray(tag) => tag.write_payload::<W>(buf),
            NBTTag::LongArray(tag) => tag.write_payload::<W>(buf),
        }
    }

    /// Gets the discriminator of a [NBTTag]'s type used for encoding and decoding.
    pub(crate) fn tag_id(&self) -> u8 {
        match self {
            NBTTag::Byte(_) => 1,
            NBTTag::Short(_) => 2,
            NBTTag::Int(_) => 3,
            NBTTag::Long(_) => 4,
            NBTTag::Float(_) => 5,
            NBTTag::Double(_) => 6,
            NBTTag::String(_) => 8,
            NBTTag::Compound(_) => 10,
            NBTTag::List(_) => 9,
            NBTTag::ByteArray(_) => 7,
            NBTTag::IntArray(_) => 11,
            NBTTag::LongArray(_) => 12,
        }
    }
}

impl Default for NBTTag {
    fn default() -> Self {
        Self::Compound(IndexMap::new().into())
    }
}

/// A trait implemented on all NBT tags to define reading/writing their payload data.
trait TagIo: Sized {
    /// Attempts to read the payload data from a buffer into an NBT value using the specified
    /// [Reader] encoding.
    fn read_payload<R: Reader>(buf: &mut impl Read) -> reader::Res<Self>;
    /// Attempts to write the NBT data into a buffer using the specified [Writer] encoding.
    fn write_payload<W: Writer>(&self, buf: &mut impl Write) -> writer::Res;
}
impl TagIo for tag::Byte {
    fn read_payload<R: Reader>(buf: &mut impl Read) -> reader::Res<Self> {
        Ok(R::i8(buf)?.into())
    }

    fn write_payload<W: Writer>(&self, buf: &mut impl Write) -> writer::Res {
        W::write_i8(buf, self.0)
    }
}
impl TagIo for tag::Short {
    fn read_payload<R: Reader>(buf: &mut impl Read) -> reader::Res<Self> {
        Ok(R::i16(buf)?.into())
    }

    fn write_payload<W: Writer>(&self, buf: &mut impl Write) -> writer::Res {
        W::write_i16(buf, self.0)
    }
}
impl TagIo for tag::Int {
    fn read_payload<R: Reader>(buf: &mut impl Read) -> reader::Res<Self> {
        Ok(R::i32(buf)?.into())
    }

    fn write_payload<W: Writer>(&self, buf: &mut impl Write) -> writer::Res {
        W::write_i32(buf, self.0)
    }
}
impl TagIo for tag::Long {
    fn read_payload<R: Reader>(buf: &mut impl Read) -> reader::Res<Self> {
        Ok(R::i64(buf)?.into())
    }

    fn write_payload<W: Writer>(&self, buf: &mut impl Write) -> writer::Res {
        W::write_i64(buf, self.0)
    }
}
impl TagIo for tag::Float {
    fn read_payload<R: Reader>(buf: &mut impl Read) -> reader::Res<Self> {
        Ok(R::f32(buf)?.into())
    }

    fn write_payload<W: Writer>(&self, buf: &mut impl Write) -> writer::Res {
        W::write_f32(buf, self.0)
    }
}
impl TagIo for tag::Double {
    fn read_payload<R: Reader>(buf: &mut impl Read) -> reader::Res<Self> {
        Ok(R::f64(buf)?.into())
    }

    fn write_payload<W: Writer>(&self, buf: &mut impl Write) -> writer::Res {
        W::write_f64(buf, self.0)
    }
}
impl TagIo for tag::String {
    fn read_payload<R: Reader>(buf: &mut impl Read) -> reader::Res<Self> {
        let string = R::string(buf);
        match string {
            Ok(string) => Ok(tag::String::Utf8(string)),
            Err(err) => {
                if let ReadError::InvalidString(bytes) = err.boxed.inner {
                    Ok(tag::String::Bytes(bytes))
                } else {
                    Err(err)
                }
            }
        }
    }

    fn write_payload<W: Writer>(&self, buf: &mut impl Write) -> writer::Res {
        match self {
            tag::String::Utf8(x) => W::write_string(buf, x.as_str()),
            tag::String::Bytes(x) => {
                if x.len() > i16::MAX as usize {
                    return Err(NBTError::new(WriteError::SeqLengthViolation(
                        i16::MAX as usize,
                        x.len(),
                    )));
                }
                W::write_i16(buf, x.len() as i16)?;
                for (i, b) in x.iter().enumerate() {
                    W::write_u8(buf, *b).map_err(|err| err.prepend(PathPart::Element(i)))?;
                }
                Ok(())
            }
        }
    }
}
impl TagIo for tag::List {
    fn read_payload<R: Reader>(buf: &mut impl Read) -> reader::Res<Self> {
        let content_type = R::u8(buf)?;
        let len = R::i32(buf)?;
        let len: usize = len.try_into().map_err(|_| {
            NBTError::new(ReadError::SeqLengthViolation(
                // i32 has a lower limit on 32 bit machines.
                usize::MAX.min(i32::MAX as usize),
                len,
            ))
        })?;
        let mut vec = Vec::with_capacity(len.min(1024 / size_of::<NBTTag>()));
        for i in 0..len {
            vec.push(
                NBTTag::read_payload::<R>(content_type, buf)
                    .map_err(|err| err.prepend(PathPart::Element(i)))?,
            );
        }
        Ok(vec.into())
    }

    fn write_payload<W: Writer>(&self, buf: &mut impl Write) -> writer::Res {
        let first_id = if self.0.is_empty() {
            0
        } else {
            self.0[0].tag_id()
        };

        W::write_u8(buf, first_id)?;
        W::write_i32(buf, self.len() as i32)?;
        for (i, v) in self.0.iter().enumerate() {
            if v.tag_id() != first_id {
                return Err(NBTError::new_with_path(
                    WriteError::UnexpectedTag(self[0].tag_type(), v.tag_type()),
                    Path::from_single(PathPart::Element(i)),
                ));
            }
            v.write_payload::<W>(buf)?;
        }
        Ok(())
    }
}
impl TagIo for tag::Compound {
    fn read_payload<R: Reader>(buf: &mut impl Read) -> reader::Res<Self> {
        let mut map = IndexMap::new();
        loop {
            let content_type = R::u8(buf)?;
            if content_type == 0 {
                break;
            }
            let name = R::string(buf)?;
            let value = NBTTag::read_payload::<R>(content_type, buf)
                .map_err(|err| err.prepend(PathPart::MapKey(name.clone())))?;
            map.insert(name, value);
        }
        Ok(map.into())
    }

    fn write_payload<W: Writer>(&self, buf: &mut impl Write) -> writer::Res {
        for (name, val) in &self.0 {
            W::write_u8(buf, val.tag_id())?;
            W::write_string(buf, name)?;
            val.write_payload::<W>(buf)?;
        }
        W::write_end(buf)?;
        Ok(())
    }
}
impl TagIo for tag::ByteArray {
    fn read_payload<R: Reader>(buf: &mut impl Read) -> reader::Res<Self> {
        Ok(R::i8_vec(buf)?.into())
    }

    fn write_payload<W: Writer>(&self, buf: &mut impl Write) -> writer::Res {
        W::write_i8_vec(buf, &self.0)
    }
}
impl TagIo for tag::IntArray {
    fn read_payload<R: Reader>(buf: &mut impl Read) -> reader::Res<Self> {
        Ok(R::i32_vec(buf)?.into())
    }

    fn write_payload<W: Writer>(&self, buf: &mut impl Write) -> writer::Res {
        W::write_i32_vec(buf, &self.0)
    }
}
impl TagIo for tag::LongArray {
    fn read_payload<R: Reader>(buf: &mut impl Read) -> reader::Res<Self> {
        Ok(R::i64_vec(buf)?.into())
    }

    fn write_payload<W: Writer>(&self, buf: &mut impl Write) -> writer::Res {
        W::write_i64_vec(buf, &self.0)
    }
}
