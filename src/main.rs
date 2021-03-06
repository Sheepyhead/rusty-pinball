use bevy::{prelude::*, window::WindowMode};
use bevy_rapier3d::prelude::*;

mod board;
mod camera;
mod input;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            mode: WindowMode::BorderlessFullscreen,
            ..WindowDescriptor::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierRenderPlugin)
        .add_plugin(board::Plugin)
        .add_plugin(camera::Plugin)
        .add_plugin(input::Plugin)
        .run();
}
