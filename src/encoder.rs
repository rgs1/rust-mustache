use std::collections::HashMap;
use std::fmt;
use std::io::IoError as StdIoError;
use serialize;

use super::{Data, StrVal, Bool, VecVal, Map};
pub use self::Error::*;

pub struct Encoder<'a> {
    pub data: Vec<Data<'a>>,
}

impl<'a> Encoder<'a> {
    pub fn new() -> Encoder<'a> {
        Encoder { data: Vec::new() }
    }
}

#[deriving(PartialEq)]
pub enum Error {
    UnsupportedType,
    InvalidStr,
    MissingElements,
    KeyIsNotString,
    IoError(StdIoError),
}

impl fmt::Show for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UnsupportedType => "unsupported type".fmt(f),
            InvalidStr => "invalid str".fmt(f),
            MissingElements => "no elements in value".fmt(f),
            KeyIsNotString => "key is not a string".fmt(f),
            IoError(ref err) => err.fmt(f),
        }
    }
}

pub type EncoderResult = Result<(), Error>;

impl<'a> serialize::Encoder<Error> for Encoder<'a> {
    fn emit_nil(&mut self) -> EncoderResult { Err(UnsupportedType) }

    fn emit_uint(&mut self, v: uint) -> EncoderResult { self.data.push(StrVal(v.to_string())); Ok(()) }
    fn emit_u64(&mut self, v: u64) -> EncoderResult   { self.data.push(StrVal(v.to_string())); Ok(()) }
    fn emit_u32(&mut self, v: u32) -> EncoderResult   { self.data.push(StrVal(v.to_string())); Ok(()) }
    fn emit_u16(&mut self, v: u16) -> EncoderResult   { self.data.push(StrVal(v.to_string())); Ok(()) }
    fn emit_u8(&mut self, v: u8) -> EncoderResult     { self.data.push(StrVal(v.to_string())); Ok(()) }

    fn emit_int(&mut self, v: int) -> EncoderResult { self.data.push(StrVal(v.to_string())); Ok(()) }
    fn emit_i64(&mut self, v: i64) -> EncoderResult { self.data.push(StrVal(v.to_string())); Ok(()) }
    fn emit_i32(&mut self, v: i32) -> EncoderResult { self.data.push(StrVal(v.to_string())); Ok(()) }
    fn emit_i16(&mut self, v: i16) -> EncoderResult { self.data.push(StrVal(v.to_string())); Ok(()) }
    fn emit_i8(&mut self, v: i8) -> EncoderResult   { self.data.push(StrVal(v.to_string())); Ok(()) }

    fn emit_bool(&mut self, v: bool) -> EncoderResult { self.data.push(Bool(v)); Ok(()) }

    fn emit_f64(&mut self, v: f64) -> EncoderResult { self.data.push(StrVal(v.to_string())); Ok(()) }
    fn emit_f32(&mut self, v: f32) -> EncoderResult { self.data.push(StrVal(v.to_string())); Ok(()) }

    fn emit_char(&mut self, v: char) -> EncoderResult {
        self.data.push(StrVal(String::from_char(1, v)));
        Ok(())
    }
    fn emit_str(&mut self, v: &str) -> EncoderResult { self.data.push(StrVal(v.to_string())); Ok(()) }

    fn emit_enum< F >(&mut self, _name: &str, _f: F ) -> EncoderResult
    where F : FnOnce(&mut Encoder<'a>) -> EncoderResult {
        Err(UnsupportedType)
    }

    fn emit_enum_variant< F >(&mut self,
                         _name: &str,
                         _id: uint,
                         _len: uint,
                         _f: F) -> EncoderResult
                         where F : FnOnce(&mut Encoder<'a>) -> EncoderResult {
        Err(UnsupportedType)
    }

    fn emit_enum_variant_arg< F >(&mut self,
                             _a_idx: uint,
                             _f: F) -> EncoderResult
                             where F : FnOnce(&mut Encoder<'a>) -> EncoderResult {
        Err(UnsupportedType)
    }

    fn emit_enum_struct_variant< F >(&mut self,
                                _v_name: &str,
                                _v_id: uint,
                                _len: uint,
                                _f: F) -> EncoderResult
                                where F : FnOnce(&mut Encoder<'a>) -> EncoderResult {
        Err(UnsupportedType)
    }

    fn emit_enum_struct_variant_field< F >(&mut self,
                                      _f_name: &str,
                                      _f_idx: uint,
                                      _f: F) -> EncoderResult
                                      where F : FnOnce(&mut Encoder<'a>) -> EncoderResult  {
        Err(UnsupportedType)
    }

    fn emit_struct< F >(&mut self,
                   _name: &str,
                   _len: uint,
                   f: F) -> EncoderResult
                   where F : FnOnce(&mut Encoder<'a>) -> EncoderResult  {
        self.data.push(Map(HashMap::new()));
        f(self)
    }

