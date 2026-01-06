use crate::pixel::Color;
use crate::screen::Screen;
use crate::vecs::{Vec2, Vec3};
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
                    let first = CUBE[face[idx]];
                    let second = CUBE[*second_idx];

                    let first = translate_z(rotate_xz(first, it), 1.5);
                    let second = translate_z(rotate_xz(second, it), 1.5);

                    let first = project(first);
                    let second = project(second);

                    let col = Color::from_hsv((it * 10.0) % 360.0, 0.7, 0.8);

                    screen.draw_line(col, first, second);
                }
            }
        }
        screen.render();
        thread::sleep(Duration::from_millis(1000 / FPS));
        it += 0.05;
        screen.clear_context();
    }
}

fn rotate_xz(Vec3 { x, y, z }: Vec3<f32>, theta: f32) -> Vec3<f32> {
    let cos = theta.cos();
    let sin = theta.sin();
    (x * cos - z * sin, y, x * sin + z * cos).into()
}
fn project(Vec3 { x, y, z }: Vec3<f32>) -> Vec2<f32> {
    (x / z, y / z).into()
}

fn translate_z(Vec3 { x, y, z }: Vec3<f32>, dz: f32) -> Vec3<f32> {
    (x, y, z + dz).into()
}
