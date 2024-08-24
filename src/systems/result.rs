use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use crate::{components::{result::InResult, Bg}, events::result::ResultEvent, resources::assets::FlappyBirdAssets, states::{Game, States}};

pub fn on_result(
    mut commands: Commands,
    mut reader: EventReader<ResultEvent>,
    q_bg: Query<Entity, With<Bg>>,
    fb_assets: Res<FlappyBirdAssets>,
    mut next_state: ResMut<NextState<States>>
) {
    for _ in reader.read() {
        info!("result");
        if let Ok(bg) = q_bg.get_single() {
            if let Some(mut ec) = commands.get_entity(bg) {
                next_state.set(States::Game(Game::Result));
                ec.with_children(|parent| {
                    
                    

                    parent.spawn(
                        (
                            Name::new("result parent"),
                            InResult,
                            SpatialBundle::default()
                        )
                    )
                    .with_children(|parent| {
                        parent.spawn(
                            (
                                Name::new("panel"),
                                SpriteBundle {
                                    texture: fb_assets.panel_score.clone(),
                                    transform: Transform {
                                        translation: Vec3::new(0., 0., 10.),
                                        ..default()
                                    },
                                    ..default()
                                }
                            )
                        );
    
                        parent.spawn(
                            (
                                Name::new("ok"),
                                SpriteBundle {
                                    texture: fb_assets.button_ok.clone(),
                                    transform: Transform {
                                        translation: Vec3::new(0., -60., 10.),
                                        ..default()
                                    },
                                    ..default()
                                },
                                On::<Pointer<Click>>::run(|mut next_state: ResMut<NextState<States>>| {
                                    next_state.set(States::MainMenu);
                                }),
                            )
                        );
                    });
                });
            }
        }
    }
}