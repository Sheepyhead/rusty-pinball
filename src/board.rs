use bevy::{
    math::{const_vec3, vec3},
    prelude::{self, *},
};
use bevy_rapier3d::prelude::*;

pub struct Plugin;

impl prelude::Plugin for Plugin {
    fn build(&self, app: &mut prelude::App) {
        app.add_startup_system(build).add_system(respawn_ball);
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

#[derive(Component)]
pub struct DeathZone;

const LEFT_FLIPPER_POS: bevy::prelude::Vec3 = const_vec3!([40.0, 3.0, 15.0]);
const RIGHT_FLIPPER_POS: bevy::prelude::Vec3 = const_vec3!([40.0, 3.0, -15.0]);
enum CollisionGroups {
    Balls = 0b1,
}

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

    spawn_ball(&mut commands);

    let backboard = commands
        .spawn_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(50.0, 1.0, 25.0).into(),
            material: ColliderMaterial {
                friction: 0.01,
                ..ColliderMaterial::default()
            }
            .into(),
            flags: ColliderFlags {
                collision_groups: InteractionGroups::new(
                    u32::MAX ^ (CollisionGroups::Balls as u32),
                    CollisionGroups::Balls as u32,
                ),
                ..ColliderFlags::default()
            }
            .into(),
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

    commands
        .spawn_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(60.0, 1.0, 30.0).into(),
            material: ColliderMaterial {
                friction: 0.01,
                ..ColliderMaterial::default()
            }
            .into(),
            ..ColliderBundle::default()
        })
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyTypeComponent(RigidBodyType::Static),
            position: Vec3::new(0.0, 25.0, 0.0).into(),
            ..RigidBodyBundle::default()
        })
        .insert_bundle((ColliderPositionSync::Discrete,));

    commands
        .spawn_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(50.0, 10.0, 1.0).into(),
            material: ColliderMaterial {
                friction: 0.01,
                ..ColliderMaterial::default()
            }
            .into(),
            ..ColliderBundle::default()
        })
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyTypeComponent(RigidBodyType::Static),
            position: vec3(0.0, 0.0, -25.0).into(),
            ..RigidBodyBundle::default()
        })
        .insert_bundle((
            ColliderPositionSync::Discrete,
            ColliderDebugRender::from(Color::ORANGE),
        ));

    commands
        .spawn_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(50.0, 10.0, 1.0).into(),
            material: ColliderMaterial {
                friction: 0.01,
                ..ColliderMaterial::default()
            }
            .into(),
            ..ColliderBundle::default()
        })
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyTypeComponent(RigidBodyType::Static),
            position: vec3(0.0, 0.0, 25.0).into(),
            ..RigidBodyBundle::default()
        })
        .insert_bundle((
            ColliderPositionSync::Discrete,
            ColliderDebugRender::from(Color::ORANGE),
        ));

    commands
        .spawn_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(1.0, 10.0, 26.0).into(),
            material: ColliderMaterial {
                friction: 0.01,
                ..ColliderMaterial::default()
            }
            .into(),
            ..ColliderBundle::default()
        })
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyTypeComponent(RigidBodyType::Static),
            position: vec3(-50.0, 0.0, 0.0).into(),
            ..RigidBodyBundle::default()
        })
        .insert_bundle((
            ColliderPositionSync::Discrete,
            ColliderDebugRender::from(Color::ORANGE),
        ));

    commands
        .spawn_bundle(ColliderBundle {
            collider_type: ColliderType::Sensor.into(),
            flags: ActiveEvents::empty().into(),
            shape: ColliderShape::cuboid(1.0, 10.0, 25.0).into(),
            ..ColliderBundle::default()
        })
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyTypeComponent(RigidBodyType::Static),
            position: vec3(60.0, 0.0, 0.0).into(),
            ..RigidBodyBundle::default()
        })
        .insert_bundle((ColliderPositionSync::Discrete, DeathZone));

    let right_flipper = commands
        .spawn_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(1.0, 2.0, 5.0).into(),
            flags: ColliderFlags {
                collision_groups: InteractionGroups::new(
                    u32::MAX ^ (CollisionGroups::Balls as u32),
                    CollisionGroups::Balls as u32,
                ),
                ..ColliderFlags::default()
            }
            .into(),
            ..ColliderBundle::default()
        })
        .insert_bundle(RigidBodyBundle {
            position: RIGHT_FLIPPER_POS.into(),
            ..RigidBodyBundle::default()
        })
        .insert_bundle((
            RigidBodyPositionSync::Discrete,
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
            shape: ColliderShape::cuboid(1.0, 2.0, 5.0).into(),
            flags: ColliderFlags {
                collision_groups: InteractionGroups::new(
                    u32::MAX ^ (CollisionGroups::Balls as u32),
                    CollisionGroups::Balls as u32,
                ),
                ..ColliderFlags::default()
            }
            .into(),
            ..ColliderBundle::default()
        })
        .insert_bundle(RigidBodyBundle {
            position: LEFT_FLIPPER_POS.into(),
            ..RigidBodyBundle::default()
        })
        .insert_bundle((
            RigidBodyPositionSync::Discrete,
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

fn spawn_ball(commands: &mut Commands) {
    commands
        .spawn_bundle(ColliderBundle {
            flags: ColliderFlags {
                collision_groups: InteractionGroups::new(CollisionGroups::Balls as u32, u32::MAX),
                active_events: ActiveEvents::INTERSECTION_EVENTS,
                ..ColliderFlags::default()
            }
            .into(),
            shape: ColliderShape::ball(1.).into(),
            material: ColliderMaterial {
                friction: 0.01,
                ..ColliderMaterial::default()
            }
            .into(),
            ..ColliderBundle::default()
        })
        .insert_bundle(RigidBodyBundle {
            position: Vec3::new(0.0, 5.0, 10.0).into(),
            ccd: RigidBodyCcd {
                ccd_enabled: true,
                ..RigidBodyCcd::default()
            }
            .into(),
            ..RigidBodyBundle::default()
        })
        .insert_bundle((
            RigidBodyPositionSync::Discrete,
            ColliderDebugRender::from(Color::GREEN),
            Ball,
        ));
}

fn respawn_ball(
    mut commands: Commands,
    mut intersection_events: EventReader<IntersectionEvent>,
    balls: Query<(), With<Ball>>,
    death_zones: Query<(), With<DeathZone>>,
) {
    intersection_events
        .iter()
        .filter_map(|event| {
            let e1 = event.collider1.entity();
            let e2 = event.collider2.entity();
            let ball_dz = (balls.get(e1).is_ok() && death_zones.get(e2).is_ok()).then(|| e1);
            let dz_ball = (balls.get(e2).is_ok() && death_zones.get(e1).is_ok()).then(|| e2);
            ball_dz.or(dz_ball)
        })
        .for_each(|ball| {
            commands.entity(ball).despawn_recursive();
            spawn_ball(&mut commands);
        });
}
