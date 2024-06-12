use bevy::prelude::*;
use rand::Rng;
use std::f32::consts::PI;

pub fn get_random_position_around(center: Vec2, radius: f32) -> Vec2 {
    let mut rng = rand::thread_rng();
    let angle = rng.gen_range(0.0..PI * 2.0);
    let x = center.x + radius * angle.cos();
    let y = center.y + radius * angle.sin();
    Vec2::new(x, y)
}
