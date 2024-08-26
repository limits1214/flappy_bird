use bevy::prelude::*;

use super::timer::BirdAnimateTimer;

#[derive(Component)]
pub struct Bg;

#[derive(Component)]
pub struct Bird;

#[derive(Bundle)]
pub struct BirdBundle {
    name: Name,
    bird: Bird,
    animate_timer: BirdAnimateTimer,
}

impl Default for BirdBundle {
    fn default() -> Self {
        BirdBundle {
            name: Name::new("bird"),
            animate_timer: BirdAnimateTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            bird: Bird,
        }
    }
}

#[derive(Component)]
pub struct Ground;

#[derive(Component)]
pub struct GroundCollider;

#[derive(Component)]
pub struct Guide;

#[derive(Component, Clone)]
pub struct Pipe;

#[derive(Component, Clone)]
pub struct PipeParent;

#[derive(Component, Clone)]
pub struct PipePoint;

#[derive(Component, Clone)]
pub struct PointEarned;

#[derive(Component)]
pub struct PanelParent;

#[derive(Component)]
pub struct ScoreParent;

#[derive(Component, Debug)]
pub struct NowScore(pub u32);

#[derive(Component, Debug)]
pub struct BestScore(pub u32);

#[derive(Component)]
pub struct Sparkle;
