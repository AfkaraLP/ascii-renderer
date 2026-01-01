#[derive(Clone, Copy, Debug, Default)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec3<T> {
    pub const fn from(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}
impl<T> Vec2<T> {
    #[allow(dead_code)]
    pub const fn from(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> From<(T, T, T)> for Vec3<T> {
    fn from(value: (T, T, T)) -> Self {
        Self {
            x: value.0,
            y: value.1,
            z: value.2,
        }
    }
}
impl<T> From<(T, T)> for Vec2<T> {
    fn from(value: (T, T)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}
