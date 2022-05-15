use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "weapon-parser", about = "rwr gf mod weapon data parser")]
pub struct Opt {
    /// weapon folder full path
    #[structopt(short, long, parse(from_os_str))]
    pub input: PathBuf,
    /// cn translation file full path
    #[structopt(short, long, parse(from_os_str))]
    pub translation: PathBuf,
    /// translation replace template file
    #[structopt(long, parse(from_os_str))]
    pub template: Option<PathBuf>,
}

// 适用于 CSV 输出的内容
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Output {
    // source file
    pub source_file_name: String,
    // weapon info
    pub key: Option<String>,
    pub weapon_template_file: Option<String>,
    pub radius: Option<f32>,
    pub transform_on_consume: Option<String>,
    pub on_ground_up: Option<String>,

    pub hud_icon: Option<String>,

    pub tag: Option<String>,

    pub drop_count_factor_on_death: Option<f32>,
    pub drop_count_factor_on_player_death: Option<f32>,
    pub time_to_live_out_in_the_open: Option<f32>,
    pub player_death_drop_owner_lock_time: Option<f32>,

    pub quality: Option<String>,

    pub carry_in_two_hands: Option<i8>,

    pub retrigger_time: Option<f32>,
    pub last_burst_retrigger_time: Option<f32>,

    pub accuracy_factor: Option<f32>,
    pub sustained_fire_grow_step: Option<f32>,
    pub sustained_fire_diminish_rate: Option<f32>,
    pub magazine_size: Option<i32>,

    pub can_shoot_standing: Option<u8>,
    pub can_shoot_crouching: Option<u8>,
    pub can_shoot_prone: Option<u8>,

    pub burst_shots: Option<f32>,
    pub sight_range_modifier: Option<f32>,
    pub ai_sight_range_modifier: Option<f32>,

    pub stab_enabled: Option<i8>,
    pub stab_range: Option<f32>,
    pub reload_one_at_a_time: Option<i8>,

    pub suppressed: Option<u8>,
    pub name: Option<String>,
    pub class: Option<u8>,
    pub slot: Option<i8>,

    pub projectile_speed: Option<f32>,
    pub projectiles_per_shot: Option<u8>,

    pub spread_range: Option<f32>,
    pub barrel_offset: Option<f32>,
    // FIXME: 源文件包含为空的内容
    // pub sight_height_offset: Option<f32>,
    pub sight_height_offset: Option<String>,

    pub running_accuracy: Option<f32>,
    pub walking_accuracy: Option<f32>,
    pub crouch_moving_accuracy: Option<f32>,
    pub standing_accuracy: Option<f32>,
    pub crouching_accuracy: Option<f32>,
    pub prone_accuracy: Option<f32>,
    pub prone_moving_accuracy: Option<f32>,
    pub over_wall_accuracy: Option<f32>,

    // next_in_chain
    pub next_in_chain_key: Option<String>,
    pub next_in_chain_share_ammo: Option<i8>,

    // modifier
    pub modifier_speed: Option<String>,

    // result
    pub result_hit_kill_probability: Option<f32>,
    pub result_hit_kill_decay_start_time: Option<f32>,
    pub result_hit_kill_decay_end_time: Option<f32>,

    pub use_basic_muzzle_smoke_effect: Option<i32>,
    pub spawn_instance_class: Option<String>,
    pub spawn_instance_key: Option<String>,
    pub consume: Option<i32>,
    pub deployment: Option<i32>,
    pub stance_accuracy_rate: Option<f32>,
    pub barrel_offset_ed: Option<String>,
    pub success_probability: Option<f32>,
    pub range: Option<f32>,
    pub character_state: Option<String>,
    pub cover_deployment: Option<i32>,
    pub affect_characters: Option<i32>,
    pub affect_vehicles: Option<i32>,
    pub damage: Option<f32>,
    pub untransform_equipment_class: Option<String>,
    pub untransform_count: Option<i32>,
    pub solt: Option<i32>,
    pub barrel_offset_3d: Option<String>,

    // pub modifier_speed: Option<f32>,

    // 猜测: 中文名称
    pub cn_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TemplateOutput {
    pub key: String,
    pub text: String,
}

impl Default for Output {
    fn default() -> Self {
        Output {
            source_file_name: "".to_string(),

            weapon_template_file: None,
            radius: None,
            key: None,
            hud_icon: None,
            transform_on_consume: None,
            on_ground_up: None,

            quality: None,

            carry_in_two_hands: None,

            tag: None,

            drop_count_factor_on_death: None,
            drop_count_factor_on_player_death: None,
            time_to_live_out_in_the_open: None,
            player_death_drop_owner_lock_time: None,

            retrigger_time: None,
            last_burst_retrigger_time: None,
            accuracy_factor: None,
            sustained_fire_grow_step: None,
            sustained_fire_diminish_rate: None,

            magazine_size: None,

            can_shoot_standing: None,
            can_shoot_crouching: None,
            can_shoot_prone: None,

            burst_shots: None,

            sight_range_modifier: None,
            ai_sight_range_modifier: None,
            stab_enabled: None,
            stab_range: None,
            reload_one_at_a_time: None,
            suppressed: None,
            name: None,
            class: None,

            slot: None,
            projectile_speed: None,
            projectiles_per_shot: None,
            spread_range: None,
            barrel_offset: None,
            sight_height_offset: None,

            running_accuracy: None,
            walking_accuracy: None,
            crouch_moving_accuracy: None,
            crouching_accuracy: None,
            standing_accuracy: None,
            prone_accuracy: None,
            prone_moving_accuracy: None,
            over_wall_accuracy: None,

            // next_in_chain
            next_in_chain_key: None,
            next_in_chain_share_ammo: None,

            // modifier
            modifier_speed: None,

            // result
            result_hit_kill_probability: None,
            result_hit_kill_decay_start_time: None,
            result_hit_kill_decay_end_time: None,

            use_basic_muzzle_smoke_effect: None,
            spawn_instance_class: None,
            spawn_instance_key: None,
            consume: None,
            deployment: None,
            stance_accuracy_rate: None,
            barrel_offset_ed: None,
            success_probability: None,
            range: None,
            character_state: None,
            cover_deployment: None,
            affect_characters: None,
            affect_vehicles: None,
            damage: None,
            untransform_equipment_class: None,
            untransform_count: None,
            solt: None,
            barrel_offset_3d: None,
            // modifier_speed: None
            cn_name: None,
        }
    }
}

impl Default for TemplateOutput {
    fn default() -> Self {
        Self {
            key: "".to_string(),
            text: "".to_string(),
        }
    }
}
