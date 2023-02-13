use std::fmt::Display;

use serde::{
    ser::{
        SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
        SerializeTupleStruct, SerializeTupleVariant,
    },
    Serialize,
};

use super::EOT;

#[derive(Debug, PartialEq)]
pub enum Error {
    Custom(String),
    UnsizedSeq,
    UnsizedMap,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

impl serde::ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self::Custom(format!("{}", msg))
    }
}

#[derive(Default)]
pub struct Serializer {
    buffer: Vec<u8>,
}

impl Serializer {
    fn serialize_single_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Error>
    where
        T: Serialize,
    {
        Ok(self
            .buffer
            .extend_from_slice(value.serialize(Self::default())?.as_slice()))
    }
}

impl serde::Serializer for Serializer {
    type Ok = Vec<u8>;

    type Error = Error;

    type SerializeSeq = Self;

    type SerializeTuple = Self;

    type SerializeTupleStruct = Self;

    type SerializeTupleVariant = Self;

    type SerializeMap = Self;

    type SerializeStruct = Self;

    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(vec![if v { 1 } else { 0 }])
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        (v as u8).serialize(self)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        (v as u16).serialize(self)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        (v as u32).serialize(self)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        (v as u64).serialize(self)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Ok(vec![v])
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Ok(unsafe { std::mem::transmute::<_, [u8; 2]>(v) }.to_vec())
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Ok(unsafe { std::mem::transmute::<_, [u8; 4]>(v) }.to_vec())
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Ok(unsafe { std::mem::transmute::<_, [u8; 8]>(v) }.to_vec())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        v.to_bits().serialize(self)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        v.to_bits().serialize(self)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        (v as u32).serialize(self)
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        [v.as_bytes(), &[EOT]].concat().serialize(self)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_vec())
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        false.serialize(self)
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        let mut output = true.serialize(Self::default())?;
        output.extend(value.serialize(self)?);
        Ok(output)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(vec![])
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        variant_index.serialize(self)
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        let mut output = variant_index.serialize(Self::default())?;
        output.extend(value.serialize(self)?);
        Ok(output)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        match len {
            None => Err(Error::UnsizedSeq),
            Some(len) => Ok(Self {
                buffer: len.serialize(self)?,
            }),
        }
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(Self::default())
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(Self {
            buffer: len.serialize(self)?,
        })
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        let mut buffer = variant_index.serialize(self)?;
        buffer.extend(len.serialize(Self::default())?);

        Ok(Self { buffer })
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        match len {
            None => Err(Error::UnsizedMap),
            Some(len) => Ok(Self {
                buffer: len.serialize(self)?,
            }),
        }
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(Self::default())
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(Self {
            buffer: variant_index.serialize(self)?,
        })
    }
}

impl SerializeSeq for Serializer {
    type Ok = Vec<u8>;

    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.serialize_single_element(value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.buffer)
    }
}

impl SerializeTuple for Serializer {
    type Ok = Vec<u8>;

    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.serialize_single_element(value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.buffer)
    }
}

impl SerializeTupleStruct for Serializer {
    type Ok = Vec<u8>;

    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.serialize_single_element(value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.buffer)
    }
}

impl SerializeTupleVariant for Serializer {
    type Ok = Vec<u8>;

    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.serialize_single_element(value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.buffer)
    }
}

impl SerializeMap for Serializer {
    type Ok = Vec<u8>;

    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.serialize_single_element(key)
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.serialize_single_element(value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.buffer)
    }
}

impl SerializeStruct for Serializer {
    type Ok = Vec<u8>;

    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        _key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.serialize_single_element(value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.buffer)
    }
}

impl SerializeStructVariant for Serializer {
    type Ok = Vec<u8>;

    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        _key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.serialize_single_element(value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.buffer)
    }
}
