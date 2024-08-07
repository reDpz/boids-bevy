use bevy::prelude::*;

pub fn dist_vec3(p1: &Vec3, p2: &Vec3) -> f32 {
    // sqrt(dx^2 + dy^2 + dz^2)
    ((p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2) + (p1.z - p2.z).powi(2)).sqrt()
}

pub struct Rect {
    pub position: Vec2,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    pub fn is_point_in(&self, point: &Vec2) -> bool {
        // should i cache some of this
        let half_w = self.width / 2.0;
        let half_h = self.height / 2.0;
        (point.x < self.position.x + half_w && point.x > self.position.x - half_w)
            && (point.y < self.position.y + half_h && point.y > self.position.y - half_h)
    }

    pub fn line_intersect(&self, line_fn: fn(f32) -> f32) -> Option<Vec2> {
        todo!()
    }
}

#[inline]
pub fn point_to_vec2(from: Vec2, to: Vec2) -> Vec2 {
    to - from
}
