use crate::error;
use crate::error::Result;
use serde::{ser, Serialize};
use std::io;
use std::io::Write;

pub struct Serializer<W, F = CompactFormatter> {
    writer: W,
    formatter: F,
}

impl<W: Write, F: Formatter> Serializer<W, F> {
    #[inline]
    pub fn with_formatter(writer: W, formatter: F) -> Self {
        Serializer { writer, formatter }
    }

    #[inline]
    pub fn into_inner(self) -> W {
        self.writer
    }
}

impl<W: Write> Serializer<W> {
    #[inline]
    pub fn new(writer: W) -> Self {
        Serializer::with_formatter(writer, CompactFormatter)
    }
}

pub trait Formatter {
    #[inline]
    fn write_null<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + Write,
    {
        writer.write_all(b"")
    }

    #[inline]
    fn write_bool<W>(&mut self, writer: &mut W, value: bool) -> io::Result<()>
    where
        W: ?Sized + Write,
    {
        let s = if value {
            b"true" as &[u8]
        } else {
            b"false" as &[u8]
        };
        writer.write_all(s)
    }

    #[inline]
    fn write_i8<W>(&mut self, writer: &mut W, value: i8) -> io::Result<()>
    where
        W: ?Sized + Write,
    {
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(value);
        writer.write_all(s.as_bytes())
    }

    #[inline]
    fn write_i16<W>(&mut self, writer: &mut W, value: i16) -> io::Result<()>
    where
        W: ?Sized + Write,
    {
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(value);
        writer.write_all(s.as_bytes())
    }

    #[inline]
    fn write_i32<W>(&mut self, writer: &mut W, value: i32) -> io::Result<()>
    where
        W: ?Sized + Write,
    {
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(value);
        writer.write_all(s.as_bytes())
    }

    #[inline]
    fn write_i64<W>(&mut self, writer: &mut W, value: i64) -> io::Result<()>
    where
        W: ?Sized + Write,
    {
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(value);
        writer.write_all(s.as_bytes())
    }

    #[inline]
    fn write_u8<W>(&mut self, writer: &mut W, value: u8) -> io::Result<()>
    where
        W: ?Sized + Write,
    {
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(value);
        writer.write_all(s.as_bytes())
    }

    #[inline]
    fn write_u16<W>(&mut self, writer: &mut W, value: u16) -> io::Result<()>
    where
        W: ?Sized + Write,
    {
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(value);
        writer.write_all(s.as_bytes())
    }

    #[inline]
    fn write_u32<W>(&mut self, writer: &mut W, value: u32) -> io::Result<()>
    where
        W: ?Sized + Write,
    {
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(value);
        writer.write_all(s.as_bytes())
    }

    #[inline]
    fn write_u64<W>(&mut self, writer: &mut W, value: u64) -> io::Result<()>
    where
        W: ?Sized + Write,
    {
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(value);
        writer.write_all(s.as_bytes())
    }

    #[inline]
    fn write_f32<W>(&mut self, writer: &mut W, value: f32) -> io::Result<()>
    where
        W: ?Sized + Write,
    {
        let mut buffer = ryu::Buffer::new();
        let s = buffer.format_finite(value);
        writer.write_all(s.as_bytes())
    }

    #[inline]
    fn write_f64<W>(&mut self, writer: &mut W, value: f64) -> io::Result<()>
    where
        W: ?Sized + Write,
    {
        let mut buffer = ryu::Buffer::new();
        let s = buffer.format_finite(value);
        writer.write_all(s.as_bytes())
    }

    #[inline]
    fn write_str<W>(&mut self, writer: &mut W, value: &str) -> io::Result<()>
    where
        W: ?Sized + Write,
    {
        writer.write_all(value.as_bytes())
    }

    #[inline]
    fn begin_key<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + Write,
    {
        writer.write_all(b"\n")
    }

    #[inline]
    fn end_key<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + Write,
    {
        writer.write_all(b"=")
    }

    #[inline]
    fn begin_value<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + Write,
    {
        writer.write_all(b"")
    }

    #[inline]
    fn end_value<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + Write,
    {
        writer.write_all(b"")
    }
}

impl<'a, W: Write, F: Formatter> ser::Serializer for &'a mut Serializer<W, F> {
    type Ok = ();
    type Error = error::Error;

