use bevy::prelude::*;
use avian2d::prelude::*;
use crate::components::pipe::Pipe;
use crate::states::{Game, States};
use crate::{components::{bird::Bird, guide::Guide, Bg}, constant::PAUSE_BTN_DEPTH, events::jump::JumpEvent, resources::resize_scale::ResizeScale};

pub fn touch(
    mut commands: Commands,
    mut read: EventReader<JumpEvent>,
    q_guide: Query<Entity, With<Guide>>,
    q_bird: Query<Entity, With<Bird>>,
    q_pipe: Query<Entity, With<Pipe>>,
    resize_scale: Res<ResizeScale>,
    mut next_state: ResMut<NextState<States>>,
) {
    for JumpEvent(_, a) in read.read() {
        if *a == PAUSE_BTN_DEPTH {
            return;
        }
        info!("jump!");

        // 만약 가이드가 존재하면 가이드를 꺼준다.
        if let Ok(entity) = q_guide.get_single() {
            if let Some(ec) = commands.get_entity(entity) {
                ec.despawn_recursive();
            }

            if let Ok(entity) = q_bird.get_single() {
                if let Some(mut ec) = commands.get_entity(entity) {
                    ec.insert(RigidBody::Dynamic);
                }
            }
            next_state.set(States::Game(Game::Game));
            // for entity in &q_pipe {
            //     commands.entity(entity).insert(LinearVelocity(Vec2::new(-100., 0.)));
            // }
        }

        // impulse
        if let Ok(entity) = q_bird.get_single() {
            if let Some(mut ec) = commands.get_entity(entity) {
                
                let mut impulse = ExternalImpulse::default();
                let scale = resize_scale.0;
                let y = 1000.* scale;
                info!("{y}");
                impulse.apply_impulse(Vec2::new(0., y));// * tr.scale.x
                ec.insert(impulse);
                ec.insert(LinearVelocity::ZERO);
            }
        }
    }
}