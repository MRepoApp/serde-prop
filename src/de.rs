use alloc::vec::Vec;
use core::str;
use core::str::FromStr;

use serde::de;
use serde::de::{DeserializeSeed, Unexpected, Visitor};

use crate::error;
use crate::error::Result;
use crate::read::{Read, SliceRead, StrRead};

pub struct Deserializer<R> {
    read: R,
    inner: Vec<u8>,
}

impl<'de, R: Read<'de>> Deserializer<R> {
    pub fn new(read: R) -> Self {
        Deserializer {
            read,
            inner: Vec::new(),
        }
    }
}

impl<'a> Deserializer<SliceRead<'a>> {
    pub fn from_slice(bytes: &'a [u8]) -> Self {
        Deserializer::new(SliceRead::new(bytes))
    }
}

impl<'a> Deserializer<StrRead<'a>> {
    pub fn from_str(s: &'a str) -> Self {
        Deserializer::new(StrRead::new(s))
    }
}

impl<'de, R: Read<'de>> Deserializer<R> {
    fn next_char(&mut self) -> Option<u8> {
        self.read.next()
    }

    fn eat_char(&mut self) {
        self.read.discard()
    }

    fn peek(&mut self) -> Option<u8> {
        self.read.peek()
    }

    fn end(&mut self) -> Result<()> {
        match self.peek() {
            Some(_) => Err(de::Error::custom("not over yet")),
            None => Ok(()),
        }
    }

    fn parse_whitespace(&mut self) -> Option<u8> {
        loop {
            match self.next_char()? {
                b' ' | b'\n' | b'\t' | b'\r' => {}
                other => return Some(other),
            }
        }
    }

    fn parse_value(&mut self) -> Vec<u8> {
        if let Some(b' ') = self.peek() {
            self.eat_char()
        }

        let mut slice = Vec::new();
        loop {
            match self.next_char() {
                Some(b'\n' | b'\t' | b'\r') | None => {
                    return slice;
                }
                Some(b) => slice.push(b),
            };
        }
    }

    fn parse_comment(&mut self) -> Option<u8> {
        loop {
            match self.parse_whitespace()? {
                b'#' | b'!' => {
                    self.parse_value();
                }
                b => return Some(b),
            };
        }
    }

    fn parse_key(&mut self) -> Option<Vec<u8>> {
        let mut slice = Vec::new();
        loop {
            match self.parse_comment()? {
                b'=' | b':' => {
                    return Some(slice);
                }
                b => slice.push(b),
            };
        }
    }

    fn parse_str(&self) -> Option<&str> {
        match str::from_utf8(&self.inner).ok() {
            Some("") | None => None,
            Some(v) => Some(v),
        }
    }

    fn parse_any<T: FromStr>(&self) -> Option<T> {
        T::from_str(self.parse_str()?).ok()
    }
}

