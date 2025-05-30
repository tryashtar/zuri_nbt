//! Contains all NBT tags.
use indexmap::IndexMap;

use crate::NBTTag;

/// An 8-bit signed integer.
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct Byte(pub i8);

/// A 16-bit signed integer.
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct Short(pub i16);

/// A 32-bit signed integer.
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct Int(pub i32);

/// A 64-bit signed integer.
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct Long(pub i64);

/// A 32-bit floating point number.
#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct Float(pub f32);

/// A 64-bit floating point number.
#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct Double(pub f64);

/// A string of characters.
///
/// Should never be larger than [i16::MAX].
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum String {
    /// Normal and correct UTF-8 encoded string.
    Utf8(std::string::String),
    /// Arbitrary bytes that do not constitute valid UTF-8.
    Bytes(Vec<u8>),
}
impl String {
    /// Converts the string tag to a UTF-8 string, substituting invalid characters if necessary.
    pub fn to_string_lossy(&self) -> std::borrow::Cow<str> {
        match self {
            Self::Utf8(str) => std::borrow::Cow::Borrowed(str),
            Self::Bytes(bytes) => std::string::String::from_utf8_lossy(bytes),
        }
    }
}

/// A map containing zero or more key-value pairs.
///
/// Each key maps to exactly one [NBTTag] of any type.
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Compound(pub IndexMap<std::string::String, NBTTag>);

/// A variable-length list [NBTTag]s of the same type.
///
/// Lists will fail to encode/decode should it contain values of which the type does not match
/// the type of the first element in the list.
#[derive(Default, Debug, Clone, PartialEq)]
pub struct List(pub Vec<NBTTag>);

/// A variable-length array containing 8-bit signed integers.
#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct ByteArray(pub Vec<i8>);

/// A variable-length array containing 32-bit signed integers.
#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct IntArray(pub Vec<i32>);

/// A variable-length array containing 64-bit signed integers.
#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct LongArray(pub Vec<i64>);

/// Contains utilities for the [Compound] NBT tag.
pub mod compound {
    use crate::{tag, NBTTag};

    impl super::Compound {
        /// Returns a reference to a contained tag by name, if it exists and is a byte tag.
        pub fn get_byte(&self, key: &str) -> Option<&tag::Byte> {
            match self.get(key) {
                Some(NBTTag::Byte(val)) => Some(val),
                _ => None,
            }
        }
        /// Returns a reference to a contained tag by name, if it exists and is a short tag.
        pub fn get_short(&self, key: &str) -> Option<&tag::Short> {
            match self.get(key) {
                Some(NBTTag::Short(val)) => Some(val),
                _ => None,
            }
        }
        /// Returns a reference to a contained tag by name, if it exists and is an int tag.
        pub fn get_int(&self, key: &str) -> Option<&tag::Int> {
            match self.get(key) {
                Some(NBTTag::Int(val)) => Some(val),
                _ => None,
            }
        }
        /// Returns a reference to a contained tag by name, if it exists and is a long tag.
        pub fn get_long(&self, key: &str) -> Option<&tag::Long> {
            match self.get(key) {
                Some(NBTTag::Long(val)) => Some(val),
                _ => None,
            }
        }
        /// Returns a reference to a contained tag by name, if it exists and is a float tag.
        pub fn get_float(&self, key: &str) -> Option<&tag::Float> {
            match self.get(key) {
                Some(NBTTag::Float(val)) => Some(val),
                _ => None,
            }
        }
        /// Returns a reference to a contained tag by name, if it exists and is a double tag.
        pub fn get_double(&self, key: &str) -> Option<&tag::Double> {
            match self.get(key) {
                Some(NBTTag::Double(val)) => Some(val),
                _ => None,
            }
        }
        /// Returns a reference to a contained tag by name, if it exists and is a string tag.
        pub fn get_string(&self, key: &str) -> Option<&tag::String> {
            match self.get(key) {
                Some(NBTTag::String(val)) => Some(val),
                _ => None,
            }
        }
        /// Returns a reference to a contained tag by name, if it exists and is a compound tag.
        pub fn get_compound(&self, key: &str) -> Option<&tag::Compound> {
            match self.get(key) {
                Some(NBTTag::Compound(val)) => Some(val),
                _ => None,
            }
        }
        /// Returns a reference to a contained tag by name, if it exists and is a list tag.
        pub fn get_list(&self, key: &str) -> Option<&tag::List> {
            match self.get(key) {
                Some(NBTTag::List(val)) => Some(val),
                _ => None,
            }
        }
        /// Returns a reference to a contained tag by name, if it exists and is a byte array tag.
        pub fn get_byte_array(&self, key: &str) -> Option<&tag::ByteArray> {
            match self.get(key) {
                Some(NBTTag::ByteArray(val)) => Some(val),
                _ => None,
            }
        }
        /// Returns a reference to a contained tag by name, if it exists and is an int array tag.
        pub fn get_int_array(&self, key: &str) -> Option<&tag::IntArray> {
            match self.get(key) {
                Some(NBTTag::IntArray(val)) => Some(val),
                _ => None,
            }
        }
        /// Returns a reference to a contained tag by name, if it exists and is a long array tag.
        pub fn get_long_array(&self, key: &str) -> Option<&tag::LongArray> {
            match self.get(key) {
                Some(NBTTag::LongArray(val)) => Some(val),
                _ => None,
            }
        }
    }

