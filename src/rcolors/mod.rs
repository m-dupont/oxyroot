use num_derive::FromPrimitive;
use num_derive::ToPrimitive;

#[derive(Debug)]
pub enum Error {
    CantMakeColorNamedFromInteger(i16),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Color {
    Named(ColorNamed),
    Int(i16),
}

impl Default for Color {
    fn default() -> Self {
        Self::Named(ColorNamed::default())
    }
}

impl Color {
    pub fn from_i16(i: i16) -> Self {
        match ColorNamed::from_i16(i) {
            Ok(ii) => Self::Named(ii),
            Err(_) => Self::Int(i),
        }
    }

    // pub fn to_i16(&self) -> i16 {
    //     match self {
    //         Self::Named(o) => num::ToPrimitive::to_i16(o).unwrap(),
    //         Self::Int(i) => *i,
    //     }
    // }
}

#[derive(FromPrimitive, ToPrimitive, Default, Debug)]
pub enum ColorNamed {
    #[default]
    White = 0,
    Black = 1,
    Yellow = 400,
    Green = 416,
    Cyan = 432,
    Blue = 600,
    Magenta = 616,
    Red = 632,
    Orange = 800,
    Spring = 820,
    Teal = 840,
    Azure = 860,
    Violet = 880,
    Pink = 900,
    Gray = 920,
}

impl ColorNamed {
    pub fn from_i16(i: i16) -> Result<Self> {
        num::FromPrimitive::from_i16(i).ok_or_else(|| Error::CantMakeColorNamedFromInteger(i))
    }

    // pub fn to_i16(&self) -> Result<i16> {
    //     num::ToPrimitive::to_i16(self).ok_or_else(|| anyhow!("Cant make a i16 from {:?}", self))
    // }
}
