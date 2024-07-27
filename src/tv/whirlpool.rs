use bevy::prelude::*;

#[derive(Component)]
pub struct Whirlpool {
    pub speed: f32
}

fn update(mut whirlpools: Query<(&mut Transform, &Whirlpool)>) {
    for mut whirlpool in &mut whirlpools.iter_mut() {
        whirlpool.0.rotate(Quat::from_rotation_z(whirlpool.1.speed));
    }
}

pub fn whirlpool_plugin(app: &mut App) {
    app.add_systems(Update, update);
}
