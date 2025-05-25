# Overview

Read and write [NBT](https://wiki.vg/NBT) data.

Named Binary Tag (NBT) is a structured binary format used throughout Minecraft for a multitude
of things. This crate mainly focuses on Minecraft: Bedrock Edition, and supports the
**little endian** and **network little endian** encoding. **Big endian**, which is more commonly
used in Minecraft: Java Edition, is also supported, however.

## Feature flags

 - `serde` - Allows rust types to be serialized and deserialized into NBT using [serde](https://serde.rs/).

## Examples

NBT data can be constructed and written as follows:

```rust
use indexmap::IndexMap;
use zuri_nbt::encoding::LittleEndian;
use zuri_nbt::NBTTag;

let mut nbt = IndexMap::new();
nbt.insert("name".to_string(), NBTTag::String("Zuri".to_string().into()));
nbt.insert("age".to_string(), NBTTag::Int(18.into()));

let mut buf = vec![];
NBTTag::Compound(nbt.into()).write::<LittleEndian>(&mut buf)
    .expect("Something went wrong while writing nbt");
 ```

Reading NBT data can be done as follows:

 ```rust
use zuri_nbt::encoding::LittleEndian;
use zuri_nbt::NBTTag;

let mut buf = vec![
    0x08, 0x00, 0x00, 0x0c,
    0x00, 0x48, 0x65, 0x6c,
    0x6c, 0x6f, 0x20, 0x57,
    0x6f, 0x72, 0x6c, 0x64,
    0x21, 0x00, 0x00, 0x00,
];

let value = NBTTag::read::<LittleEndian>(&mut buf.as_slice())
    .expect("Something went wrong while reading nbt");
assert_eq!(value, NBTTag::String("Hello World!".to_string().into()));
 ```
