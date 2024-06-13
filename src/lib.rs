#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

use std::collections::HashMap;
use std::fmt::Debug;
use std::io::{Read, Write};

use strum_macros::{Display, IntoStaticStr};

use encode::Writer;

use crate::decode::Reader;
use crate::err::{ErrorPath, Path, PathPart, ReadError, WriteError};
use crate::view::View;

pub mod decode;
pub mod encode;
pub mod encoding;
pub mod err;
mod r#impl;
#[cfg(feature = "serde")]
pub mod serde;
pub mod tag;
pub mod view;

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
    pub fn read(buf: &mut impl Read, r: &mut impl Reader) -> decode::Res<Self> {
        let tag_id = r.u8(buf)?;
        r.string(buf)?;
        Self::read_payload(tag_id, buf, r)
    }

    fn read_payload(tag_id: u8, buf: &mut impl Read, r: &mut impl Reader) -> decode::Res<Self> {
        match tag_id {
            1 => Ok(NBTTag::Byte(tag::Byte::read_payload(buf, r)?)),
            2 => Ok(NBTTag::Short(tag::Short::read_payload(buf, r)?)),
            3 => Ok(NBTTag::Int(tag::Int::read_payload(buf, r)?)),
            4 => Ok(NBTTag::Long(tag::Long::read_payload(buf, r)?)),
            5 => Ok(NBTTag::Float(tag::Float::read_payload(buf, r)?)),
            6 => Ok(NBTTag::Double(tag::Double::read_payload(buf, r)?)),
            8 => Ok(NBTTag::String(tag::String::read_payload(buf, r)?)),
            10 => Ok(NBTTag::Compound(tag::Compound::read_payload(buf, r)?)),
            9 => Ok(NBTTag::List(tag::List::read_payload(buf, r)?)),
            7 => Ok(NBTTag::ByteArray(tag::ByteArray::read_payload(buf, r)?)),
            11 => Ok(NBTTag::IntArray(tag::IntArray::read_payload(buf, r)?)),
            12 => Ok(NBTTag::LongArray(tag::LongArray::read_payload(buf, r)?)),
            other => Err(ErrorPath::new(ReadError::UnknownTagType(other))),
        }
    }

    /// Attempts to write the NBT data into a buffer using the specified [Writer] encoding.
    pub fn write(&self, buf: &mut impl Write, w: &mut impl Writer) -> encode::Res {
        w.write_u8(buf, self.tag_id())?;
        w.write_string(buf, "")?;
        self.write_payload(buf, w)
    }

    fn write_payload(&self, buf: &mut impl Write, w: &mut impl Writer) -> encode::Res {
        match self {
            NBTTag::Byte(tag) => tag.write_payload(buf, w),
            NBTTag::Short(tag) => tag.write_payload(buf, w),
            NBTTag::Int(tag) => tag.write_payload(buf, w),
            NBTTag::Long(tag) => tag.write_payload(buf, w),
            NBTTag::Float(tag) => tag.write_payload(buf, w),
            NBTTag::Double(tag) => tag.write_payload(buf, w),
            NBTTag::String(tag) => tag.write_payload(buf, w),
            NBTTag::Compound(tag) => tag.write_payload(buf, w),
            NBTTag::List(tag) => tag.write_payload(buf, w),
            NBTTag::ByteArray(tag) => tag.write_payload(buf, w),
            NBTTag::IntArray(tag) => tag.write_payload(buf, w),
            NBTTag::LongArray(tag) => tag.write_payload(buf, w),
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
        Self::Compound(HashMap::new().into())
    }
}

/// A trait implemented on all NBT tags to define reading/writing their payload data.
trait TagIo: Sized {
    /// Attempts to read the payload data from a buffer into an NBT value using the specified
    /// [Reader] encoding.
    fn read_payload(buf: &mut impl Read, r: &mut impl Reader) -> decode::Res<Self>;
    /// Attempts to write the NBT data into a buffer using the specified [Writer] encoding.
    fn write_payload(&self, buf: &mut impl Write, w: &mut impl Writer) -> encode::Res;
}
impl TagIo for tag::Byte {
    fn read_payload(buf: &mut impl Read, r: &mut impl Reader) -> decode::Res<Self> {
        Ok(r.i8(buf)?.into())
    }

    fn write_payload(&self, buf: &mut impl Write, w: &mut impl Writer) -> encode::Res {
        w.write_i8(buf, self.0)
    }
}
impl TagIo for tag::Short {
    fn read_payload(buf: &mut impl Read, r: &mut impl Reader) -> decode::Res<Self> {
        Ok(r.i16(buf)?.into())
    }

    fn write_payload(&self, buf: &mut impl Write, w: &mut impl Writer) -> encode::Res {
        w.write_i16(buf, self.0)
    }
}
impl TagIo for tag::Int {
    fn read_payload(buf: &mut impl Read, r: &mut impl Reader) -> decode::Res<Self> {
        Ok(r.i32(buf)?.into())
    }

    fn write_payload(&self, buf: &mut impl Write, w: &mut impl Writer) -> encode::Res {
        w.write_i32(buf, self.0)
    }
}
impl TagIo for tag::Long {
    fn read_payload(buf: &mut impl Read, r: &mut impl Reader) -> decode::Res<Self> {
        Ok(r.i64(buf)?.into())
    }

