use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "weapon-parser", about = "rwr gf mod weapon data parser")]
pub struct Opt {
    /// weapon folder full path
    #[structopt(short, long, parse(from_os_str))]
    pub input: PathBuf
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TagEnum {
    #[serde(rename = "name")]
    Name(String)

    // Item {
    //     name: String,
    // }
    // #[serde(rename = "name")]
    // Name(String)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tag {
    #[serde(flatten)]
    item: TagEnum
}

// 模板暂未读取
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Specification {
    pub retrigger_time: Option<f32>,
    pub accuracy_factor: Option<f32>,
    pub sustained_fire_grow_step: Option<f32>,
    pub sustained_fire_diminish_rate: Option<f32>,
    pub magazine_size: Option<i32>,

    pub can_shoot_standing: Option<u8>,
    pub can_shoot_crouching: Option<u8>,
    pub can_shoot_prone: Option<u8>,

    pub burst_shots: Option<i32>,
    pub sight_range_modifier: Option<f32>,
    pub suppressed: Option<u8>,
    pub name: Option<String>,
    pub class: Option<u8>,
    pub projectile_speed: Option<f32>,

    pub projectiles_per_shot: Option<u8>,
    pub barrel_offset: Option<f32>
}

/**
 包含属性:
[running]: accuracy: f32,
[walking]: ..
[crouching]
[crouch_moving]
[standing]
[prone]
[prone_moving]
[over_wall]

*/
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Stance {
    pub state_key: String,
    pub accuracy: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum StanceEnum {
    #[serde(rename = "accuracy")]
    Accuracy(f32)
}

// 包含内容
/**
 包含属性:
 speed: f32

*/
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Modifier {
    pub class: String,
    pub value: f32
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ModifierEnum {
    #[serde(rename = "class")]
    Class(String),
    #[serde(rename = "value")]
    Value(f32)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HudIcon {
    pub filename: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Weapon {
    pub file: Option<String>,
    pub key: Option<String>,
    pub on_ground_up: Option<String>,

    pub drop_count_factor_on_death: Option<f32>,
    pub drop_count_factor_on_player_death: Option<f32>,
    pub time_to_live_out_in_the_open: Option<f32>,
    pub player_death_drop_owner_lock_time: Option<f32>,

    // #[serde(flatten)]
    // pub tag: Option<Vec<TagEnum>>,

    // pub tag: Option<HashMap<String, String>>,

    // TODO
    // pub tag: Option<Vec<Tag>>,

    pub specification: Option<Specification>,

    pub hud_icon: Option<HudIcon>,

    // TODO
    // pub modifier: Option<HashMap<String, String>>,

    pub stance: Option<Vec<Stance>>,
}


// 适用于 CSV 输出的内容
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Output {
    pub key: Option<String>,
    pub hud_icon: Option<String>,

    pub drop_count_factor_on_death: Option<f32>,
    pub drop_count_factor_on_player_death: Option<f32>,
    pub time_to_live_out_in_the_open: Option<f32>,
    pub player_death_drop_owner_lock_time: Option<f32>,

    pub retrigger_time: Option<f32>,
    pub accuracy_factor: Option<f32>,
    pub sustained_fire_grow_step: Option<f32>,
    pub sustained_fire_diminish_rate: Option<f32>,

    pub magazine_size: Option<i32>,
    pub can_shoot_standing: Option<u8>,
    pub can_shoot_crouching: Option<u8>,
    pub can_shoot_prone: Option<u8>,

    pub burst_shots: Option<i32>,

    pub sight_range_modifier: Option<f32>,
    pub suppressed: Option<u8>,
    pub name: Option<String>,
    pub class: Option<u8>,

    pub projectile_speed: Option<f32>,
    pub projectiles_per_shot: Option<u8>,
    pub barrel_offset: Option<f32>,

    pub running_accuracy: Option<f32>,
    pub walking_accuracy: Option<f32>,
    pub crouch_moving_accuracy: Option<f32>,
    pub standing_accuracy: Option<f32>,
    pub crouching_accuracy: Option<f32>,
    pub prone_accuracy: Option<f32>,
    pub prone_moving_accuracy: Option<f32>,
    pub over_wall_accuracy: Option<f32>
}

impl Default for Output {
    fn default() -> Self {
        Output {
            key: None,
            hud_icon: None,

            drop_count_factor_on_death: None,
            drop_count_factor_on_player_death: None,
            time_to_live_out_in_the_open: None,
            player_death_drop_owner_lock_time: None,

            retrigger_time: None,
            accuracy_factor: None,
            sustained_fire_grow_step: None,
            sustained_fire_diminish_rate: None,

            magazine_size: None,

            can_shoot_standing: None,
            can_shoot_crouching: None,
            can_shoot_prone: None,

            burst_shots: None,

            sight_range_modifier: None,
            suppressed: None,
            name: None,
            class: None,

            projectile_speed: None,
            projectiles_per_shot: None,
            barrel_offset: None,

            running_accuracy: None,
            walking_accuracy: None,
            crouch_moving_accuracy: None,
            crouching_accuracy: None,
            standing_accuracy: None,
            prone_accuracy: None,
            prone_moving_accuracy: None,
            over_wall_accuracy: None
        }
    }
}

impl Default for Specification {
    fn default() -> Self {
        Specification {
            retrigger_time: None,
            accuracy_factor: None,
            sustained_fire_grow_step: None,
            sustained_fire_diminish_rate: None,
            magazine_size: None,
            can_shoot_standing: None,
            // TODO
            can_shoot_crouching: None,
            // TODO
            can_shoot_prone: None,
            burst_shots: None,
            sight_range_modifier: None,
            suppressed: None,
            name: None,
            class: None,
            projectile_speed: None,
            // TODO
            barrel_offset: None,
            // TODO
            projectiles_per_shot: None
        }
    }
}

// impl Default for Tag {
//     fn default() -> Self {
//         Tag {
//             name: None
//         }
//     }
// }

impl Default for HudIcon {
    fn default() -> Self {
        HudIcon {
            filename: None,
        }
    }
}

