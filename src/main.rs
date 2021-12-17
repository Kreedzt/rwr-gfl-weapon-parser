extern crate serde_xml_rs;
extern crate csv;

mod model;

use std::{fs, io};
use chrono::prelude::*;
use csv::Writer;
use serde_xml_rs::{from_reader};
use crate::model::Output;

fn main() {
    let local = Local::now();

    let current_time = local.format("%Y-%m-%d-%H-%M-%S").to_string();

    let folder_path = r#"D:\SteamLibrary\steamapps\workshop\content\270150\2513537759\media\packages\Girls_FrontLine\weapons"#;

    println!("current_time: {}", current_time);

    println!("folder_path: {}", folder_path);

    let entries = fs::read_dir(folder_path).expect("can't read dir")
        .map(|res| res.map(|e| e.path()))
        .filter(|path| path.as_ref().unwrap().display().to_string().ends_with(".weapon"))
        .collect::<Result<Vec<_>, io::Error>>()
        .expect("parse error");

    let output_file_name = format!("output-{}.csv", current_time);
    let mut writer = Writer::from_path(output_file_name).expect("can't output file");

    for path in entries {
        println!("path_name: {:?}", path.display().to_string());

        let de: model::Weapon = from_reader(fs::File::open(path).expect("fs open file error"))
            .expect("parse error");

        println!("{:?}", de);

        let mut output_struct: Output = model::Output::default();

        // 若包含 file， 表明存在模板
        if let Some(template_name) = de.file {
            println!("");
            println!("===Starting parsing template===");
            let template_de: model::Weapon = from_reader(fs::File::open(format!("{}\\{}", folder_path, template_name)).expect("can't open template file name")).expect("de template error");

            output_struct.key = template_de.key;
            output_struct.hud_icon = template_de.hud_icon.unwrap_or_default().filename;

            output_struct.drop_count_factor_on_death = template_de.drop_count_factor_on_death;
            output_struct.drop_count_factor_on_player_death = template_de.drop_count_factor_on_player_death;
            output_struct.time_to_live_out_in_the_open = template_de.time_to_live_out_in_the_open;
            output_struct.player_death_drop_owner_lock_time = template_de.player_death_drop_owner_lock_time;

            // output_struct.retrigger_time = template_de.specification.retrigger_time;
            // output_struct.accuracy_factor = template_de.specification.accuracy_factor;
            // output_struct.sustained_fire_grow_step = template_de.specification.sustained_fire_grow_step;
            // output_struct.sustained_fire_diminish_rate = template_de.specification.sustained_fire_diminish_rate;
            //
            // output_struct.magazine_size = template_de.specification.magazine_size;
            // output_struct.can_shoot_standing = template_de.specification.can_shoot_standing;
            // output_struct.sight_range_modifier = template_de.specification.sight_range_modifier;
            // output_struct.suppressed = template_de.specification.suppressed;
            // output_struct.name = template_de.specification.name;
            // output_struct.class = template_de.specification.class;
            //
            // output_struct.projectile_speed = template_de.specification.projectile_speed;
            // output_struct.barrel_offset = template_de.specification.barrel_offset;

        }

        if let Some(key) = de.key {
            output_struct.key.get_or_insert(key);
        }

        if let Some(hud_icon) = de.hud_icon.unwrap_or_default().filename {
            output_struct.hud_icon.get_or_insert(hud_icon);
        }

        // 掉落信息
        if let Some(drop_count_factor_on_death) = de.drop_count_factor_on_death {
            output_struct.drop_count_factor_on_death.get_or_insert(drop_count_factor_on_death);
        }

        if let Some(drop_count_factor_on_player_death) = de.drop_count_factor_on_player_death {
            output_struct.drop_count_factor_on_player_death.get_or_insert(drop_count_factor_on_player_death);
        }

        if let Some(time_to_live_out_in_the_open) = de.time_to_live_out_in_the_open {
            output_struct.time_to_live_out_in_the_open.get_or_insert(time_to_live_out_in_the_open);
        }

        if let Some(player_death_drop_owner_lock_time) = de.player_death_drop_owner_lock_time {
            output_struct.player_death_drop_owner_lock_time.get_or_insert(player_death_drop_owner_lock_time);
        }
        // End


        // 特殊信息
        if let Some(specification) = de.specification {
            if let Some(retrigger_time) = specification.retrigger_time {
                output_struct.retrigger_time.get_or_insert(retrigger_time);
            }

            if let Some(accuracy_factory) = specification.accuracy_factor {
                output_struct.accuracy_factor.get_or_insert(accuracy_factory);
            }

            if let Some(sustained_fire_grow_step) = specification.sustained_fire_grow_step {
                output_struct.sustained_fire_grow_step.get_or_insert(sustained_fire_grow_step);
            }

            if let Some(sustated_fire_diminish_rate) = specification.sustained_fire_diminish_rate {
                output_struct.sustained_fire_diminish_rate.get_or_insert(sustated_fire_diminish_rate);
            }

            if let Some(magazine_size) = specification.magazine_size {
                output_struct.magazine_size.get_or_insert(magazine_size);
            }

            if let Some(can_shoot_standing) = specification.can_shoot_standing {
                output_struct.can_shoot_standing.get_or_insert(can_shoot_standing);
            }

            if let Some(sight_range_modifier) = specification.sight_range_modifier {
                output_struct.sight_range_modifier.get_or_insert(sight_range_modifier);
            }

            if let Some(suppressed) = specification.suppressed {
                output_struct.suppressed.get_or_insert(suppressed);
            }

            if let Some(name) = specification.name {
                output_struct.name.get_or_insert(name);
            }

            if let Some(class) = specification.class {
                output_struct.class.get_or_insert(class);
            }

            if let Some(projectile_speed) = specification.projectile_speed {
                output_struct.projectile_speed.get_or_insert(projectile_speed);
            }

            if let Some(barrel_offset) = specification.barrel_offset {
                output_struct.barrel_offset.get_or_insert(barrel_offset);
            }
        }


        // writer.write_record(&["key", "hud_icon", "retrigger_time", "accuracy_factor",
        // "suppressed", "sustained_fire_grow_step", "sustained_fire_diminish_rate",
        //     "magazine_size", "projectile_speed", "barrel_offset", "can_shoot_standing", "sight_range_modifier",
        //     "suppressed", "name", "class", "projectile_speed", "barrel_offset"]);

        writer.serialize(output_struct);

    }

    writer.flush().expect("flush error");
}
