use bevy::{
    math::const_vec3,
    prelude::{self, *},
};
use bevy_rapier3d::prelude::*;

pub struct Plugin;

impl prelude::Plugin for Plugin {
    fn build(&self, app: &mut prelude::App) {
        app.add_startup_system(build);
    }
}

#[derive(Component)]
pub struct Ball;

#[derive(Component)]
pub struct Backboard;

#[derive(Component, PartialEq, Eq, Clone, Debug)]
pub enum Flipper {
    Left,
    Right,
}

#[derive(Component)]
pub enum FlipperMotor {
    Left,
    Right,
}

const LEFT_FLIPPER_POS: bevy::prelude::Vec3 = const_vec3!([40.0, 2.0, 20.0]);
const RIGHT_FLIPPER_POS: bevy::prelude::Vec3 = const_vec3!([40.0, 2.0, -20.0]);

fn build(mut commands: Commands, mut config: ResMut<RapierConfiguration>) {
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
            ..ColliderBundle::default()
        })
        .insert_bundle(RigidBodyBundle {
            position: Vec3::new(0.0, 10.0, 15.0).into(),
            ..RigidBodyBundle::default()
        })
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
            ..ColliderBundle::default()
        })
        .insert_bundle(RigidBodyBundle {
            position: RIGHT_FLIPPER_POS.into(),
            ..RigidBodyBundle::default()
        })
        .insert_bundle((
            ColliderPositionSync::Discrete,
            ColliderDebugRender::from(Color::RED),
            Flipper::Right,
        ))
        .id();

    commands.spawn_bundle((
        JointBuilderComponent::new(
            RevoluteJoint::new(Vector::y_axis())
                .local_anchor1(RIGHT_FLIPPER_POS.into())
                .local_anchor2(point![0.0, 0.0, -5.0])
                .limit_axis([-1.0, 1.0]),
            backboard,
            right_flipper,
        ),
        FlipperMotor::Right,
    ));

    let left_flipper = commands
        .spawn_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(1.0, 1.0, 5.0).into(),
            ..ColliderBundle::default()
        })
        .insert_bundle(RigidBodyBundle {
            position: LEFT_FLIPPER_POS.into(),
            ..RigidBodyBundle::default()
        })
        .insert_bundle((
            ColliderPositionSync::Discrete,
            ColliderDebugRender::from(Color::RED),
            Flipper::Left,
        ))
        .id();

    commands.spawn_bundle((
        JointBuilderComponent::new(
            RevoluteJoint::new(Vector::y_axis())
                .local_anchor1(LEFT_FLIPPER_POS.into())
                .local_anchor2(point![0.0, 0.0, 5.0])
                .limit_axis([-1.0, 1.0]),
            backboard,
            left_flipper,
        ),
        FlipperMotor::Left,
    ));
}
