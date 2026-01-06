#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]
use std::ops::Add;
use std::ops::Deref;

#[derive(Clone, Debug, Default, PartialEq, Copy)]
pub struct Pixel {
    pub color: Color,
    pub alpha: f32,
}

#[derive(Clone, Debug, PartialEq, Copy, Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn from_hsv(hue: impl Into<f32>, saturation: f32, value: f32) -> Self {
        let hue_degrees = hue.into().clamp(0.0, 360.0);
        let hue_sector = hue_degrees / 60.0;

        let saturation = saturation.clamp(0.0, 1.0);
        let value = value.clamp(0.0, 1.0);

        let sector_index = hue_sector.floor() as u32;
        let sector_fraction = hue_sector - hue_sector.floor();

        let chroma_min = value * (1.0 - saturation);
        let chroma_q = value * (1.0 - sector_fraction * saturation);
        let chroma_t = value * (1.0 - (1.0 - sector_fraction) * saturation);

        let (red, green, blue) = match sector_index % 6 {
            0 => (value, chroma_t, chroma_min),
            1 => (chroma_q, value, chroma_min),
            2 => (chroma_min, value, chroma_t),
            3 => (chroma_min, chroma_q, value),
            4 => (chroma_t, chroma_min, value),
            5 => (value, chroma_min, chroma_q),
            _ => unreachable!(),
        };

        Self {
            r: (red * 255.0).round() as u8,
            g: (green * 255.0).round() as u8,
            b: (blue * 255.0).round() as u8,
        }
    }
}

impl Deref for Pixel {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.alpha
    }
}

impl From<f32> for Pixel {
    fn from(value: f32) -> Self {
        Self {
            color: Color::default(),
            alpha: value.clamp(0.0, 1.0),
        }
    }
}
impl From<Color> for Pixel {
    fn from(value: Color) -> Self {
        Self {
            color: value,
            alpha: 1.0,
        }
    }
}

impl Add for Pixel {
    type Output = Pixel;

    fn add(self, other: Pixel) -> Pixel {
        Pixel {
            color: Color {
                r: self.color.r.saturating_add(other.color.r),
                g: self.color.g.saturating_add(other.color.g),
                b: self.color.b.saturating_add(other.color.b),
            },
            alpha: (self.alpha + other.alpha).clamp(0.0, 1.0),
        }
    }
}

impl std::fmt::Display for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let characters: &[char] = &[' ', '░', '▒', '▓', '█'];
        // let cs: &[char] = &[' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];
        let character = match self.alpha {
            ..0.0 => characters[0],
            1.0.. => characters[characters.len() - 1],
            v => {
                characters[((v * (characters.len() as f32 - 1.0)).round() as usize)
                    .clamp(0, characters.len())]
            }
        };
        let red = self.color.r;
        let green = self.color.g;
        let blue = self.color.b;
        write!(f, "\u{1b}[38;2;{red};{green};{blue}m{character}\u{1b}[0m")
    }
}
