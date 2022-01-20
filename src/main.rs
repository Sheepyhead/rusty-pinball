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

#[derive(Component)]
pub enum Flipper {
    Left,
    Right,
}

#[derive(Component)]
pub enum FlipperMotor {
    Left,
    Right,
}

fn build_board(mut commands: Commands, mut config: ResMut<RapierConfiguration>) {
    config.gravity = vector!(-0.5, 1.0, 0.0).normalize() * -9.81 * 25.0;

    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::ANTIQUE_WHITE,
            illuminance: 32_000.0,
            ..DirectionalLight::default()
        },
        transform: Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -45.0, 0.0, 0.0)),
        ..DirectionalLightBundle::default()
    });

    commands
        .spawn_bundle(ColliderBundle {
            shape: ColliderShape::ball(1.).into(),
            position: Vec3::new(0.0, 10.0, 15.0).into(),
            ..ColliderBundle::default()
        })
        .insert_bundle(RigidBodyBundle::default())
        .insert_bundle((
            ColliderPositionSync::Discrete,
            ColliderDebugRender::from(Color::GREEN),
            Ball,
        ));

    let backboard = commands
        .spawn_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(50.0, 1.0, 25.0).into(),
            ..ColliderBundle::default()
        })
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyTypeComponent(RigidBodyType::Static),
            ..RigidBodyBundle::default()
        })
        .insert_bundle((
            ColliderPositionSync::Discrete,
            ColliderDebugRender::from(Color::ORANGE),
            Backboard,
        ))
        .id();

    let right_flipper = commands
        .spawn_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(1.0, 1.0, 5.0).into(),
            position: Vec3::new(40.0, 2.0, -20.0).into(),
            ..ColliderBundle::default()
        })
        .insert_bundle(RigidBodyBundle::default())
        .insert_bundle((
            ColliderPositionSync::Discrete,
            ColliderDebugRender::from(Color::RED),
            Flipper::Right,
        ))
        .id();

    commands.spawn_bundle((
        JointBuilderComponent::new(
            RevoluteJoint::new(Vector::y_axis())
                .local_anchor1(point![40.0, 2.0, -20.0])
                .local_anchor2(point![40.0, 2.0, -25.0])
                .limit_axis([-2.0, 1.0]),
            backboard,
            right_flipper,
        ),
        FlipperMotor::Right,
    ));

    let left_flipper = commands
        .spawn_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(1.0, 1.0, 5.0).into(),
            position: Vec3::new(40.0, 2.0, 20.0).into(),
            ..ColliderBundle::default()
        })
        .insert_bundle(RigidBodyBundle::default())
        .insert_bundle((
            ColliderPositionSync::Discrete,
            ColliderDebugRender::from(Color::RED),
            Flipper::Left,
        ))
        .id();

    commands.spawn_bundle((
        JointBuilderComponent::new(
            RevoluteJoint::new(Vector::y_axis())
                .local_anchor1(point![40.0, 2.0, 20.0])
                .local_anchor2(point![40.0, 2.0, 25.0])
                .limit_axis([-1.0, 3.0]),
            backboard,
            left_flipper,
        ),
        FlipperMotor::Left,
    ));
}

fn spawn_cameras(mut commands: Commands) {
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_translation(Vec3::new(100.0, 100.0, 0.0))
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        ..PerspectiveCameraBundle::default()
    });
    commands.spawn_bundle(UiCameraBundle::default());
}
