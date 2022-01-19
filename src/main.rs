use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierRenderPlugin)
        .add_startup_system(build_board)
        .add_startup_system(spawn_cameras)
        .run();
}

#[derive(Component)]
pub struct Ball;

#[derive(Component)]
pub struct Backboard;

fn build_board(mut commands: Commands) {
    commands
        .spawn_bundle(ColliderBundle {
            shape: ColliderShape::ball(0.5).into(),
            position: Vec3::new(0.0, 50.0, 0.0).into(),
            ..ColliderBundle::default()
        })
        .insert_bundle(RigidBodyBundle {
            ..RigidBodyBundle::default()
        })
        .insert_bundle((
            ColliderPositionSync::Discrete,
            ColliderDebugRender::with_id(2),
            Ball,
            Transform::default(),
            GlobalTransform::default(),
        ));

    commands
        .spawn_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(50.0, 1.0, 25.0).into(),
            ..ColliderBundle::default()
        })
        .insert_bundle((
            ColliderPositionSync::Discrete,
            ColliderDebugRender::with_id(0),
            Backboard,
            Transform::default(),
            GlobalTransform::default(),
        ));

    commands
        .spawn_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(1.0, 1.0, 25.0).into(),
            position: Vec3::new(40.0, 2.0, 0.0).into(),
            ..ColliderBundle::default()
        })
        .insert_bundle((
            ColliderPositionSync::Discrete,
            ColliderDebugRender::with_id(1),
            Transform::default(),
            GlobalTransform::default(),
        ));
}

fn spawn_cameras(mut commands: Commands) {
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 100.0, 200.0))
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        ..PerspectiveCameraBundle::default()
    });
    commands.spawn_bundle(UiCameraBundle::default());
}
