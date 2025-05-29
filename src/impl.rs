//! Implementations for type conversions from and to [NBTTag] using [From] and [TryFrom] and other
//! useful traits and methods.
use crate::decode::Reader;
use crate::encode::Writer;
use crate::err::{ErrorPath, ReadError};
use crate::{decode, encode, tag, NBTTag, NBTTagType, TagIo};
use indexmap::IndexMap;
use std::fmt::{Display, Formatter};
use std::io::{Read, Write};
use std::ops::{Deref, DerefMut};

macro_rules! impl_enum_conv {
    ($typ:ty, $enum_variant:path) => {
        impl TryFrom<NBTTag> for $typ {
            type Error = NBTTag;

            fn try_from(value: NBTTag) -> Result<Self, Self::Error> {
                if let $enum_variant(v) = value {
                    Ok(v.into())
                } else {
                    Err(value)
                }
            }
        }

        impl From<$typ> for NBTTag {
            fn from(value: $typ) -> Self {
                $enum_variant(value.into())
            }
        }
    };
    ($(($typ:ty, $enum_variant:path)$(,)?)*) => {
        $(impl_enum_conv!($typ, $enum_variant);)*
    };
}

impl_enum_conv!(
    (tag::Byte, NBTTag::Byte),
    (tag::Short, NBTTag::Short),
    (tag::Int, NBTTag::Int),
    (tag::Long, NBTTag::Long),
    (tag::Float, NBTTag::Float),
    (tag::Double, NBTTag::Double),
    (tag::String, NBTTag::String),
    (tag::Compound, NBTTag::Compound),
    (tag::List, NBTTag::List),
    (tag::ByteArray, NBTTag::ByteArray),
    (tag::IntArray, NBTTag::IntArray),
    (tag::LongArray, NBTTag::LongArray),
);

