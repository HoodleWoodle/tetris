use serde::Deserialize;

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct Point2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point2<T> {
    pub fn new(x: T, y: T) -> Point2<T> {
        Point2 {
            x,
            y,
        }
    }
}