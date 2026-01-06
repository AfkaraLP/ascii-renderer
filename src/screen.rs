#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_precision_loss)]
use std::io::Write;

use crate::pixel::Pixel;
use crate::vecs::Vec2;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Screen {
    pub width: usize,
    pub height: usize,
    pixels: Vec<Pixel>,
}

impl Screen {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![Pixel::default(); width * height],
        }
    }

    #[inline]
    pub fn clear_context(&mut self) {
        self.pixels = vec![Pixel::default(); self.width * self.height];
    }

    #[inline]
    fn add_pixel(&mut self, xy: Vec2<usize>, pixel: Pixel) {
        let idx = self.get_arr_idx(xy);
        if let Some(px) = self.pixels.get_mut(idx) {
            let curr_pix: Pixel = *px;
            let new_pix = curr_pix + pixel;
            *px = new_pix;
        }
    }

    #[inline]
    fn get_pixel(&self, xy: Vec2<usize>) -> &Pixel {
        let idx = self.get_arr_idx(xy);
        &self.pixels[idx]
    }

    pub fn render(&self) {
        print!("\x1B[2J\x1B[H");
        std::io::stdout().flush().unwrap();
        for y in 0..self.height {
            for x in 0..self.width {
                if y == 0 || y == self.height - 1 || x == 0 || x == self.width - 1 {
                    print!("█");
                } else {
                    let px = self.get_pixel((x, y).into());
                    print!("{px}");
                }
            }
            println!();
        }
    }

    fn project(&self, Vec2 { x, y }: Vec2<f32>) -> Vec2<usize> {
        let x_norm = x.midpoint(1.0);
        let y_norm = 1.0 - y.midpoint(1.0);

        let width = self.width as f32;
        let height = self.height as f32;

        let x = (x_norm * width) as usize;
        let y = (y_norm * height) as usize;

        (x, y).into()
    }

    #[inline]
    fn get_arr_idx(&self, Vec2 { x, y }: Vec2<usize>) -> usize {
        y * self.width + x
    }

    #[allow(unused)]
    pub fn draw_dot(&mut self, pixel: Pixel, pos: Vec2<f32>) {
        let xy = self.project(pos);
        self.add_pixel(xy, pixel);
    }

    #[allow(unused)]
    pub fn draw_line(&mut self, pixel: impl Into<Pixel>, start: Vec2<f32>, end: Vec2<f32>) {
        let start = self.project(start);
        let end = self.project(end);
        let pixel: Pixel = pixel.into();

        for x in start.x.min(end.x)..=start.x.max(end.x) {
            for y in start.y.min(end.y)..=start.y.max(end.y) {
                let px = x as f32 + 0.5;
                let py = y as f32 + 0.5;

                let x0 = start.x as f32;
                let y0 = start.y as f32;
                let x1 = end.x as f32;
                let y1 = end.y as f32;

                let dx = x1 - x0;
                let dy = y1 - y0;

                let len_sq = dx.powf(2.0) + dy.powf(2.0);

                let t = if len_sq == 0.0 {
                    0.0
                } else {
                    ((px - x0) * dx + (py - y0) * dy) / len_sq
                }
                .clamp(0.0, 1.0);

                let cx = x0 + t * dx;
                let cy = y0 + t * dy;

                let dist = (px - cx).powf(2.0) + (py - cy).powf(2.0);

                if dist <= 0.5 {
                    let alpha = (1.0 - (dist * 2.0)).clamp(0.0, *pixel);
                    let pix = Pixel {
                        color: pixel.color,
                        alpha,
                    };

                    self.add_pixel((x, y).into(), pix);
                }
            }
        }
    }
}
