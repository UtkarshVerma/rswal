use std::fmt::Debug;

use crate::util::Error;
use palette::{self, Darken, Lighten, Srgb};

#[derive(Error, Debug)]
pub enum ColorError {
    #[error("invalid hex format: should be #RRGGBB")]
    InvalidHex,
}

pub struct Color {
    value: Srgb<f32>,
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Color {
            value: palette::Srgb::new(
                red as f32 / 255.0,
                green as f32 / 255.0,
                blue as f32 / 255.0,
            ),
        }
    }

    pub fn from_hex(hex: &str) -> Result<Self, ColorError> {
        if hex.len() != 7 || !hex.starts_with('#') {
            return Err(ColorError::InvalidHex);
        }

        let red = u8::from_str_radix(&hex[1..3], 16);
        let green = u8::from_str_radix(&hex[3..5], 16);
        let blue = u8::from_str_radix(&hex[5..7], 16);

        match (red, green, blue) {
            (Ok(red), Ok(green), Ok(blue)) => Ok(Color::new(red, green, blue)),
            _ => Err(ColorError::InvalidHex),
        }
    }

    pub fn to_hex(&self) -> String {
        let color: Srgb<u8> = self.value.into_format();

        format!("#{:02x}{:02x}{:02x}", color.red, color.green, color.blue)
    }

    pub fn lighten(&self, amount: f32) -> Self {
        Color {
            value: self.value.lighten(amount),
        }
    }

    pub fn darken(&self, amount: f32) -> Self {
        Color {
            value: self.value.darken(amount),
        }
    }
}

#[test]
fn test_color_parse_and_render() {
    let color = Color::from_hex("#ffffff").unwrap();
    assert_eq!(color.value, palette::Srgb::new(1.0, 1.0, 1.0));

    let color = Color::from_hex("#ffffff").unwrap();
    assert_eq!(color.to_hex(), "#ffffff");
}

#[test]
fn test_color_transforms() {
    let color = Color::from_hex("#000000").unwrap().lighten(0.5);
    assert_eq!(color.value, palette::Srgb::new(0.5, 0.5, 0.5));

    let color = Color::from_hex("#ffffff").unwrap().darken(0.5);
    assert_eq!(color.value, palette::Srgb::new(0.5, 0.5, 0.5));
}
