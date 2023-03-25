use crate::{colors::RGBAColor, layer::Layer, filters::LayerFilter, canvas::Canvas};

use std::{io::BufRead, collections::HashMap};
use toml::Value;

pub type LayerFilterTuple = Vec<(Box<dyn Layer>, Vec<Box<dyn LayerFilter>>)>;
pub trait LayerParser {
    fn parse<R: BufRead, L: ConfigDeserializer<Box<dyn Layer>>, F: ConfigDeserializer<Box<dyn LayerFilter>>>(reader: R) -> Canvas;
}

#[derive(Debug)]
pub enum ParsedArgs {
    String(String),
    Integer(i32),
    Float(f64),
    RGBAColor(RGBAColor),
    Coord2D((i32, i32)),
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

    pub fn as_int(&self) -> Option<i32> {
        match self {
            ParsedArgs::Integer(i) => Some(*i),
            ParsedArgs::Float(f) => Some(f.clone() as i32),
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
            ParsedArgs::String(s) => {
                let inner = s.trim_start_matches('(').trim_end_matches(')');
                let mut numbers = inner.split(',');

                let mut numbers_parsed: [f64; 4] = [255.0; 4];
                for n in 0..4 {
                    let next_num_str = numbers.next();

                    if next_num_str.is_none() {
                        break;
                    }
                    
                    let parsed = next_num_str.unwrap().trim().parse();

                    if parsed.is_err() {
                        return None
                    }

                    numbers_parsed[n] = parsed.unwrap();
                }

                Some(RGBAColor(numbers_parsed[0], numbers_parsed[1], numbers_parsed[2], numbers_parsed[3]))
            }
            _ => None
        }
    }

    pub fn as_coord_2d(&self) -> Option<(i32, i32)> {
        match self {
            ParsedArgs::Coord2D(c) => Some(c.clone()),
            ParsedArgs::String(s) => {
                let inner = s.trim_start_matches("(").trim_end_matches(")");
                let mut nums = inner.split(",");

                Some((nums.next()?.trim().parse().ok()?, nums.next()?.trim().parse().ok()?))
            },
            _ => None
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            ParsedArgs::Boolean(b) => Some(b.clone()),
            ParsedArgs::String(s) => s.parse().ok(),
            _ => None
        }
    }
}

impl TryFrom<toml::Value> for ParsedArgs {
    type Error = ();

    fn try_from(v: toml::Value) -> Result<Self, Self::Error> {
        match v {
            Value::String(s) => Ok(Self::String(s)),
            Value::Integer(i) => Ok(
                Self::Integer(
                    i.try_into().ok().ok_or(())?
                )
            ),
            Value::Float(f) => Ok(Self::Float(f)),
            Value::Boolean(b) => Ok(Self::Boolean(b)),
            _ => Err(())
        }
    }
}

pub trait ConfigDeserializer<T> {
    fn from_str_and_args(from: &str, args: HashMap<String, ParsedArgs>) -> T;
}
