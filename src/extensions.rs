use macroquad::prelude::*;

#[derive(serde::Deserialize)]
pub struct Square<T> {
    pub width: T,
    pub height: T,
}

pub trait RectExtensions {
    fn move_center(&mut self, destination: Vec2);
}

impl RectExtensions for Rect {
    fn move_center(&mut self, destination: Vec2) {
        let point_destination = vec2(destination.x - self.w / 2., destination.y - self.h / 2.);
        self.move_to(point_destination);
    }
}
