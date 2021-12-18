extern crate serde_xml_rs;
extern crate csv;

mod model;
mod decode;

use std::{fs, io};
use std::io::Read;
use std::path::{Path, PathBuf};
use chrono::prelude::*;
use structopt::StructOpt;
use csv::Writer;
use serde_xml_rs::{from_reader, from_str};
use crate::model::{Output, Weapon};

fn main() {
    let local = Local::now();

    let current_time = local.format("%Y-%m-%d-%H-%M-%S").to_string();

    // let folder_path = r#"D:\SteamLibrary\steamapps\workshop\content\270150\2513537759\media\packages\Girls_FrontLine\weapons"#;

    let opt = model::Opt::from_args();
    let folder_path: PathBuf = opt.input;

    println!("current_time: {}", current_time);

    println!("folder_path: {}", folder_path.display().to_string());

    let entries = fs::read_dir(folder_path.clone()).expect("can't read dir")
        .map(|res| res.map(|e| e.path()))
        .filter(|path| path.as_ref().unwrap().display().to_string().ends_with(".weapon"))
        .collect::<Result<Vec<_>, io::Error>>()
        .expect("parse error");

    let output_file_name = format!("output-{}.csv", current_time);
    let mut writer = Writer::from_path(&output_file_name).expect("can't output file");

    let total = entries.len();

    for (index, path) in entries.into_iter().enumerate() {
        println!("process: {} / {}", index + 1, total);
        println!("===Starting parsing file: {}===", path.display().to_string());

        let res_str = decode::read_file_decode_to_utf8(&path.into_os_string().into_string().unwrap());

        let de: model::Weapon = from_str(&res_str).expect("parse error");

        println!("{:?}", de);

        let mut output_struct: Output = model::Output::default();

        // 若包含 file， 表明存在模板
        if let Some(template_name) = de.file {
            println!("found template");
            println!("===Starting parsing template: {}===", template_name);
            //let template_de: model::Weapon = from_reader(fs::File::open(format!("{}\\{}", folder_path, template_name)).expect("can't open template file name")).expect("de template error");

            let template_file_path = format!("{}\\{}", folder_path.clone().display().to_string(), template_name);
            let res_str = decode::read_file_decode_to_utf8(&template_file_path);
            let template_de: Weapon = from_str(&res_str).expect("parse str err");

            output_struct.key = template_de.key;
            output_struct.hud_icon = template_de.hud_icon.unwrap_or_default().filename;

            output_struct.drop_count_factor_on_death = template_de.drop_count_factor_on_death;
            output_struct.drop_count_factor_on_player_death = template_de.drop_count_factor_on_player_death;
            output_struct.time_to_live_out_in_the_open = template_de.time_to_live_out_in_the_open;
            output_struct.player_death_drop_owner_lock_time = template_de.player_death_drop_owner_lock_time;

            println!("===Parsing template completed===");
        }

        if let Some(key) = de.key {
            output_struct.key = Some(key);
        }

        if let Some(hud_icon) = de.hud_icon.unwrap_or_default().filename {
            output_struct.hud_icon = Some(hud_icon);
        }

        // 掉落信息
        if let Some(drop_count_factor_on_death) = de.drop_count_factor_on_death {
            output_struct.drop_count_factor_on_death = Some(drop_count_factor_on_death);
        }

        if let Some(drop_count_factor_on_player_death) = de.drop_count_factor_on_player_death {
            output_struct.drop_count_factor_on_player_death = Some(drop_count_factor_on_player_death);
        }

        if let Some(time_to_live_out_in_the_open) = de.time_to_live_out_in_the_open {
            output_struct.time_to_live_out_in_the_open = Some(time_to_live_out_in_the_open);
        }

        if let Some(player_death_drop_owner_lock_time) = de.player_death_drop_owner_lock_time {
            output_struct.player_death_drop_owner_lock_time = Some(player_death_drop_owner_lock_time);
        }
        // End


        // 特殊信息
        if let Some(specification) = de.specification {
            if let Some(retrigger_time) = specification.retrigger_time {
                output_struct.retrigger_time = Some(retrigger_time);
            }

            if let Some(accuracy_factory) = specification.accuracy_factor {
                output_struct.accuracy_factor = Some(accuracy_factory);
            }

            if let Some(sustained_fire_grow_step) = specification.sustained_fire_grow_step {
                output_struct.sustained_fire_grow_step = Some(sustained_fire_grow_step);
            }

            if let Some(sustated_fire_diminish_rate) = specification.sustained_fire_diminish_rate {
                output_struct.sustained_fire_diminish_rate = Some(sustated_fire_diminish_rate);
            }

            if let Some(magazine_size) = specification.magazine_size {
                output_struct.magazine_size = Some(magazine_size);
            }

            if let Some(can_shoot_standing) = specification.can_shoot_standing {
                output_struct.can_shoot_standing = Some(can_shoot_standing);
            }

            if let Some(sight_range_modifier) = specification.sight_range_modifier {
                output_struct.sight_range_modifier = Some(sight_range_modifier);
            }

            if let Some(suppressed) = specification.suppressed {
                output_struct.suppressed = Some(suppressed);
            }

            if let Some(name) = specification.name {
                output_struct.name = Some(name);
            }

            if let Some(class) = specification.class {
                output_struct.class = Some(class);
            }

            if let Some(projectile_speed) = specification.projectile_speed {
                output_struct.projectile_speed = Some(projectile_speed);
            }

            if let Some(barrel_offset) = specification.barrel_offset {
                output_struct.barrel_offset = Some(barrel_offset);
            }
        }


        // writer.write_record(&["key", "hud_icon", "retrigger_time", "accuracy_factor",
        // "suppressed", "sustained_fire_grow_step", "sustained_fire_diminish_rate",
        //     "magazine_size", "projectile_speed", "barrel_offset", "can_shoot_standing", "sight_range_modifier",
        //     "suppressed", "name", "class", "projectile_speed", "barrel_offset"]);

        println!("===Parsing file completed===");
        writer.serialize(output_struct);

    }

    writer.flush().expect("flush error");

    println!("Task completed, fileName: {}", output_file_name);
}
