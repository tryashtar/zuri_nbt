//! Contains all the standard NBT encodings.
//!
//! These include:
//!  - [BigEndian]
//!  - [LittleEndian]
//!  - [NetworkLittleEndian]
use crate::err::{ErrorPath, PathPart, ReadError, WriteError};
use crate::reader::Reader;
use crate::writer::Writer;
use crate::{reader, writer};
use byteorder::{ReadBytesExt, WriteBytesExt};
use std::io::{Read, Write};

/// An NBT encoding that encodes all basic types using big endian encoding.
///
/// This format is most commonly used in Minecraft: Java Edition.
#[derive(Debug, Default, Clone)]
pub struct BigEndian;

/// An NBT encoding that encodes all basic types using little endian encoding.
///
/// This format is most commonly used in Minecraft: Bedrock Edition, and more specifically in
/// Bedrock Edition world saves.
///
/// It is not to be confused with the [NetworkLittleEndian] encoding.
#[derive(Debug, Default, Clone)]
pub struct LittleEndian;

/// An NBT encoding that encodes certain integer types using variable-length encoding, while using
/// fixed-size little endian encoding for all other basic types.
///
/// This format is most commonly used for nbt sent in Minecraft: Bedrock Edition's protocol.
#[derive(Debug, Default, Clone)]
pub struct NetworkLittleEndian;

impl Reader for BigEndian {
    fn u8(buf: &mut impl Read) -> reader::Res<u8> {
        buf.read_u8().map_err(|x| ErrorPath::new(x.into()))
    }

    fn i8(buf: &mut impl Read) -> reader::Res<i8> {
        buf.read_i8().map_err(|x| ErrorPath::new(x.into()))
    }

    fn i16(buf: &mut impl Read) -> reader::Res<i16> {
        buf.read_i16::<byteorder::BigEndian>()
            .map_err(|x| ErrorPath::new(x.into()))
    }

    fn i32(buf: &mut impl Read) -> reader::Res<i32> {
        buf.read_i32::<byteorder::BigEndian>()
            .map_err(|x| ErrorPath::new(x.into()))
    }

    fn i64(buf: &mut impl Read) -> reader::Res<i64> {
        buf.read_i64::<byteorder::BigEndian>()
            .map_err(|x| ErrorPath::new(x.into()))
    }

    fn f32(buf: &mut impl Read) -> reader::Res<f32> {
        buf.read_f32::<byteorder::BigEndian>()
            .map_err(|x| ErrorPath::new(x.into()))
    }

    fn f64(buf: &mut impl Read) -> reader::Res<f64> {
        buf.read_f64::<byteorder::BigEndian>()
            .map_err(|x| ErrorPath::new(x.into()))
    }
}

impl Writer for BigEndian {
    fn write_u8(buf: &mut impl Write, x: u8) -> writer::Res {
        buf.write_u8(x).map_err(|x| ErrorPath::new(x.into()))
    }

    fn write_i8(buf: &mut impl Write, x: i8) -> writer::Res {
        buf.write_i8(x).map_err(|x| ErrorPath::new(x.into()))
    }

    fn write_i16(buf: &mut impl Write, x: i16) -> writer::Res {
        buf.write_i16::<byteorder::BigEndian>(x)
            .map_err(|x| ErrorPath::new(x.into()))
    }

    fn write_i32(buf: &mut impl Write, x: i32) -> writer::Res {
        buf.write_i32::<byteorder::BigEndian>(x)
            .map_err(|x| ErrorPath::new(x.into()))
    }

    fn write_i64(buf: &mut impl Write, x: i64) -> writer::Res {
        buf.write_i64::<byteorder::BigEndian>(x)
            .map_err(|x| ErrorPath::new(x.into()))
    }

    fn write_f32(buf: &mut impl Write, x: f32) -> writer::Res {
        buf.write_f32::<byteorder::BigEndian>(x)
            .map_err(|x| ErrorPath::new(x.into()))
    }