impl<'de, 'a, R: Read<'de>> de::Deserializer<'de> for &'a mut Deserializer<R> {
    type Error = error::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.parse_any() {
            Some(v) => visitor.visit_bool(v),
            None => {
                return Err(de::Error::invalid_value(
                    Unexpected::Str(self.parse_str().unwrap_or("")),
                    &"boolean",
                ))
            }
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_i64(visitor)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_i64(visitor)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_i64(visitor)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.parse_any() {
            Some(v) => visitor.visit_i64(v),
            None => {
                return Err(de::Error::invalid_value(
                    Unexpected::Str(self.parse_str().unwrap_or("")),
                    &"signed integer",
                ))
            }
        }
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_u64(visitor)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_u64(visitor)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_u64(visitor)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.parse_any() {
            Some(v) => visitor.visit_u64(v),
            None => {
                return Err(de::Error::invalid_value(
                    Unexpected::Str(self.parse_str().unwrap_or("")),
                    &"unsigned integer",
                ))
            }
        }
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_f64(visitor)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.parse_any() {
            Some(v) => visitor.visit_f64(v),
            None => {
                return Err(de::Error::invalid_value(
                    Unexpected::Str(self.parse_str().unwrap_or("")),
                    &"float",
                ))
            }
        }
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(de::Error::custom("unsupported char"))
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.parse_str() {
            None => Err(de::Error::invalid_length(0, &"length > 0")),
            Some(v) => visitor.visit_str(v),
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(de::Error::custom("unsupported bytes"))
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_bytes(visitor)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.parse_str() {
            None => visitor.visit_none(),
            Some(_) => visitor.visit_some(self),
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.parse_str() {
            None => visitor.visit_unit(),
            Some(_) => Err(de::Error::invalid_type(Unexpected::Unit, &"unit")),
        }
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(de::Error::custom("unsupported newtype struct"))
    }

    fn deserialize_seq<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(de::Error::custom("unsupported seq"))
    }

    fn deserialize_tuple<V>(self, _len: usize, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(de::Error::custom("unsupported tuple"))
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(de::Error::custom("unsupported tuple struct"))
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_map(MapAccess::new(self))
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_enum(UnitVariantAccess::new(self))
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
}

struct MapAccess<'a, R: 'a> {
    de: &'a mut Deserializer<R>,
}

impl<'a, R: 'a> MapAccess<'a, R> {
    fn new(de: &'a mut Deserializer<R>) -> Self {
        MapAccess { de }
    }
}

impl<'de, 'a, R: Read<'de> + 'a> de::MapAccess<'de> for MapAccess<'a, R> {
    type Error = error::Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        let key = match self.de.parse_key() {
            None => return Ok(None),
            Some(b) => b,
        };

        self.de.inner.clear();
        self.de.inner.extend(key);
        seed.deserialize(&mut *self.de).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        let value = self.de.parse_value();

        self.de.inner.clear();
        self.de.inner.extend(value);
        seed.deserialize(&mut *self.de)
    }
}

struct UnitVariantAccess<'a, R: 'a> {
    de: &'a mut Deserializer<R>,
}

impl<'a, R: 'a> UnitVariantAccess<'a, R> {
    fn new(de: &'a mut Deserializer<R>) -> Self {
        UnitVariantAccess { de }
    }
}

impl<'de, 'a, R: Read<'de> + 'a> de::EnumAccess<'de> for UnitVariantAccess<'a, R> {
    type Error = error::Error;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self)>
    where
        V: DeserializeSeed<'de>,
    {
        let variant = seed.deserialize(&mut *self.de)?;
        Ok((variant, self))
    }
}

impl<'de, 'a, R: Read<'de> + 'a> de::VariantAccess<'de> for UnitVariantAccess<'a, R> {
    type Error = error::Error;

    fn unit_variant(self) -> Result<()> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, _seed: T) -> Result<T::Value>
    where
        T: DeserializeSeed<'de>,
    {
        Err(de::Error::invalid_type(
            Unexpected::UnitVariant,
            &"newtype variant",
        ))
    }

    fn tuple_variant<V>(self, _len: usize, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(de::Error::invalid_type(
            Unexpected::UnitVariant,
            &"tuple variant",
        ))
    }

    fn struct_variant<V>(self, _fields: &'static [&'static str], _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(de::Error::invalid_type(
            Unexpected::UnitVariant,
            &"struct variant",
        ))
    }
}

fn from_trait<'de, R, T>(read: R) -> Result<T>
where
    R: Read<'de>,
    T: de::Deserialize<'de>,
{
    let mut de = Deserializer::new(read);
    let value = de::Deserialize::deserialize(&mut de)?;

    de.end()?;
    Ok(value)
}

pub fn from_slice<'a, T: de::Deserialize<'a>>(v: &'a [u8]) -> Result<T> {
    from_trait(SliceRead::new(v))
}

pub fn from_str<'a, T: de::Deserialize<'a>>(s: &'a str) -> Result<T> {
    from_trait(StrRead::new(s))
}
