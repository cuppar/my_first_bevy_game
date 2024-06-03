use bevy::math::Vec2;
use bevy::prelude::*;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, movement);
    }
}

#[derive(Component, Debug)]
pub struct Velocity {
    pub direction: Vec2,
    pub speed: f32,
}

impl Velocity {
    fn value(&self) -> Vec2 {
        self.direction.normalize_or_zero() * self.speed
    }
}

fn movement(time: Res<Time>, mut query: Query<(&Velocity, &mut Transform)>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.value().extend(0.) * time.delta_seconds();
    }
}
