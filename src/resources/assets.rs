use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_kira_audio::AudioSource;

#[derive(AssetCollection, Resource)]
pub struct FlappyBirdAssets {
    //audio
    #[asset(path = "audio/sfx_die.wav")]
    pub sfx_die: Handle<AudioSource>,
    #[asset(path = "audio/sfx_hit.wav")]
    pub sfx_hit: Handle<AudioSource>,
    #[asset(path = "audio/sfx_point.wav")]
    pub sfx_point: Handle<AudioSource>,
    #[asset(path = "audio/sfx_swooshing.wav")]
    pub sfx_swooshing: Handle<AudioSource>,
    #[asset(path = "audio/sfx_wing.wav")]
    pub sfx_wing: Handle<AudioSource>,
    //sprites
    #[asset(path = "sprites/background_day.png")]
    pub background_day: Handle<Image>,
    #[asset(path = "sprites/ground.png")]
    pub ground: Handle<Image>,
    #[asset(path = "sprites/bird_orange_0.png")]
    pub bird_orange_0: Handle<Image>,
    #[asset(path = "sprites/bird_orange_1.png")]
    pub bird_orange_1: Handle<Image>,
    #[asset(path = "sprites/bird_orange_2.png")]
    pub bird_orange_2: Handle<Image>,
    #[asset(path = "sprites/button_menu.png")]
    pub button_menu: Handle<Image>,
    #[asset(path = "sprites/button_ok.png")]
    pub button_ok: Handle<Image>,
    #[asset(path = "sprites/button_ad.png")]
    pub button_ad: Handle<Image>,
    #[asset(path = "sprites/button_pause.png")]
    pub button_pause: Handle<Image>,
    #[asset(path = "sprites/button_resume.png")]
    pub button_resume: Handle<Image>,
    #[asset(path = "sprites/button_play_normal.png")]
    pub button_play_normal: Handle<Image>,
    #[asset(path = "sprites/button_play_pressed.png")]
    pub button_play_pressed: Handle<Image>,
    #[asset(path = "sprites/instructions.png")]
    pub instructions: Handle<Image>,
    #[asset(path = "sprites/label_flappy_bird.png")]
    pub label_flappy_bird: Handle<Image>,
    #[asset(path = "sprites/label_game_over.png")]
    pub label_game_over: Handle<Image>,
    #[asset(path = "sprites/label_get_ready.png")]
    pub label_get_ready: Handle<Image>,
    #[asset(path = "sprites/label_new.png")]
    pub label_new: Handle<Image>,
    #[asset(path = "sprites/medal_bronze.png")]
    pub medal_bronze: Handle<Image>,
    #[asset(path = "sprites/medal_gold.png")]
    pub medal_gold: Handle<Image>,
    #[asset(path = "sprites/medal_platinum.png")]
    pub medal_platinum: Handle<Image>,
    #[asset(path = "sprites/medal_silver.png")]
    pub medal_silver: Handle<Image>,
    #[asset(path = "sprites/number_large_0.png")]
    pub number_large_0: Handle<Image>,
    #[asset(path = "sprites/number_large_1.png")]
    pub number_large_1: Handle<Image>,
    #[asset(path = "sprites/number_large_2.png")]
    pub number_large_2: Handle<Image>,
    #[asset(path = "sprites/number_large_3.png")]
    pub number_large_3: Handle<Image>,
    #[asset(path = "sprites/number_large_4.png")]
    pub number_large_4: Handle<Image>,
    #[asset(path = "sprites/number_large_5.png")]
    pub number_large_5: Handle<Image>,
    #[asset(path = "sprites/number_large_6.png")]
    pub number_large_6: Handle<Image>,
    #[asset(path = "sprites/number_large_7.png")]
    pub number_large_7: Handle<Image>,
    #[asset(path = "sprites/number_large_8.png")]
    pub number_large_8: Handle<Image>,
    #[asset(path = "sprites/number_large_9.png")]
    pub number_large_9: Handle<Image>,
    #[asset(path = "sprites/number_middle_0.png")]
    pub number_middle_0: Handle<Image>,
    #[asset(path = "sprites/number_middle_1.png")]
    pub number_middle_1: Handle<Image>,
    #[asset(path = "sprites/number_middle_2.png")]
    pub number_middle_2: Handle<Image>,
    #[asset(path = "sprites/number_middle_3.png")]
    pub number_middle_3: Handle<Image>,
    #[asset(path = "sprites/number_middle_4.png")]
    pub number_middle_4: Handle<Image>,
    #[asset(path = "sprites/number_middle_5.png")]
    pub number_middle_5: Handle<Image>,
    #[asset(path = "sprites/number_middle_6.png")]
    pub number_middle_6: Handle<Image>,
    #[asset(path = "sprites/number_middle_7.png")]
    pub number_middle_7: Handle<Image>,
    #[asset(path = "sprites/number_middle_8.png")]
    pub number_middle_8: Handle<Image>,
    #[asset(path = "sprites/number_middle_9.png")]
    pub number_middle_9: Handle<Image>,
    #[asset(path = "sprites/panel_score.png")]
    pub panel_score: Handle<Image>,
    #[asset(path = "sprites/pipe_green_bottom.png")]
    pub pipe_green_bottom: Handle<Image>,
    #[asset(path = "sprites/pipe_green_top.png")]
    pub pipe_green_top: Handle<Image>,
    #[asset(path = "sprites/spark_0.png")]
    pub spark_0: Handle<Image>,
    #[asset(path = "sprites/spark_1.png")]
    pub spark_1: Handle<Image>,
    #[asset(path = "sprites/spark_2.png")]
    pub spark_2: Handle<Image>,
    // generated atlas
    pub gen_bird_atlas_layout: Handle<TextureAtlasLayout>,
    pub gen_bird_atlas_texture: Handle<Image>,
    pub gen_sparkle_atlas_layout: Handle<TextureAtlasLayout>,
    pub gen_sparkle_atlas_texture: Handle<Image>,
}

impl FlappyBirdAssets {
    pub fn get_large_num(&self, num: &str) -> Handle<Image> {
        match num {
            "0" => self.number_large_0.clone(),
            "1" => self.number_large_1.clone(),
            "2" => self.number_large_2.clone(),
            "3" => self.number_large_3.clone(),
            "4" => self.number_large_4.clone(),
            "5" => self.number_large_5.clone(),
            "6" => self.number_large_6.clone(),
            "7" => self.number_large_7.clone(),
            "8" => self.number_large_8.clone(),
            "9" => self.number_large_9.clone(),
            _ => unreachable!(),
        }
    }

    pub fn get_middle_num(&self, num: &str) -> Handle<Image> {
        match num {
            "0" => self.number_middle_0.clone(),
            "1" => self.number_middle_1.clone(),
            "2" => self.number_middle_2.clone(),
            "3" => self.number_middle_3.clone(),
            "4" => self.number_middle_4.clone(),
            "5" => self.number_middle_5.clone(),
            "6" => self.number_middle_6.clone(),
            "7" => self.number_middle_7.clone(),
            "8" => self.number_middle_8.clone(),
            "9" => self.number_middle_9.clone(),
            _ => unreachable!(),
        }
    }
}
