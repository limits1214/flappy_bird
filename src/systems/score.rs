use crate::{
    components::game::ScoreParent,
    events::game::ScoreUpEvent,
    resources::{assets::FlappyBirdAssets, game::GameConfig},
};
use bevy::prelude::*;

pub fn score_up(
    mut commands: Commands,
    mut reader: EventReader<ScoreUpEvent>,
    mut config: ResMut<GameConfig>,
    score_parent: Query<(Entity, &Transform, &Children), With<ScoreParent>>,
    fb_assets: Res<FlappyBirdAssets>,
) {
    for _ in reader.read() {
        config.score += 1;
        // scoreing_helper(&mut commands, &score_parent, &fb_assets, config.score, 13., true);
        scoring_helper2(ScoreingHelperArgs {
            commands: &mut commands,
            query: &score_parent,
            fb_assets: &fb_assets,
            is_large: true,
            number: config.score,
            offset: 13.,
        });

        // if let Ok((entity, transform, children)) = score_parent.get_single() {

        // // 기존 제거
        // for &entity in children {
        //     if let Some(ec) = commands.get_entity(entity) {
        //         ec.despawn_recursive();
        //     }
        // }

        // let score_str = config.score.to_string();
        // let mut x_offset = 0.;
        // let offset = 13.;
        // let vstr = score_str
        //     .chars()
        //     .map(|char| char.to_string())
        //     .map(|str| {
        //         let e = commands.spawn(
        //             (
        //                 Name::new("num"),
        //                 SpriteBundle {
        //                     texture: fb_assets.get_large_num(&str),
        //                     transform: Transform {
        //                         translation: Vec3::new(x_offset, 0., 0.),
        //                         ..default()
        //                     },
        //                     ..default()
        //                 }
        //             )
        //         ).id();
        //         x_offset += offset;
        //         return e;
        //     })
        //     .collect::<Vec<_>>();
        // commands.entity(entity).push_children(vstr.as_slice());

        // let tr = transform.translation;
        // let adjust_x = -1. * (x_offset - offset) / 2.;
        // commands.entity(entity).insert(Transform {
        //     translation: Vec3::new(adjust_x, tr.y, tr.z),
        //     ..default()
        // });
        // }
    }
}

struct TestCallArgs<'a, 'b> {
    str: &'a str,
    str2: &'b str,
}

fn test_call(args: TestCallArgs) {}

pub struct ScoreingHelperArgs<'a, 'w, 's, 'world, 'state, 'c, 'd, 'e, T: Component> {
    pub commands: &'a mut Commands<'w, 's>,
    pub query: &'a Query<'world, 'state, (Entity, &'c Transform, &'d Children), With<T>>,
    pub fb_assets: &'a Res<'e, FlappyBirdAssets>,
    pub number: u32,
    pub offset: f32,
    pub is_large: bool,
}

pub fn scoring_helper2<T: Component>(args: ScoreingHelperArgs<T>) {
    let ScoreingHelperArgs {
        commands,
        query,
        fb_assets,
        is_large,
        number,
        offset,
    } = args;
    scoreing_helper(commands, query, fb_assets, number, offset, is_large);
}

pub fn scoreing_helper<T: Component>(
    commands: &mut Commands,
    query: &Query<(Entity, &Transform, &Children), With<T>>,
    fb_assets: &Res<FlappyBirdAssets>,
    number: u32,
    offset: f32,
    is_large: bool,
) {
    if let Ok((entity, transform, children)) = query.get_single() {
        // 기존 제거
        for &entity in children {
            if let Some(ec) = commands.get_entity(entity) {
                ec.despawn_recursive();
            }
        }
        let mut x_offset = 0.;
        let vstr =
            get_score_entitiy_vec(commands, fb_assets, is_large, offset, number, &mut x_offset);
        commands.entity(entity).push_children(vstr.as_slice());

        let tr = transform.translation;
        let adjust_x = -1. * (x_offset - offset) / 2.;
        commands.entity(entity).insert(Transform {
            translation: Vec3::new(adjust_x, tr.y, tr.z),
            ..default()
        });
    }
}

pub fn get_score_entitiy_vec(
    commands: &mut Commands,
    fb_assets: &Res<FlappyBirdAssets>,
    is_large: bool,
    offset: f32,
    number: u32,
    x_offset: &mut f32,
) -> Vec<Entity> {
    let score_str = number.to_string();
    // let mut x_offset = 0.;
    // let offset = 13.;
    let vstr = score_str
        .chars()
        .map(|char| char.to_string())
        .map(|str| {
            let e = commands
                .spawn((
                    Name::new("num"),
                    SpriteBundle {
                        texture: if is_large {
                            fb_assets.get_large_num(&str)
                        } else {
                            fb_assets.get_middle_num(&str)
                        },
                        transform: Transform {
                            translation: Vec3::new(*x_offset, 0., 0.),
                            ..default()
                        },
                        ..default()
                    },
                ))
                .id();
            *x_offset += offset;
            return e;
        })
        .collect::<Vec<_>>();
    vstr
}

#[test]
fn test() {
    let score = 1;
    let binding = score.to_string();
    let score_str = binding.split("").collect::<Vec<_>>();
    println!("score: {:?}", score_str);
}
