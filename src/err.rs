//! See [NbtError].
use std::collections::VecDeque;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Deref, DerefMut};
use thiserror::Error;

use crate::NBTTagType;

/// An error that can occur while reading NBT data from a buffer.
#[derive(Error, Debug)]
pub enum ReadError {
    /// Occurs when an IO error occurs.
    #[error("{0}")]
    Io(#[from] std::io::Error),
    /// Occurs when attempting to read a tag with an unknown type.
    #[error("unknown tag type {0}")]
    UnknownTagType(u8),
    /// Occurs when the reader finds a tag type while reading that is not part of the expected tag
    /// types.
    #[error("expected tag {0}, found {1}")]
    UnexpectedTag(u8, u8),
    /// The length prefix found in the buffer for a sequence is not in the acceptable bounds for
    /// that type.
    #[error("sequence length must be between 0 and {0}, but got {1}")]
    SeqLengthViolation(usize, i32),
    /// A byte sequence could not be read as a valid UTF-8 byte sequence.
    #[error("could not decode string")]
    InvalidString(Vec<u8>),
    /// A custom variant for errors other than the provided variants.
    #[error("{0}")]
    Custom(String),
}

/// An error that can occur while writing NBT data into a buffer.
#[derive(Error, Debug)]
pub enum WriteError {
    /// Occurs when an IO error occurs.
    #[error("{0}")]
    Io(#[from] std::io::Error),
    /// Occurs when a list is made up of NBT tags with differing types.
    #[error("expected tag {0}, found {1}")]
    UnexpectedTag(NBTTagType, NBTTagType),
    /// The length of a  sequence (such as list or string) is not in the acceptable bounds for that
    /// type.
    #[error("sequence length must be between 0 and {0}, but got {1}")]
    SeqLengthViolation(usize, usize),
    /// A custom variant for errors other than the provided variants.
    #[error("{0}")]
    Custom(String),
}

/// A generic wrapper that gives a [Path] to an error type.
pub struct NBTError<I> {
    /// The inner data for an NBT error. It is boxed to reduce the memory footprint of the happy
    /// path.
    pub boxed: Box<InnerNBTError<I>>,
}

/// Inner container for the [NBTError] type.
pub struct InnerNBTError<I> {
    /// The inner element that the wrapper wraps around.
    pub inner: I,
    /// The associated path. Usually, this should be the location where the error occurred.
    pub path: Path,
}

impl<I> Deref for NBTError<I> {
    type Target = InnerNBTError<I>;

    fn deref(&self) -> &Self::Target {
        &self.boxed
    }
}

impl<I> DerefMut for NBTError<I> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.boxed
    }
}

impl<I> NBTError<I> {
    /// Create a new [ErrorPath] wrapper from the inner element, using the default (empty) path.
    pub fn new(inner: I) -> Self {
        Self {
            boxed: Box::new(InnerNBTError {
                inner,
                path: Default::default(),
            }),
        }
    }

    /// Create a new [ErrorPath] wrapper from the inner element and a path.
    pub fn new_with_path(inner: I, path: Path) -> Self {
        Self {
            boxed: Box::new(InnerNBTError { inner, path }),
        }
    }

    /// Prepend the path in the wrapper with a new [PathPart].
    pub fn prepend(mut self, part: PathPart) -> Self {
        self.path.0.push_front(part);
        self
    }
}

impl<I: Error + 'static> Error for NBTError<I> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.inner)
    }
}

impl<I: Clone> Clone for NBTError<I> {
    fn clone(&self) -> Self {
        Self {
            boxed: Box::new(InnerNBTError {
                inner: self.inner.clone(),
                path: self.path.clone(),
            }),
        }
    }
}

impl<I: Default> Default for NBTError<I> {
    fn default() -> Self {
        Self {
            boxed: Box::new(InnerNBTError {
                inner: Default::default(),
                path: Default::default(),
            }),
        }
    }
}

impl<I: Debug> Debug for NBTError<I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ErrorPath")
            .field("inner", &self.inner)
            .field("path", &self.path)
            .finish()
    }
}

impl<I: Display> Display for NBTError<I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("`")?;
        <Path as Display>::fmt(&self.path, f)?;
        f.write_str("`: ")?;
        self.inner.fmt(f)
    }
}

impl<I: PartialEq> PartialEq for NBTError<I> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner && self.path == other.path
    }
}

impl<I: Eq> Eq for NBTError<I> {}

/// A 'path' in a rust type that indicates where an error occurred.
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Path(pub VecDeque<PathPart>);

impl Path {
    /// Create a path from a single [PathPart].
    pub fn from_single(part: PathPart) -> Self {
        Self(VecDeque::from([part]))
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.0.is_empty() {
            return f.write_str("(root)");
        }

        let mut iter = self.0.iter();
        // Unwrapping will never panic here to the 'is empty' check.
        <PathPart as Display>::fmt(iter.next().unwrap(), f)?;

        for next in iter {
            if let PathPart::Element(_) = next {
                // Dont write a `.` for sequence elements.
            } else {
                f.write_str(".")?;
            }
            <PathPart as Display>::fmt(next, f)?;
        }
        Ok(())
    }
}

/// A single part of an [Path].
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PathPart {
    /// The path part is a map key.
    MapKey(String),
    /// The path part is a field in a struct.
    Field(String),
    /// THe path part is a field of a tuple.
    TupleField(usize),
    /// The path part is a sequence element.
    Element(usize),
}

impl Display for PathPart {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PathPart::MapKey(v) => f.write_str(v),
            PathPart::Field(v) => f.write_str(v),
            PathPart::Element(v) => {
                f.write_str("[")?;
                f.write_str(&v.to_string())?;
                f.write_str("]")
            }
            PathPart::TupleField(v) => f.write_str(&v.to_string()),
        }
    }
}
