use crate::{colors::RGBAColor, layer::Layer, filters::LayerFilter};

use std::{io::BufRead, collections::HashMap};
use toml::Value;

pub type LayerFilterTuple = Vec<(Box<dyn Layer>, Vec<Box<dyn LayerFilter>>)>;
pub trait LayerParser {
    fn parse<R: BufRead, L: ConfigDeserializer<Box<dyn Layer>>, F: ConfigDeserializer<Box<dyn LayerFilter>>>(reader: R) -> LayerFilterTuple;
}

#[derive(Debug)]
pub enum ParsedArgs {
    String(String),
    Integer(i64),
    Float(f64),
    RGBAColor(RGBAColor),
    Boolean(bool)
}

impl ParsedArgs {
    pub fn as_string(&self) -> Option<String> {
        match self {
            ParsedArgs::String(s) => Some(s.clone()),
            _ => None
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            ParsedArgs::String(s) => Some(s),
            _ => None
        }
    }

    pub fn as_int(&self) -> Option<i64> {
        match self {
            ParsedArgs::Integer(i) => Some(*i),
            ParsedArgs::Float(f) => Some(f.clone() as i64),
            _ => None
        }
    }

    pub fn as_float(&self) -> Option<f64> {
        match self {
            ParsedArgs::Float(f) => Some(*f),
            ParsedArgs::Integer(i) => Some(i.clone() as f64),
            _ => None
        }
    }

    pub fn as_rgb_color(&self) -> Option<RGBAColor> {
        match self {
            ParsedArgs::RGBAColor(c) => Some(c.clone()),
            _ => None
        }
    }
}

impl TryFrom<toml::Value> for ParsedArgs {
    type Error = ();

    fn try_from(v: toml::Value) -> Result<Self, Self::Error> {
        match v {
            Value::String(s) => Ok(Self::String(s)),
            Value::Integer(i) => Ok(Self::Integer(i)),
            Value::Float(f) => Ok(Self::Float(f)),
            Value::Boolean(b) => Ok(Self::Boolean(b)),
            _ => Err(())
        }
    }
}

pub trait ConfigDeserializer<T> {
    fn from_str_and_args(from: &str, args: HashMap<String, ParsedArgs>) -> T;
}
