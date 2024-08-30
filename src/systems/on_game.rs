use crate::constant::*;
use crate::events::picking_callback::PausePickingEvent;
use crate::events::prelude::*;
use crate::resources::prelude::*;
use crate::states::prelude::*;
use crate::{components::prelude::*, events::picking_callback::JumpPickingEvent};
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_tweening::{Animator, AnimatorState};
use rand::Rng;

pub fn trsition_result_on_main(
    mut commands: Commands,
    q_in_game: Query<Entity, With<InGame>>,
    q_in_result: Query<Entity, With<InResult>>,
    mut config: ResMut<GameConfig>,
) {
    config.score = 0;
    config.is_ad_show = false;
    for e in &q_in_game {
        if let Some(ec) = commands.get_entity(e) {
            ec.despawn_recursive();
        }
    }
}

pub fn trsition_result_to_game(
    mut commands: Commands,
    q_in_result: Query<Entity, With<InResult>>,
    q_pause: Query<Entity, With<PauseBtn>>,
) {
    for e in &q_in_result {
        if let Some(ec) = commands.get_entity(e) {
            ec.despawn_recursive();
        }
    }
    let pause_entity = q_pause.single();
    commands.entity(pause_entity).insert(Visibility::Inherited);
}

pub fn game_enter(
    mut commands: Commands,
    fb_assets: Res<FlappyBirdAssets>,
    mut ew_resize: EventWriter<ResizeEvent>,
    mut next_state: ResMut<NextState<MyStates>>,
) {
    info!("guide_enter");
    let bg = (
        Name::new("bg"),
        InGame,
        Bg,
        Resizable,
        SpriteBundle {
            texture: fb_assets.background_day.clone(),
            ..default()
        },
        On::<Pointer<Down>>::send_event::<JumpPickingEvent>(),
    );

    let bird = (
        BirdBundle::default(),
        RigidBody::Static,
        LockedAxes::ROTATION_LOCKED,
        Collider::circle(17. / 2.),
        ColliderDensity(0.0),
        Mass(5.0),
        SpriteBundle {
            texture: fb_assets.gen_bird_atlas_texture.clone(),
            transform: Transform {
                translation: Vec3::new(-30., 30., Z_INDEX_2),
                ..default()
            },
            ..default()
        },
        TextureAtlas {
            index: 0,
            layout: fb_assets.gen_bird_atlas_layout.clone(),
        },
    );

    let pause_btn = (
        Name::new("pause"),
        PauseBtn,
        SpatialBundle::from_transform(Transform {
            translation: Vec3::new(-55., 110., PAUSE_BTN_DEPTH),
            ..default()
        }),
        On::<Pointer<Click>>::send_event::<PausePickingEvent>(),
    );

    let pause_btn_sprite = (
        Name::new("pause sprite"),
        SpriteBundle {
            texture: fb_assets.button_pause.clone(),
            ..default()
        },
    );

    let get_ready = (
        Name::new("get_ready"),
        SpriteBundle {
            texture: fb_assets.label_get_ready.clone(),
            transform: Transform::from_xyz(0., 60., Z_INDEX_1),
            ..default()
        },
    );

    let score = (
        Name::new("score"),
        ScoreParent,
        SpatialBundle::from_transform(Transform::from_xyz(0., 110., Z_INDEX_2)),
    );

    let num_0 = (
        Name::new("num"),
        SpriteBundle {
            texture: fb_assets.number_large_0.clone(),
            transform: Transform {
                translation: Vec3::new(0., 0., 0.),
                ..default()
            },
            ..default()
        },
    );

    let ground = (
        Name::new("ground"),
        Ground,
        SpriteBundle {
            texture: fb_assets.ground.clone(),
            transform: Transform {
                translation: Vec3::new(12., -100., Z_INDEX_2),
                ..default()
            },
            ..default()
        },
    );
    let ground_collider = (
        Name::new("groundCollider"),
        GroundCollider,
        RigidBody::Static,
        Collider::rectangle(168., 56.),
        TransformBundle::from_transform(Transform {
            translation: Vec3::new(0., -100., Z_INDEX_1),
            ..default()
        }),
    );

    let sky_collider = (
        Name::new("skyCollider"),
        // GroundCollider,
        RigidBody::Static,
        Collider::rectangle(168., 56.),
        TransformBundle::from_transform(Transform {
            translation: Vec3::new(0., 200., Z_INDEX_1),
            ..default()
        }),
    );

    let guide = (
        Name::new("guide"),
        SpriteBundle {
            texture: fb_assets.instructions.clone(),
            transform: Transform {
                translation: Vec3::new(0., 0., Z_INDEX_1),
                ..default()
            },
            ..default()
        },
    );

    let guide_parent = (Name::new("guide"), Guide, SpatialBundle { ..default() });

    let pipe_bottom_sprite = fb_assets.pipe_green_bottom.clone();
    let pipe_top_sprite = fb_assets.pipe_green_top.clone();

    let pipe_score_collider = (
        Name::new("pipe_collider"),
        Sensor,
        PipePoint,
        Collider::rectangle(5., 300.),
        SpatialBundle::from_transform(Transform { ..default() }),
    );

    let pipe_bottom = (
        Name::new("pipe_bottom"),
        Sensor,
        Pipe,
        Collider::rectangle(26., 160.),
        SpriteBundle {
            texture: pipe_bottom_sprite,
            transform: Transform {
                translation: Vec3::new(0., -110., 0.),
                ..default()
            },
            ..default()
        },
    );

    let pipe_top = (
        Name::new("pipe_top"),
        Sensor,
        Pipe,
        Collider::rectangle(26., 160.),
        SpriteBundle {
            texture: pipe_top_sprite,
            transform: Transform {
                translation: Vec3::new(0., 110., 0.),
                ..default()
            },
            ..default()
        },
    );

    let r = rand::thread_rng().gen_range((-30.0)..(100.0));
    let r2 = rand::thread_rng().gen_range((-30.0)..(100.0));
    let pipe_parent2 = (
        Name::new("pipe_parent2"),
        RigidBody::Static,
        PipeParent,
        SpatialBundle {
            transform: Transform {
                translation: Vec3::new(85., r, Z_INDEX_1),
                ..default()
            },
            ..default()
        },
    );
    let pipe_parent3 = (
        Name::new("pipe_parent3"),
        RigidBody::Static,
        PipeParent,
        SpatialBundle {
            transform: Transform {
                translation: Vec3::new(170., r2, Z_INDEX_1),
                ..default()
            },
            ..default()
        },
    );

    commands.spawn(bg).with_children(|parent| {
        parent.spawn(pause_btn).with_children(|parent| {
            parent.spawn(pause_btn_sprite);
        });
        parent.spawn(score).with_children(|parent| {
            parent.spawn(num_0);
        });
        parent.spawn(guide_parent).with_children(|parent| {
            parent.spawn(get_ready);
            parent.spawn(guide);
        });
        parent.spawn(pipe_parent2).with_children(|parent| {
            parent.spawn(pipe_top.clone());
            parent.spawn(pipe_bottom.clone());
            parent.spawn(pipe_score_collider.clone());
        });
        parent.spawn(pipe_parent3).with_children(|parent| {
            parent.spawn(pipe_top);
            parent.spawn(pipe_bottom);
            parent.spawn(pipe_score_collider);
        });
        parent.spawn(bird);
        parent.spawn(ground);
        parent.spawn(ground_collider);
        parent.spawn(sky_collider);
    });

    ew_resize.send(ResizeEvent);
    next_state.set(MyStates::Game(Game::Guide));
}