    fn emit_struct_field< F >(&mut self,
                         name: &str,
                         _idx: uint,
                         f: F) -> EncoderResult
                         where F : FnOnce(&mut Encoder<'a>) -> EncoderResult {
        let mut m = match self.data.pop() {
            Some(Map(m)) => m,
            _ => { return Err(UnsupportedType); }
        };
        try!(f(self));
        let data = match self.data.pop() {
            Some(d) => d,
            _ => { return Err(UnsupportedType); }
        };
        m.insert(name.to_string(), data);
        self.data.push(Map(m));
        Ok(())
    }

    fn emit_tuple< F >(&mut self,
                  len: uint,
                  f: F) -> EncoderResult
                  where F : FnOnce(&mut Encoder<'a>) -> EncoderResult {
        self.emit_seq(len, f)
    }

    fn emit_tuple_arg< F >(&mut self, idx: uint, f: F) -> EncoderResult
    where F : FnOnce(&mut Encoder<'a>) -> EncoderResult {
        self.emit_seq_elt(idx, f)
    }

    fn emit_tuple_struct< F >(&mut self,
                         _name: &str,
                         len: uint,
                         f: F) -> EncoderResult
                         where F : FnOnce(&mut Encoder<'a>) -> EncoderResult {
        self.emit_seq(len, f)
    }

    fn emit_tuple_struct_arg< F >(&mut self, idx: uint, f: F) -> EncoderResult
    where F : FnOnce(&mut Encoder<'a>) -> EncoderResult  {
        self.emit_seq_elt(idx, f)
    }

    // Specialized types:
    fn emit_option< F >(&mut self, _f: F) -> EncoderResult
    where F : FnOnce(&mut Encoder<'a>) -> EncoderResult  {
        Err(UnsupportedType)
    }

    fn emit_option_none(&mut self) -> EncoderResult {
        Err(UnsupportedType)
    }

    fn emit_option_some< F >(&mut self, _f: F) -> EncoderResult
  where F : FnOnce(&mut Encoder<'a>) -> EncoderResult {
        Err(UnsupportedType)
    }

    fn emit_seq< F >(&mut self, _len: uint, f: F) -> EncoderResult
  where F : FnOnce(&mut Encoder<'a>) -> EncoderResult {
        self.data.push(VecVal(Vec::new()));
        f(self)
    }

    fn emit_seq_elt< F >(&mut self, _idx: uint, f: F) -> EncoderResult
  where F : FnOnce(&mut Encoder<'a>) -> EncoderResult {
        let mut v = match self.data.pop() {
            Some(VecVal(v)) => v,
            _ => { return Err(UnsupportedType); }
        };
        try!(f(self));
        let data = match self.data.pop() {
            Some(d) => d,
            _ => { return Err(UnsupportedType); }
        };
        v.push(data);
        self.data.push(VecVal(v));
        Ok(())
    }

    fn emit_map< F >(&mut self, _len: uint, f: F) -> EncoderResult
  where F : FnOnce(&mut Encoder<'a>) -> EncoderResult {
        self.data.push(Map(HashMap::new()));
        f(self)
    }

    fn emit_map_elt_key< F >(&mut self, _idx: uint, mut f: F) -> EncoderResult
  where F : FnMut(&mut Encoder<'a>) -> EncoderResult {
        try!(f(self));
        let last = match self.data.last() {
            Some(d) => d,
            None => { return Err(MissingElements); }
        };
        match *last {
            StrVal(_) => Ok(()),
            _ => Err(KeyIsNotString),
        }
    }

    fn emit_map_elt_val< F >(&mut self, _idx: uint, f: F) -> EncoderResult
  where F : FnOnce(&mut Encoder<'a>) -> EncoderResult {
        let k = match self.data.pop() {
            Some(StrVal(s)) => s,
            _ => { return Err(KeyIsNotString); }
        };
        let mut m = match self.data.pop() {
            Some(Map(m)) => m,
            _ => panic!("Expected a map"),
        };
        try!(f(self));
        let popped = match self.data.pop() {
            Some(p) => p,
            None => panic!("Error: Nothing to pop!"),
        };
        m.insert(k, popped);
        self.data.push(Map(m));
        Ok(())
    }
}

pub fn encode<'a, T: serialize::Encodable<Encoder<'a>, Error>>(data: &T) -> Result<Data<'a>, Error> {
    let mut encoder = Encoder::new();
    try!(data.encode(&mut encoder))
    assert_eq!(encoder.data.len(), 1);
    match encoder.data.pop() {
        Some(data) => Ok(data),
        None => panic!("Error: Nothing to pop!"),
    }
}
