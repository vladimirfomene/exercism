use int_enum::IntEnum;
use enum_iterator::{all, Sequence};
use std::fmt;

#[repr(u32)]
#[derive(Debug, PartialEq, Eq, IntEnum, Sequence, Copy, Clone)]
pub enum ResistorColor {
    Black = 0,
    Brown = 1,
    Red = 2,
    Orange = 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Violet = 7,
    Grey = 8,
    White = 9
}

impl fmt::Display for ResistorColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResistorColor::Black => write!(f, "Black"),
            ResistorColor::Blue => write!(f, "Blue"),
            ResistorColor::Brown => write!(f, "Brown"),
            ResistorColor::Green => write!(f, "Green"),
            ResistorColor::Grey => write!(f, "Grey"),
            ResistorColor::Orange => write!(f, "Orange"),
            ResistorColor::Red => write!(f, "Red"),
            ResistorColor::Violet => write!(f, "Violet"),
            ResistorColor::White => write!(f, "White"),
            ResistorColor::Yellow => write!(f, "Yellow")
        }
    }
}

pub fn color_to_value(_color: ResistorColor) -> u32 {
    _color.int_value()
}

pub fn value_to_color_string(value: u32) -> String {
    match ResistorColor::from_int(value) {
        Ok(color) => color.to_string(),
        Err(_e) => String::from("value out of range")
    }
}

pub fn colors() -> Vec<ResistorColor> {
    all::<ResistorColor>().collect::<Vec<_>>()
}
