use bevy::prelude::{self, *};
use bevy_input_actionmap::{ActionPlugin, InputMap};
use bevy_rapier3d::prelude::*;

use crate::board::Flipper;

pub struct Plugin;

impl prelude::Plugin for Plugin {
    fn build(&self, app: &mut prelude::App) {
        app.add_plugin(ActionPlugin::<Action>::default())
            .add_startup_system(setup)
            .add_system(flipper_action);
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
enum Action {
    RightFlippers,
    LeftFlippers,
}

fn setup(mut input: ResMut<InputMap<Action>>) {
    input
        .bind(Action::RightFlippers, KeyCode::T)
        .bind(Action::LeftFlippers, KeyCode::E);
}

fn flipper_action(
    input: Res<InputMap<Action>>,
    mut flippers: Query<(
        &mut RigidBodyVelocityComponent,
        &RigidBodyMassPropsComponent,
        &Flipper,
    )>,
) {
    if input.active(Action::LeftFlippers) {
        flippers
            .iter_mut()
            .filter(|(_, _, flipper)| **flipper == Flipper::Left)
            .for_each(|(mut vel, mass_props, _)| {
                vel.apply_torque_impulse(mass_props, Vec3::new(0.0, 10_000.0, 0.0).into());
            });
    } else {
        flippers
            .iter_mut()
            .filter(|(_, _, flipper)| **flipper == Flipper::Left)
            .for_each(|(mut vel, mass_props, _)| {
                vel.apply_torque_impulse(mass_props, Vec3::new(0.0, -10_000.0, 0.0).into());
            });
    }

    if input.active(Action::RightFlippers) {
        flippers
            .iter_mut()
            .filter(|(_, _, flipper)| **flipper == Flipper::Right)
            .for_each(|(mut vel, mass_props, _)| {
                vel.apply_torque_impulse(mass_props, Vec3::new(0.0, -10_000.0, 0.0).into());
            });
    } else {
        flippers
            .iter_mut()
            .filter(|(_, _, flipper)| **flipper == Flipper::Right)
            .for_each(|(mut vel, mass_props, _)| {
                vel.apply_torque_impulse(mass_props, Vec3::new(0.0, 10_000.0, 0.0).into());
            });
    }
}
