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

fn build_board(mut commands: Commands, mut config: ResMut<RapierConfiguration>) {
    config.gravity = vector!(-0.5, 1.0, 0.0).normalize() * -9.81 * 25.0;

    commands
        .spawn_bundle(ColliderBundle {
            shape: ColliderShape::ball(0.5).into(),
            position: Vec3::new(0.0, 10.0, 0.0).into(),
            ..ColliderBundle::default()
        })
        .insert_bundle(RigidBodyBundle {
            ..RigidBodyBundle::default()
        })
        .insert_bundle((
            ColliderPositionSync::Discrete,
            ColliderDebugRender::from(Color::GREEN),
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
            ColliderDebugRender::from(Color::ORANGE),
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
            ColliderDebugRender::from(Color::ORANGE),
            Transform::default(),
            GlobalTransform::default(),
        ));

    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::ANTIQUE_WHITE,
            illuminance: 32_000.0,
            ..DirectionalLight::default()
        },
        ..DirectionalLightBundle::default()
    });
}

fn spawn_cameras(mut commands: Commands) {
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_translation(Vec3::new(100.0, 100.0, 0.0))
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        ..PerspectiveCameraBundle::default()
    });
    commands.spawn_bundle(UiCameraBundle::default());
}