    fn write_payload(&self, buf: &mut impl Write, w: &mut impl Writer) -> encode::Res {
        w.write_i64(buf, self.0)
    }
}
impl TagIo for tag::Float {
    fn read_payload(buf: &mut impl Read, r: &mut impl Reader) -> decode::Res<Self> {
        Ok(r.f32(buf)?.into())
    }

    fn write_payload(&self, buf: &mut impl Write, w: &mut impl Writer) -> encode::Res {
        w.write_f32(buf, self.0)
    }
}
impl TagIo for tag::Double {
    fn read_payload(buf: &mut impl Read, r: &mut impl Reader) -> decode::Res<Self> {
        Ok(r.f64(buf)?.into())
    }

    fn write_payload(&self, buf: &mut impl Write, w: &mut impl Writer) -> encode::Res {
        w.write_f64(buf, self.0)
    }
}
impl TagIo for tag::String {
    fn read_payload(buf: &mut impl Read, r: &mut impl Reader) -> decode::Res<Self> {
        let string = r.string(buf);
        if let Err(ErrorPath {
            inner: ReadError::InvalidString(bytes),
            path: _,
        }) = string
        {
            Ok(tag::String::Bytes(bytes))
        } else {
            Ok(tag::String::Utf8(string?))
        }
    }

    fn write_payload(&self, buf: &mut impl Write, w: &mut impl Writer) -> encode::Res {
        match self {
            tag::String::Utf8(x) => w.write_string(buf, x.as_str()),
            tag::String::Bytes(x) => {
                if x.len() > i16::MAX as usize {
                    return Err(ErrorPath::new(WriteError::SeqLengthViolation(
                        i16::MAX as usize,
                        x.len(),
                    )));
                }
                w.write_i16(buf, x.len() as i16)?;
                for (i, b) in x.iter().enumerate() {
                    w.write_u8(buf, *b)
                        .map_err(|err| err.prepend(PathPart::Element(i)))?;
                }
                Ok(())
            }
        }
    }
}
impl TagIo for tag::List {
    fn read_payload(buf: &mut impl Read, r: &mut impl Reader) -> decode::Res<Self> {
        let content_type = r.u8(buf)?;
        let len = r.i32(buf)?;
        if len < 0 {
            return Err(ErrorPath::new(ReadError::SeqLengthViolation(
                i32::MAX as usize,
                len as usize,
            )));
        }
        let mut vec = Vec::with_capacity(len as usize);
        for i in 0..len {
            vec.push(
                NBTTag::read_payload(content_type, buf, r)
                    .map_err(|err| err.prepend(PathPart::Element(i as usize)))?,
            );
        }
        Ok(vec.into())
    }

    fn write_payload(&self, buf: &mut impl Write, w: &mut impl Writer) -> encode::Res {
        let first_id = if self.0.is_empty() {
            NBTTag::Byte(0.into()).tag_id()
        } else {
            self.0[0].tag_id()
        };

        w.write_u8(buf, first_id)?;
        w.write_i32(buf, self.len() as i32)?;
        for (i, v) in self.0.iter().enumerate() {
            if v.tag_id() != first_id {
                return Err(ErrorPath::new_with_path(
                    WriteError::UnexpectedTag(self[0].tag_type(), v.tag_type()),
                    Path::from_single(PathPart::Element(i)),
                ));
            }
            v.write_payload(buf, w)?;
        }
        Ok(())
    }
}
impl TagIo for tag::Compound {
    fn read_payload(buf: &mut impl Read, r: &mut impl Reader) -> decode::Res<Self> {
        let mut map = HashMap::new();
        loop {
            let content_type = r.u8(buf)?;
            if content_type == 0 {
                break;
            }
            let name = r.string(buf)?;
            let value = NBTTag::read_payload(content_type, buf, r)
                .map_err(|err| err.prepend(PathPart::MapKey(name.clone())))?;
            map.insert(name, value);
        }
        Ok(map.into())
    }

    fn write_payload(&self, buf: &mut impl Write, w: &mut impl Writer) -> encode::Res {
        for (name, val) in &self.0 {
            w.write_u8(buf, val.tag_id())?;
            w.write_string(buf, name)?;
            val.write_payload(buf, w)?;
        }
        w.write_end(buf)?;
        Ok(())
    }
}
impl TagIo for tag::ByteArray {
    fn read_payload(buf: &mut impl Read, r: &mut impl Reader) -> decode::Res<Self> {
        Ok(r.i8_vec(buf)?.into())
    }

    fn write_payload(&self, buf: &mut impl Write, w: &mut impl Writer) -> encode::Res {
        w.write_i8_vec(buf, &self.0)
    }
}
impl TagIo for tag::IntArray {
    fn read_payload(buf: &mut impl Read, r: &mut impl Reader) -> decode::Res<Self> {
        Ok(r.i32_vec(buf)?.into())
    }

    fn write_payload(&self, buf: &mut impl Write, w: &mut impl Writer) -> encode::Res {
        w.write_i32_vec(buf, &self.0)
    }
}
impl TagIo for tag::LongArray {
    fn read_payload(buf: &mut impl Read, r: &mut impl Reader) -> decode::Res<Self> {
        Ok(r.i64_vec(buf)?.into())
    }

    fn write_payload(&self, buf: &mut impl Write, w: &mut impl Writer) -> encode::Res {
        w.write_i64_vec(buf, &self.0)
    }
}
