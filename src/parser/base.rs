use crate::colors::RGBAColor;

pub enum ParsedArgs {
    String(String),
    Integer(i32),
    Float(f64),
    RGBAColor(RGBAColor)
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
            ParsedArgs::Integer(i) => Some(i.clone().into()),
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
