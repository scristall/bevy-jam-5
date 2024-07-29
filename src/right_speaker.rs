use std::time::Duration;

use bevy::prelude::*;

#[derive(Component)]
struct Smoke {
    trajectory: Vec3,
    timer: Timer,
}

#[derive(Component)]
struct SmokeSpawner {
    timer: Timer,
}

fn smoke_setup(mut commands: Commands) {
    commands.spawn(SmokeSpawner {
        timer: Timer::new(Duration::from_millis(130), TimerMode::Repeating),
    });
}

fn spawn_smoke(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut smoke_spawner: Query<&mut SmokeSpawner>,
    time: Res<Time>,
) {
    for mut smoke_spawner in smoke_spawner.iter_mut() {
        smoke_spawner.timer.tick(time.delta());
        if smoke_spawner.timer.just_finished() {
            let x = (rand::random::<f32>() - 0.5) * 0.6;
            let y = rand::random::<f32>();

            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load("images/smoke.png"),
                    transform: Transform::from_xyz(166.0, 153.0, 3.0),
                    ..Default::default()
                },
                Smoke {
                    trajectory: Vec3::new(x, y, 0.0),
                    timer: Timer::new(Duration::from_millis(2000), TimerMode::Once),
                },
            ));
        }
    }
}

fn update_smoke(
    mut commands: Commands,
    mut smoke: Query<(&mut Transform, &mut Smoke, Entity)>,
    time: Res<Time>,
) {
    for mut smoke in smoke.iter_mut() {
        smoke.0.translation += smoke.1.trajectory;
        smoke.1.trajectory.y *= 1.005;

        smoke.1.timer.tick(time.delta());

        if smoke.1.timer.finished() {
            commands.entity(smoke.2).despawn();
        }
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, smoke_setup);
    app.add_systems(Update, spawn_smoke);
    app.add_systems(Update, update_smoke);
}
