use crate::pixel::Color;
use crate::screen::Screen;
use crate::vecs::{Vec2, Vec3};
use std::ops::Mul;
use std::thread;
use std::time::Duration;

const FPS: u64 = 60;

mod pixel;
mod screen;
mod vecs;

const CUBE: [Vec3<f32>; 8] = [
    Vec3::from(-0.5, -0.5, 0.5), // 0
    Vec3::from(0.5, -0.5, 0.5),  // 1
    Vec3::from(0.5, 0.5, 0.5),   // 2
    Vec3::from(-0.5, 0.5, 0.5),  // 3
    //
    Vec3::from(-0.5, -0.5, -0.5), // 4
    Vec3::from(0.5, -0.5, -0.5),  // 5
    Vec3::from(0.5, 0.5, -0.5),   // 6
    Vec3::from(-0.5, 0.5, -0.5),  // 7
];

const FACES: &[&[usize]] = &[
    &[0, 1, 2, 3, 0],
    //
    &[4, 5, 6, 7, 4],
    &[0, 4],
    &[1, 5],
    &[2, 6],
    &[3, 7],
];

fn main() {
    let mut screen = Screen::new(90, 26);
    let mut it: f32 = 0.0;
    loop {
        for face in FACES {
            for idx in 0..face.len() {
                let second_idx = face.get(idx + 1);
                if let Some(second_idx) = second_idx {
                    let vert_1 = CUBE[face[idx]];
                    let vert_2 = CUBE[*second_idx];

                    let vert_1 = vert_1.rotate_y(it).rotate_x(it / 2.0).translate_z(1.5);
                    let vert_2 = vert_2.rotate_y(it).rotate_x(it / 2.0).translate_z(1.5);

                    let vert_1 = project(vert_1);
                    let vert_2 = project(vert_2);

                    let col = Color::from_hsv((it * 10.0) % 360.0, 0.7, 0.8);

                    screen.draw_line(col, vert_1, vert_2);
                }
            }
        }
        screen.render();
        thread::sleep(Duration::from_millis(1000 / FPS));
        it += 0.05;
        screen.clear_context();
    }
}

pub trait Transformation {
    #[must_use]
    fn scale(self, scale: f32) -> Self;
    #[must_use]
    fn translate_z(self, translation: f32) -> Self;
    #[must_use]
    fn translate_x(self, translation: f32) -> Self;
    #[must_use]
    fn translate_y(self, translation: f32) -> Self;
    #[must_use]
    fn rotate_x(self, theta: f32) -> Self;
    #[must_use]
    fn rotate_y(self, theta: f32) -> Self;
    #[must_use]
    fn rotate_z(self, theta: f32) -> Self;
}

impl Transformation for Vec3<f32> {
    fn scale(self, scale: f32) -> Self {
        Self {
            x: self.x.mul(scale),
            y: self.y.mul(scale),
            z: self.z.mul(scale),
        }
    }

    fn translate_z(self, translation: f32) -> Self {
        (self.x, self.y, self.z + translation).into()
    }

    fn translate_x(self, translation: f32) -> Self {
        (self.x + translation, self.y, self.z).into()
    }

    fn translate_y(self, translation: f32) -> Self {
        (self.x, self.y + translation, self.z).into()
    }

    fn rotate_y(self, theta: f32) -> Self {
        let cos = theta.cos();
        let sin = theta.sin();
        (
            self.x * cos - self.z * sin,
            self.y,
            self.x * sin + self.z * cos,
        )
            .into()
    }

    fn rotate_x(self, theta: f32) -> Self {
        let cos = theta.cos();
        let sin = theta.sin();
        (
            self.x,
            self.y.mul(cos) - self.z.mul(sin),
            self.y.mul(sin) + self.z.mul(cos),
        )
            .into()
    }

    fn rotate_z(self, theta: f32) -> Self {
        let cos = theta.cos();
        let sin = theta.sin();
        (
            self.x.mul(cos) - self.y.mul(sin),
            self.x.mul(sin) + self.y.mul(cos),
            self.z,
        )
            .into()
    }
}

fn project(Vec3 { x, y, z }: Vec3<f32>) -> Vec2<f32> {
    (x / z, y / z).into()
}