    /// Allows for a more ergonomic way of creating NBT compound tags.
    #[must_use]
    #[derive(Debug, Default)]
    pub struct Builder {
        value: super::Compound,
    }

    impl super::Compound {
        /// Returns a new builder object to create a compound tag.
        pub fn builder() -> Builder {
            Builder {
                value: Default::default(),
            }
        }
    }

    impl Builder {
        /// Consume the builder and return the underlying compound tag.
        #[must_use]
        pub fn build(self) -> super::Compound {
            self.value
        }

        /// Inserts a new NBT tag into the underlying compound tag under the provided key.
        ///
        /// Panics when inserting with a key that already exists.
        pub fn with<T: Into<NBTTag>>(mut self, key: impl Into<String>, value: T) -> Self {
            let key = key.into();
            if let Some(val) = self.value.0.get(&key) {
                panic!("trying to overwrite key `{key}` that has value: {val:?}",);
            }
            self.value.0.insert(key, value.into());
            self
        }

        /// Inserts a [tag::Byte] into the builder under the provided key.
        ///
        /// Panics when inserting with a key that already exists.
        pub fn with_byte(self, key: impl Into<String>, v: impl Into<tag::Byte>) -> Self {
            self.with(key, v.into())
        }

        /// Inserts a [tag::Short] into the builder under the provided key.
        ///
        /// Panics when inserting with a key that already exists.
        pub fn with_short(self, key: impl Into<String>, v: impl Into<tag::Short>) -> Self {
            self.with(key, v.into())
        }

        /// Inserts a [tag::Int] into the builder under the provided key.
        ///
        /// Panics when inserting with a key that already exists.
        pub fn with_int(self, key: impl Into<String>, v: impl Into<tag::Int>) -> Self {
            self.with(key, v.into())
        }

        /// Inserts a [tag::Long] into the builder under the provided key.
        ///
        /// Panics when inserting with a key that already exists.
        pub fn with_long(self, key: impl Into<String>, v: impl Into<tag::Long>) -> Self {
            self.with(key, v.into())
        }

        /// Inserts a [tag::Float] into the builder under the provided key.
        ///
        /// Panics when inserting with a key that already exists.
        pub fn with_float(self, key: impl Into<String>, v: impl Into<tag::Float>) -> Self {
            self.with(key, v.into())
        }

        /// Inserts a [tag::Double] into the builder under the provided key.
        ///
        /// Panics when inserting with a key that already exists.
        pub fn with_double(self, key: impl Into<String>, v: impl Into<tag::Double>) -> Self {
            self.with(key, v.into())
        }

        /// Inserts a [tag::String] into the builder under the provided key.
        ///
        /// Panics when inserting with a key that already exists.
        pub fn with_string(self, key: impl Into<String>, v: impl Into<tag::String>) -> Self {
            self.with(key, v.into())
        }

        /// Inserts a [tag::Compound] into the builder under the provided key.
        ///
        /// Panics when inserting with a key that already exists.
        pub fn with_compound(self, key: impl Into<String>, v: impl Into<tag::Compound>) -> Self {
            self.with(key, v.into())
        }

        /// Inserts a [tag::List] into the builder under the provided key.
        ///
        /// Panics when inserting with a key that already exists.
        pub fn with_list(self, key: impl Into<String>, v: impl Into<tag::List>) -> Self {
            self.with(key, v.into())
        }

        /// Inserts a [tag::ByteArray] into the builder under the provided key.
        ///
        /// Panics when inserting with a key that already exists.
        pub fn with_byte_array(self, key: impl Into<String>, v: impl Into<tag::ByteArray>) -> Self {
            self.with(key, v.into())
        }

        /// Inserts a [tag::IntArray] into the builder under the provided key.
        ///
        /// Panics when inserting with a key that already exists.
        pub fn with_int_array(self, key: impl Into<String>, v: impl Into<tag::IntArray>) -> Self {
            self.with(key, v.into())
        }

        /// Inserts a [tag::LongArray] into the builder under the provided key.
        ///
        /// Panics when inserting with a key that already exists.
        pub fn with_long_array(self, key: impl Into<String>, v: impl Into<tag::LongArray>) -> Self {
            self.with(key, v.into())
        }
    }

    impl From<Builder> for tag::Compound {
        fn from(value: Builder) -> Self {
            value.build()
        }
    }
}
