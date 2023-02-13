use std::fmt::Display;

use serde::de::{self, Unexpected};

use super::EOT;

#[derive(Debug, PartialEq)]
pub enum Error {
    Custom(String),
    DeserializeAny,
    WrongDeserializeType,
    EotNotFound,
    EmptyBuffer,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

impl de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self::Custom(format!("{}", msg))
    }
}

pub struct Deserializer<'a> {
    buffer: &'a [u8],
}

impl<'a> Deserializer<'a> {
    pub fn new(buffer: &'a [u8]) -> Self {
        Self { buffer }
    }

    unsafe fn deserialize_integer<Integer, const SIZE: usize, V>(
        self,
        visitor: &V,
    ) -> Result<Integer, <Self as serde::Deserializer<'a>>::Error>
    where
        Integer: Sized,
        V: serde::de::Visitor<'a>,
    {
        if !self.buffer[SIZE..].is_empty() {
            return Err(Error::WrongDeserializeType);
        }

        let value = self
            .buffer
            .get(0..SIZE)
            .ok_or(<Error as de::Error>::invalid_length(
                self.buffer.len(),
                visitor,
            ))?;
        let value = <[u8; SIZE]>::try_from(value).map_err(<Error as de::Error>::custom)?;

        Ok(std::mem::transmute_copy::<[u8; SIZE], Integer>(&value))
    }

    fn deserialize_str(self) -> Result<&'a str, <Self as serde::Deserializer<'a>>::Error> {
        if self.buffer.is_empty() || self.buffer[self.buffer.len() - 1] != EOT {
            return Err(Error::EotNotFound);
        }

        std::str::from_utf8(&self.buffer[..self.buffer.len() - 1]).map_err(de::Error::custom)
    }
}

impl<'de, 'a: 'de> serde::Deserializer<'de> for Deserializer<'a> {
    type Error = Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::DeserializeAny)
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        const SIZE: usize = std::mem::size_of::<u8>();
        let value = match unsafe { self.deserialize_integer::<u8, SIZE, V>(&visitor) }? {
            0 => false,
            1 => true,
            n => {
                return Err(de::Error::invalid_value(
                    Unexpected::Unsigned(n as u64),
                    &visitor,
                ))
            }
        };
        visitor.visit_bool(value)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        const SIZE: usize = std::mem::size_of::<i8>();
        let value = unsafe { self.deserialize_integer::<i8, SIZE, V>(&visitor) }?;
        visitor.visit_i8(value)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        const SIZE: usize = std::mem::size_of::<i16>();
        let value = unsafe { self.deserialize_integer::<i16, SIZE, V>(&visitor) }?;
        visitor.visit_i16(value)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        const SIZE: usize = std::mem::size_of::<i32>();
        let value = unsafe { self.deserialize_integer::<i32, SIZE, V>(&visitor) }?;
        visitor.visit_i32(value)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        const SIZE: usize = std::mem::size_of::<i64>();
        let value = unsafe { self.deserialize_integer::<i64, SIZE, V>(&visitor) }?;
        visitor.visit_i64(value)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        const SIZE: usize = std::mem::size_of::<u8>();
        let value = unsafe { self.deserialize_integer::<u8, SIZE, V>(&visitor) }?;
        visitor.visit_u8(value)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        const SIZE: usize = std::mem::size_of::<u16>();
        let value = unsafe { self.deserialize_integer::<u16, SIZE, V>(&visitor) }?;
        visitor.visit_u16(value)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        const SIZE: usize = std::mem::size_of::<u32>();
        let value = unsafe { self.deserialize_integer::<u32, SIZE, V>(&visitor) }?;
        visitor.visit_u32(value)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        const SIZE: usize = std::mem::size_of::<u64>();
        let value = unsafe { self.deserialize_integer::<u64, SIZE, V>(&visitor) }?;
        visitor.visit_u64(value)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        const SIZE: usize = std::mem::size_of::<u32>();
        let value = unsafe { self.deserialize_integer::<u32, SIZE, V>(&visitor) }?;
        visitor.visit_f32(f32::from_bits(value))
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        const SIZE: usize = std::mem::size_of::<u64>();
        let value = unsafe { self.deserialize_integer::<u64, SIZE, V>(&visitor) }?;
        visitor.visit_f64(f64::from_bits(value))
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        const SIZE: usize = std::mem::size_of::<u32>();
        let value = unsafe { self.deserialize_integer::<u32, SIZE, V>(&visitor) }?;
        let value = char::from_u32(value).ok_or(<Error as de::Error>::invalid_value(
            Unexpected::Unsigned(value as u64),
            &visitor,
        ))?;
        visitor.visit_char(value)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_str(self.deserialize_str()?)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_string(self.deserialize_str()?.to_string())
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_bytes(self.buffer)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_byte_buf(self.buffer.to_vec())
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match *self.buffer.get(0).ok_or(Error::EmptyBuffer)? {
            0 => visitor.visit_none(),
            1 => visitor.visit_some(Self {
                buffer: &self.buffer[1..],
            }),
            n => Err(de::Error::invalid_value(
                Unexpected::Unsigned(n as u64),
                &visitor,
            )),
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        if !self.buffer.is_empty() {
            return Err(Error::WrongDeserializeType);
        }

        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }
}