    type SerializeSeq = Compound<'a, W, F>;
    type SerializeTuple = Compound<'a, W, F>;
    type SerializeTupleStruct = Compound<'a, W, F>;
    type SerializeTupleVariant = Compound<'a, W, F>;
    type SerializeMap = Compound<'a, W, F>;
    type SerializeStruct = Compound<'a, W, F>;
    type SerializeStructVariant = Compound<'a, W, F>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok> {
        self.formatter
            .write_bool(&mut self.writer, v)
            .map_err(ser::Error::custom)
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok> {
        self.formatter
            .write_i8(&mut self.writer, v)
            .map_err(ser::Error::custom)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok> {
        self.formatter
            .write_i16(&mut self.writer, v)
            .map_err(ser::Error::custom)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok> {
        self.formatter
            .write_i32(&mut self.writer, v)
            .map_err(ser::Error::custom)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok> {
        self.formatter
            .write_i64(&mut self.writer, v)
            .map_err(ser::Error::custom)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok> {
        self.formatter
            .write_u8(&mut self.writer, v)
            .map_err(ser::Error::custom)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok> {
        self.formatter
            .write_u16(&mut self.writer, v)
            .map_err(ser::Error::custom)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok> {
        self.formatter
            .write_u32(&mut self.writer, v)
            .map_err(ser::Error::custom)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok> {
        self.formatter
            .write_u64(&mut self.writer, v)
            .map_err(ser::Error::custom)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok> {
        self.formatter
            .write_f32(&mut self.writer, v)
            .map_err(ser::Error::custom)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok> {
        self.formatter
            .write_f64(&mut self.writer, v)
            .map_err(ser::Error::custom)
    }

    fn serialize_char(self, _v: char) -> Result<Self::Ok> {
        Err(ser::Error::custom("unsupported char"))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok> {
        self.formatter
            .write_str(&mut self.writer, v)
            .map_err(ser::Error::custom)
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok> {
        Err(ser::Error::custom("unsupported bytes"))
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        self.serialize_unit()
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        self.formatter
            .write_null(&mut self.writer)
            .map_err(ser::Error::custom)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok>
    where
        T: Serialize,
    {
        Err(ser::Error::custom("unsupported newtype struct"))
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok>
    where
        T: Serialize,
    {
        Err(ser::Error::custom("unsupported newtype variant"))
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Err(ser::Error::custom("unsupported seq"))
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Err(ser::Error::custom("unsupported tuple"))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Err(ser::Error::custom("unsupported tuple struct"))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Err(ser::Error::custom("unsupported tuple variant"))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(Compound::Map {
            ser: self,
            first: true,
        })
    }

    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Err(ser::Error::custom("unsupported struct variant"))
    }
}

#[derive(Clone, Debug)]
pub struct CompactFormatter;

impl Formatter for CompactFormatter {}

pub enum Compound<'a, W: 'a, F: 'a> {
    Map {
        ser: &'a mut Serializer<W, F>,
        first: bool,
    },
}

impl<'a, W: Write, F: Formatter> ser::SerializeSeq for Compound<'a, W, F> {
    type Ok = ();
    type Error = error::Error;

    fn serialize_element<T>(&mut self, _value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok> {
        unimplemented!()
    }
}

impl<'a, W: Write, F: Formatter> ser::SerializeTuple for Compound<'a, W, F> {
    type Ok = ();
    type Error = error::Error;

    fn serialize_element<T>(&mut self, _value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok> {
        unimplemented!()
    }
}

impl<'a, W: Write, F: Formatter> ser::SerializeTupleStruct for Compound<'a, W, F> {
    type Ok = ();
    type Error = error::Error;

    fn serialize_field<T>(&mut self, _value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok> {
        unimplemented!()
    }
}

impl<'a, W: Write, F: Formatter> ser::SerializeTupleVariant for Compound<'a, W, F> {
    type Ok = ();
    type Error = error::Error;

    fn serialize_field<T>(&mut self, _value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok> {
        unimplemented!()
    }
}

impl<'a, W: Write, F: Formatter> ser::SerializeMap for Compound<'a, W, F> {
    type Ok = ();
    type Error = error::Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        match self {
            Compound::Map { ser, first } => {
                if !*first {
                    ser.formatter
                        .begin_key(&mut ser.writer)
                        .map_err(ser::Error::custom)?;
                } else {
                    *first = false
                }

                key.serialize(&mut **ser)?;
                ser.formatter
                    .end_key(&mut ser.writer)
                    .map_err(ser::Error::custom)
            }
        }
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        match self {
            Compound::Map { ser, .. } => {
                ser.formatter
                    .begin_value(&mut ser.writer)
                    .map_err(ser::Error::custom)?;

                value.serialize(&mut **ser)?;
                ser.formatter
                    .end_value(&mut ser.writer)
                    .map_err(ser::Error::custom)
            }
        }
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

impl<'a, W: Write, F: Formatter> ser::SerializeStruct for Compound<'a, W, F> {
    type Ok = ();
    type Error = error::Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        match self {
            Compound::Map { .. } => ser::SerializeMap::serialize_entry(self, key, value),
        }
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

impl<'a, W: Write, F: Formatter> ser::SerializeStructVariant for Compound<'a, W, F> {
    type Ok = ();
    type Error = error::Error;

    fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok> {
        unimplemented!()
    }
}

#[inline]
pub fn to_writer<W, T>(writer: W, value: &T) -> Result<()>
where
    W: Write,
    T: ?Sized + Serialize,
{
    let mut ser = Serializer::new(writer);
    value.serialize(&mut ser)
}

#[inline]
pub fn to_vec<T>(value: &T) -> Result<Vec<u8>>
where
    T: ?Sized + Serialize,
{
    let mut writer = Vec::with_capacity(128);
    to_writer(&mut writer, value)?;
    Ok(writer)
}

#[inline]
pub fn to_string<T>(value: &T) -> Result<String>
where
    T: ?Sized + Serialize,
{
    let vec = to_vec(value)?;
    let string = unsafe { String::from_utf8_unchecked(vec) };
    Ok(string)
}
