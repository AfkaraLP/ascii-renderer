use std::ops::Add;
use std::ops::Deref;

#[derive(Clone, Debug, Default, PartialEq, Copy)]
pub struct Pixel {
    pub color: Color,
    pub alpha: f32,
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn from_hsv(h: impl Into<f32>, s: f32, v: f32) -> Self {
        let h = h.into().clamp(0.0, 360.0) / 60.0;
        let s = s.clamp(0.0, 1.0);
        let v = v.clamp(0.0, 1.0);

        let i = h.floor() as u32;
        let f = h - h.floor();
        let p = v * (1.0 - s);
        let q = v * (1.0 - f * s);
        let t = v * (1.0 - (1.0 - f) * s);

        let (r, g, b) = match i % 6 {
            0 => (v, t, p),
            1 => (q, v, p),
            2 => (p, v, t),
            3 => (p, q, v),
            4 => (t, p, v),
            5 => (v, p, q),
            _ => unreachable!(),
        };

        Self {
            r: (r * 255.0).round() as u8,
            g: (g * 255.0).round() as u8,
            b: (b * 255.0).round() as u8,
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Self { r: 0, g: 0, b: 0 }
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cs: &[char] = &[' ', '░', '▒', '▓', '█'];
        // let cs: &[char] = &[' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];
        let c = match self.alpha {
            ..0.0 => cs[0],
            1.0.. => cs[cs.len() - 1],
            v => cs[((v * (cs.len() as f32 - 1.0)).round() as usize).clamp(0, cs.len())],
        };
        let r = self.color.r;
        let g = self.color.g;
        let b = self.color.b;
        write!(f, "\u{1b}[38;2;{};{};{}m{}\u{1b}[0m", r, g, b, c)
    }
}