macro_rules! impl_newtype_conv {
    ($typ:ty, $newtyp:path, $enum_variant:path) => {
        impl From<$newtyp> for $typ {
            fn from(value: $newtyp) -> Self {
                value.0
            }
        }

        impl From<$typ> for $newtyp {
            fn from(value: $typ) -> Self {
                $newtyp(value)
            }
        }

        impl Deref for $newtyp {
            type Target = $typ;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl DerefMut for $newtyp {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl TryFrom<NBTTag> for $typ {
            type Error = NBTTag;

            fn try_from(value: NBTTag) -> Result<Self, Self::Error> {
                if let $enum_variant(v) = value {
                    Ok(v.into())
                } else {
                    Err(value)
                }
            }
        }

        impl From<$typ> for NBTTag {
            fn from(value: $typ) -> Self {
                $enum_variant(value.into())
            }
        }

    };
    ($(($typ:ty, $newtyp:path, $enum_variant:path)$(,)?)*) => {
        $(impl_newtype_conv!($typ, $newtyp, $enum_variant);)*
    };
}

impl_newtype_conv!(
    (i8, tag::Byte, NBTTag::Byte),
    (i16, tag::Short, NBTTag::Short),
    (i32, tag::Int, NBTTag::Int),
    (i64, tag::Long, NBTTag::Long),
    (f32, tag::Float, NBTTag::Float),
    (f64, tag::Double, NBTTag::Double),
    (IndexMap<String, NBTTag>, tag::Compound, NBTTag::Compound),
    (Vec<i8>, tag::ByteArray, NBTTag::ByteArray),
    (Vec<i32>, tag::IntArray, NBTTag::IntArray),
    (Vec<i64>, tag::LongArray, NBTTag::LongArray),
);

/// Special case: converting `&str` to a [tag::String] requires a clone.
impl From<&str> for tag::String {
    fn from(value: &str) -> Self {
        tag::String::Utf8(value.to_string())
    }
}

impl From<String> for tag::String {
    fn from(value: String) -> Self {
        tag::String::Utf8(value)
    }
}

impl<T: Into<NBTTag>> From<Vec<T>> for tag::List {
    fn from(value: Vec<T>) -> Self {
        value.into_iter().collect()
    }
}

impl<T: Into<NBTTag>, const N: usize> From<[T; N]> for tag::List {
    fn from(value: [T; N]) -> Self {
        value.into_iter().collect()
    }
}

impl<T: Into<NBTTag>> FromIterator<T> for tag::List {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        tag::List(iter.into_iter().map(|v| v.into()).collect())
    }
}

impl From<tag::List> for Vec<NBTTag> {
    fn from(value: tag::List) -> Self {
        value.0
    }
}

impl Deref for tag::List {
    type Target = Vec<NBTTag>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for tag::List {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

macro_rules! impl_tagtype {
    ($typ:ty, $enum_variant:path, $variant_num:literal) => {
        impl $typ {
            /// Returns the [NBTTagType] associated with this tag.
            #[inline]
            pub fn tag_type(&self) -> NBTTagType {
                $enum_variant
            }
        }

        impl $typ {
            /// Attempts to read the data from a buffer into an NBT value using the specified
            /// [Reader] encoding.
            ///
            /// Returns an error if the variant byte doesn't match this tag type.
            pub fn read<R: Reader>(buf: &mut impl Read) -> decode::Res<Self> {
                let tag_id = R::u8(buf)?;
                if tag_id != $variant_num {
                    return Err(ErrorPath::new(ReadError::UnexpectedTag(
                        $variant_num,
                        tag_id,
                    )));
                }
                R::string(buf)?;
                Self::read_payload::<R>(buf)
            }

            /// Attempts to write the NBT data into a buffer using the specified [Writer] encoding.
            pub fn write<W: Writer>(&self, buf: &mut impl Write) -> encode::Res {
                W::write_u8(buf, $variant_num)?;
                W::write_string(buf, "")?;
                self.write_payload::<W>(buf)
            }
        }
    };
}
impl_tagtype!(tag::Byte, NBTTagType::Byte, 1);
impl_tagtype!(tag::Short, NBTTagType::Short, 2);
impl_tagtype!(tag::Int, NBTTagType::Int, 3);
impl_tagtype!(tag::Long, NBTTagType::Long, 4);
impl_tagtype!(tag::Float, NBTTagType::Float, 5);
impl_tagtype!(tag::Double, NBTTagType::Double, 6);
impl_tagtype!(tag::String, NBTTagType::String, 8);
impl_tagtype!(tag::Compound, NBTTagType::Compound, 10);
impl_tagtype!(tag::List, NBTTagType::List, 9);
impl_tagtype!(tag::ByteArray, NBTTagType::ByteArray, 7);
impl_tagtype!(tag::IntArray, NBTTagType::IntArray, 11);
impl_tagtype!(tag::LongArray, NBTTagType::LongArray, 12);

impl Display for NBTTag {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NBTTag::Byte(byte) => byte.fmt(f),
            NBTTag::Short(short) => short.fmt(f),
            NBTTag::Int(int) => int.fmt(f),
            NBTTag::Long(long) => long.fmt(f),
            NBTTag::Float(float) => float.fmt(f),
            NBTTag::Double(double) => double.fmt(f),
            NBTTag::String(str) => str.fmt(f),
            NBTTag::Compound(compound) => compound.fmt(f),
            NBTTag::List(list) => list.fmt(f),
            NBTTag::ByteArray(byte_array) => byte_array.fmt(f),
            NBTTag::IntArray(int_array) => int_array.fmt(f),
            NBTTag::LongArray(long_array) => long_array.fmt(f),
        }
    }
}

impl Display for tag::Byte {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}b", self.0)
    }
}

impl Display for tag::Short {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}s", self.0)
    }
}

impl Display for tag::Int {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for tag::Long {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}L", self.0)
    }
}

impl Display for tag::Float {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}f", self.0)
    }
}

impl Display for tag::Double {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}d", self.0)
    }
}

impl Display for tag::String {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            tag::String::Utf8(str) => write!(f, "{:?}", str),
            tag::String::Bytes(bytes) => write!(f, "{:?}", String::from_utf8_lossy(bytes)),
        }
    }
}

impl Display for tag::Compound {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        for (i, (name, val)) in self.0.iter().enumerate() {
            write!(f, "{:?}: {}", name, val)?;
            if i < self.len() - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "}}")?;
        Ok(())
    }
}

impl Display for tag::List {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        for (i, val) in self.0.iter().enumerate() {
            write!(f, "{}", val)?;
            if i < self.len() - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "}}")?;
        Ok(())
    }
}

impl Display for tag::ByteArray {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[B; ")?;
        for (i, val) in self.0.iter().enumerate() {
            write!(f, "{}b", val)?;
            if i < self.len() - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "}}")?;
        Ok(())
    }
}

impl Display for tag::IntArray {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[I; ")?;
        for (i, val) in self.0.iter().enumerate() {
            write!(f, "{}", val)?;
            if i < self.len() - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "}}")?;
        Ok(())
    }
}

impl Display for tag::LongArray {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[L; ")?;
        for (i, val) in self.0.iter().enumerate() {
            write!(f, "{}L", val)?;
            if i < self.len() - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "}}")?;
        Ok(())
    }
}
