use int_enum::IntEnum;
use enum_iterator::Sequence;

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, IntEnum, Sequence)]
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
    White = 9,
}

pub fn color_to_value(color: ResistorColor) -> u32 {
    color.int_value().into()
}

pub fn value_to_color_string(value: u32) -> String {
    if let Ok(color) = ResistorColor::from_int(value as u8) {
        format!("{:?}", color)
    } else {
        "value out of range".to_string()
    }
}

pub fn colors() -> Vec<ResistorColor> {
    enum_iterator::all::<ResistorColor>().collect()
}
