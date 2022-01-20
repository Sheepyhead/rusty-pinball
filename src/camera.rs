use bevy::prelude::{self, *};

pub struct Plugin;

impl prelude::Plugin for Plugin {
    fn build(&self, app: &mut prelude::App) {
        app.add_startup_system(spawn_cameras);
    }
}

fn spawn_cameras(mut commands: Commands) {
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_translation(Vec3::new(100.0, 100.0, 0.0))
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        ..PerspectiveCameraBundle::default()
    });
    commands.spawn_bundle(UiCameraBundle::default());
}