    fn write_f64(buf: &mut impl Write, x: f64) -> writer::Res {
        buf.write_f64::<byteorder::BigEndian>(x)
            .map_err(|x| ErrorPath::new(x.into()))
    }
}

impl Reader for LittleEndian {
    fn u8(buf: &mut impl Read) -> reader::Res<u8> {
        buf.read_u8().map_err(|x| ErrorPath::new(x.into()))
    }

    fn i8(buf: &mut impl Read) -> reader::Res<i8> {
        buf.read_i8().map_err(|x| ErrorPath::new(x.into()))
    }

    fn i16(buf: &mut impl Read) -> reader::Res<i16> {
        buf.read_i16::<byteorder::LittleEndian>()
            .map_err(|x| ErrorPath::new(x.into()))
    }

    fn i32(buf: &mut impl Read) -> reader::Res<i32> {
        buf.read_i32::<byteorder::LittleEndian>()
            .map_err(|x| ErrorPath::new(x.into()))
    }

    fn i64(buf: &mut impl Read) -> reader::Res<i64> {
        buf.read_i64::<byteorder::LittleEndian>()
            .map_err(|x| ErrorPath::new(x.into()))
    }

    fn f32(buf: &mut impl Read) -> reader::Res<f32> {
        buf.read_f32::<byteorder::LittleEndian>()
            .map_err(|x| ErrorPath::new(x.into()))
    }

    fn f64(buf: &mut impl Read) -> reader::Res<f64> {
        buf.read_f64::<byteorder::LittleEndian>()
            .map_err(|x| ErrorPath::new(x.into()))
    }
}

impl Writer for LittleEndian {
    fn write_u8(buf: &mut impl Write, x: u8) -> writer::Res {
        buf.write_u8(x).map_err(|x| ErrorPath::new(x.into()))
    }

    fn write_i8(buf: &mut impl Write, x: i8) -> writer::Res {
        buf.write_i8(x).map_err(|x| ErrorPath::new(x.into()))
    }

    fn write_i16(buf: &mut impl Write, x: i16) -> writer::Res {
        buf.write_i16::<byteorder::LittleEndian>(x)
            .map_err(|x| ErrorPath::new(x.into()))
    }

    fn write_i32(buf: &mut impl Write, x: i32) -> writer::Res {
        buf.write_i32::<byteorder::LittleEndian>(x)
            .map_err(|x| ErrorPath::new(x.into()))
    }

    fn write_i64(buf: &mut impl Write, x: i64) -> writer::Res {
        buf.write_i64::<byteorder::LittleEndian>(x)
            .map_err(|x| ErrorPath::new(x.into()))
    }

    fn write_f32(buf: &mut impl Write, x: f32) -> writer::Res {
        buf.write_f32::<byteorder::LittleEndian>(x)
            .map_err(|x| ErrorPath::new(x.into()))
    }

    fn write_f64(buf: &mut impl Write, x: f64) -> writer::Res {
        buf.write_f64::<byteorder::LittleEndian>(x)
            .map_err(|x| ErrorPath::new(x.into()))
    }
}

impl Reader for NetworkLittleEndian {
    fn u8(buf: &mut impl Read) -> reader::Res<u8> {
        buf.read_u8().map_err(|x| ErrorPath::new(x.into()))
    }

    fn i8(buf: &mut impl Read) -> reader::Res<i8> {
        buf.read_i8().map_err(|x| ErrorPath::new(x.into()))
    }

    fn i16(buf: &mut impl Read) -> reader::Res<i16> {
        buf.read_i16::<byteorder::LittleEndian>()
            .map_err(|x| ErrorPath::new(x.into()))
    }

    fn i32(buf: &mut impl Read) -> reader::Res<i32> {
        let mut v: u32 = 0;
        for i in (0..35).step_by(7) {
            let b = Self::u8(buf)?;

            v |= ((b & 0x7f) as u32) << i;
            if b & 0x80 == 0 {
                let x = (v >> 1) as i32;
                return Ok(if v & 1 != 0 { -x } else { x });
            }
        }
        Err(ErrorPath::new(ReadError::Custom(
            "varint overflows integer".to_string(),
        )))
    }

    fn i64(buf: &mut impl Read) -> reader::Res<i64> {
        let mut v: u64 = 0;
        for i in (0..70).step_by(7) {
            let b = Self::u8(buf)?;

            v |= ((b & 0x7f) as u64) << i;
            if b & 0x80 == 0 {
                let x = (v >> 1) as i64;
                return Ok(if v & 1 != 0 { -x } else { x });
            }
        }
        Err(ErrorPath::new(ReadError::Custom(
            "varint overflows integer".to_string(),
        )))
    }

    fn f32(buf: &mut impl Read) -> reader::Res<f32> {
        buf.read_f32::<byteorder::LittleEndian>()
            .map_err(|x| ErrorPath::new(x.into()))
    }

    fn f64(buf: &mut impl Read) -> reader::Res<f64> {
        buf.read_f64::<byteorder::LittleEndian>()
            .map_err(|x| ErrorPath::new(x.into()))
    }

    fn string(buf: &mut impl Read) -> reader::Res<String> {
        let len = 'var_len: {
            let mut v: u32 = 0;
            for i in (0..35).step_by(7) {
                let b = Self::u8(buf)?;

                v |= ((b & 0x7f) as u32) << i;
                if b & 0x80 == 0 {
                    break 'var_len v;
                }
            }
            return Err(ErrorPath::new(ReadError::Custom(
                "varint overflows integer".to_string(),
            )));
        };

        let mut str_buf = Vec::with_capacity(len as usize);
        for i in 0..len {
            str_buf.push(Self::u8(buf).map_err(|err| err.prepend(PathPart::Element(i as usize)))?);
        }

        match cesu8::from_java_cesu8(&str_buf) {
            Ok(str) => Ok(str.into_owned()),
            Err(_) => Err(ErrorPath::new(ReadError::InvalidString(str_buf))),
        }
    }
}

impl Writer for NetworkLittleEndian {
    fn write_u8(buf: &mut impl Write, x: u8) -> writer::Res {
        buf.write_u8(x).map_err(|x| ErrorPath::new(x.into()))
    }

    fn write_i8(buf: &mut impl Write, x: i8) -> writer::Res {
        buf.write_i8(x).map_err(|x| ErrorPath::new(x.into()))
    }

    fn write_i16(buf: &mut impl Write, x: i16) -> writer::Res {
        buf.write_i16::<byteorder::LittleEndian>(x)
            .map_err(|x| ErrorPath::new(x.into()))
    }

    fn write_i32(buf: &mut impl Write, x: i32) -> writer::Res {
        let mut u = (x as u32) << 1;
        if x < 0 {
            u = !u;
        }
        while u >= 0x80 {
            Self::write_u8(buf, u as u8 | 0x80)?;
            u >>= 7;
        }
        Self::write_u8(buf, u as u8)?;
        Ok(())
    }

    fn write_i64(buf: &mut impl Write, x: i64) -> writer::Res {
        let mut u = (x as u64) << 1;
        if x < 0 {
            u = !u;
        }
        while u >= 0x80 {
            Self::write_u8(buf, u as u8 | 0x80)?;
            u >>= 7;
        }
        Self::write_u8(buf, u as u8)?;
        Ok(())
    }

    fn write_f32(buf: &mut impl Write, x: f32) -> writer::Res {
        buf.write_f32::<byteorder::LittleEndian>(x)
            .map_err(|x| ErrorPath::new(x.into()))
    }

    fn write_f64(buf: &mut impl Write, x: f64) -> writer::Res {
        buf.write_f64::<byteorder::LittleEndian>(x)
            .map_err(|x| ErrorPath::new(x.into()))
    }

    fn write_string(buf: &mut impl Write, x: &str) -> writer::Res {
        let modified_bytes = cesu8::to_java_cesu8(x);
        if modified_bytes.len() > i16::MAX as usize {
            return Err(ErrorPath::new(WriteError::SeqLengthViolation(
                i16::MAX as usize,
                modified_bytes.len(),
            )));
        }

        let mut l = modified_bytes.len() as u32;
        while l >= 0x80 {
            Self::write_u8(buf, l as u8 | 0x80)?;
            l >>= 7;
        }
        Self::write_u8(buf, l as u8)?;
        for b in modified_bytes.iter() {
            Self::write_u8(buf, *b)?;
        }
        Ok(())
    }
}

/// Test all encodings with various data.
#[cfg(test)]
mod tests {
    use crate::encoding::{BigEndian, LittleEndian, NetworkLittleEndian};
    use crate::reader::Reader;
    use crate::writer::Writer;
    use crate::{err, tag, NBTTag};

    #[test]
    fn test_big_endian() {
        test::<BigEndian>();
    }

    #[test]
    fn test_little_endian() {
        test::<LittleEndian>();
    }

    #[test]
    fn test_network_little_endian() {
        test::<NetworkLittleEndian>();
    }

    fn test<T: Reader + Writer>() {
        let nbt = tag::Compound::builder()
            .with_long("test", 10)
            .with_byte("test1", 100)
            .with_short("test2", 1)
            .with_list(
                "test3",
                vec![tag::ByteArray(vec![1, 2, 3]), tag::ByteArray(vec![4, 5, 6])],
            )
            .with_list("test4", vec![tag::Byte(1), tag::Byte(3)])
            .with("test5", tag::Compound::default());
        let nbt = NBTTag::Compound(nbt.build());
        let mut buf = vec![];
        nbt.write::<T>(&mut buf).unwrap();

        assert_eq!(NBTTag::read::<T>(&mut buf.as_slice()).unwrap(), nbt);
    }

    #[test]
    fn test_invalid_tagtype() {
        let valid_buf: Vec<u8> = vec![0x03, 0x00, 0x01, 0x61, 0x12, 0x34, 0x56, 0x78];
        let nbt = NBTTag::read::<BigEndian>(&mut valid_buf.as_slice()).unwrap();
        assert!(matches!(nbt, NBTTag::Int(tag::Int(0x12345678))));

        let invalid_buf: Vec<u8> = vec![0x15, 0x00, 0x01, 0x61, 0x12, 0x34, 0x56, 0x78];
        let nbt = NBTTag::read::<BigEndian>(&mut invalid_buf.as_slice());
        assert!(matches!(
            nbt,
            Err(err::ErrorPath {
                inner: err::ReadError::UnknownTagType(0x15),
                path: _
            })
        ))
    }

    #[test]
    fn test_modified_utf8() {
        let normal_string = vec![0x08, 0x00, 0x00, 0x00, 0x04, 0x6e, 0x61, 0x6d, 0x65];
        let nbt = NBTTag::read::<BigEndian>(&mut normal_string.as_slice()).unwrap();
        assert!(matches!(&nbt, NBTTag::String(tag::String::Utf8(x)) if x == "name"));
        let mut buf = vec![];
        nbt.write::<BigEndian>(&mut buf).unwrap();
        assert_eq!(normal_string, buf);

        let null_encoded_string = vec![0x08, 0x00, 0x00, 0x00, 0x04, 0xc0, 0x80, 0xc0, 0x80];
        let nbt = NBTTag::read::<BigEndian>(&mut null_encoded_string.as_slice()).unwrap();
        assert!(matches!(&nbt, NBTTag::String(tag::String::Utf8(x)) if x == "\0\0"));
        let mut buf = vec![];
        nbt.write::<BigEndian>(&mut buf).unwrap();
        assert_eq!(null_encoded_string, buf);

        let null_invalid_string = vec![0x08, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x80];
        let nbt = NBTTag::read::<BigEndian>(&mut null_invalid_string.as_slice()).unwrap();
        assert!(matches!(
            &nbt,
            NBTTag::String(tag::String::Bytes(x)) if matches!(x.as_slice(), [0x00, 0x00, 0x00, 0x80])
        ));
        let mut buf = vec![];
        nbt.write::<BigEndian>(&mut buf).unwrap();
        assert_eq!(null_invalid_string, buf);
    }
}
