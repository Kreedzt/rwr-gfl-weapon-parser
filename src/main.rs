extern crate csv;

mod decode;
mod model;

use crate::model::{Output, Weapon};
use chrono::prelude::*;
use csv::Writer;
use quick_xml::{events::Event, Reader};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::{fs, io};
use structopt::StructOpt;

fn main() {
    let local = Local::now();

    let current_time = local.format("%Y-%m-%d-%H-%M-%S").to_string();

    // let folder_path = r#"D:\SteamLibrary\steamapps\workshop\content\270150\2513537759\media\packages\Girls_FrontLine\weapons"#;

    let opt = model::Opt::from_args();
    let folder_path: PathBuf = opt.input;

    println!("current_time: {}", current_time);

    println!("folder_path: {}", folder_path.display().to_string());

    let entries = fs::read_dir(folder_path.clone())
        .expect("can't read dir")
        .map(|res| res.map(|e| e.path()))
        .filter(|path| {
            path.as_ref()
                .unwrap()
                .display()
                .to_string()
                .ends_with(".weapon")
        })
        .collect::<Result<Vec<_>, io::Error>>()
        .expect("parse error");

    let output_file_name = format!("weapon-parser-output-{}.csv", current_time);
    // let mut writer = Writer::from_path(&output_file_name).expect("can't output file");

    let total = entries.len();

    for (index, path) in entries.into_iter().enumerate() {
        println!("process: {} / {}", index + 1, total);
        println!(
            "===Starting parsing file: {}===",
            path.display().to_string()
        );

        let res_str =
            decode::read_file_decode_to_utf8(&path.into_os_string().into_string().unwrap());

        // 手动解析
        // let de: model::Weapon = from_str(&res_str).expect("parse error");

        let mut reader = Reader::from_str(&res_str);
        reader.trim_text(true);

        let mut buf = Vec::new();
        let mut buf2 = Vec::new();

        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    match e.name() {
                        b"weapon" => {
                            // 一级内容
                            for attr in e.attributes() {
                                let attr_unwrap_res = attr.unwrap();
                                let attr_value = attr_unwrap_res.unescape_and_decode_value(&reader);
                                let attr_key = unsafe {
                                    String::from_utf8_unchecked(attr_unwrap_res.key.to_vec())
                                };

                                match attr_key.as_str() {
                                    "file" => {
                                        println!("weapon file value: {:?}", attr_value.unwrap());
                                    },
                                    "key" => {
                                        println!("weapon key value: {:?}", attr_value.unwrap());
                                    },
                                    "on_ground_up" => {
                                        println!("weapon on_ground_up value: {:?}", attr_value.unwrap());
                                    },
                                    _ => {
                                        println!("Don't care weapon attr: {}", attr_key);
                                    }
                                }
                            }
                        },
                        b"tag" => {
                            println!("tag=====");

                            for attr in e.attributes() {
                                let attr_unwrap_res = attr.unwrap();
                                let attr_value = attr_unwrap_res.unescape_and_decode_value(&reader);
                                let attr_key = unsafe {
                                    String::from_utf8_unchecked(attr_unwrap_res.key.to_vec())
                                };

                                match attr_key.as_str() {
                                    "name" => {
                                        println!("tag name value: {:?}", attr_value.unwrap());
                                    },
                                    _ => {
                                        println!("Don't care tag attr: {}", attr_key);
                                    }
                                }
                            }
                        },
                        b"specification" => {
                            println!("specification=======");
                            for attr in e.attributes() {
                                let attr_unwrap_res = attr.unwrap();
                                let attr_value = attr_unwrap_res.unescape_and_decode_value(&reader);
                                let attr_key = unsafe {
                                    String::from_utf8_unchecked(attr_unwrap_res.key.to_vec())
                                };

                                match attr_key.as_str() {
                                    "retrigger_time" => {
                                        println!("Specification retrigger_time value: {:?}", attr_value.unwrap());
                                    },
                                    _ => {
                                        println!("Don't care specification attr: {}", attr_key);
                                    }
                                }
                            }
                        },
                        _ => {}
                    }
                },
                Ok(Event::Text(e)) => {
                    println!("text: {}", e.unescape_and_decode(&reader).unwrap());
                }
                Ok(Event::Eof) => break,
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
        }

        break;

        // println!("{:?}", de);

        let mut output_struct: Output = model::Output::default();

        // 若包含 file， 表明存在模板
        // if let Some(template_name) = de.file {
        //     println!("found template");
        //     println!("===Starting parsing template: {}===", template_name);
        //     //let template_de: model::Weapon = from_reader(fs::File::open(format!("{}\\{}", folder_path, template_name)).expect("can't open template file name")).expect("de template error");
        //
        //     let template_file_path = format!("{}\\{}", folder_path.clone().display().to_string(), template_name);
        //     let res_str = decode::read_file_decode_to_utf8(&template_file_path);
        //     let template_de: Weapon = from_str(&res_str).expect("parse str err");
        //
        //     println!("{:?}", template_de);
        //
        //     output_struct.key = template_de.key;
        //     output_struct.hud_icon = template_de.hud_icon.unwrap_or_default().filename;
        //
        //     output_struct.drop_count_factor_on_death = template_de.drop_count_factor_on_death;
        //     output_struct.drop_count_factor_on_player_death = template_de.drop_count_factor_on_player_death;
        //     output_struct.time_to_live_out_in_the_open = template_de.time_to_live_out_in_the_open;
        //     output_struct.player_death_drop_owner_lock_time = template_de.player_death_drop_owner_lock_time;

        // stance 信息
        //     if let Some(stance_vec) = template_de.stance {
        //         for stance_item in stance_vec {
        //             match stance_item.state_key.as_str() {
        //                 "running" => {
        //                     output_struct.running_accuracy = Some(stance_item.accuracy);
        //                 },
        //                 "walking" => {
        //                     output_struct.walking_accuracy = Some(stance_item.accuracy);
        //                 },
        //                 "crouching" => {
        //                     output_struct.crouching_accuracy = Some(stance_item.accuracy);
        //                 },
        //                 "crouch_moving" => {
        //                     output_struct.crouch_moving_accuracy = Some(stance_item.accuracy);
        //                 },
        //                 "standing" => {
        //                     output_struct.standing_accuracy = Some(stance_item.accuracy);
        //                 },
        //                 "prone"  => {
        //                     output_struct.prone_accuracy = Some(stance_item.accuracy);
        //                 },
        //                 "prone_moving" => {
        //                     output_struct.prone_moving_accuracy = Some(stance_item.accuracy);
        //                 },
        //                 "over_wall" => {
        //                     output_struct.over_wall_accuracy = Some(stance_item.accuracy);
        //                 },
        //                 _ => {
        //                     println!("Not matched state key: {}", stance_item.state_key);
        //                 }
        //             }
        //         }
        //     }
        //
        //
        //     println!("===Parsing template completed===");
        // }
        // 模板解析结束

        // if let Some(key) = de.key {
        //     output_struct.key = Some(key);
        // }
        //
        // if let Some(hud_icon) = de.hud_icon.unwrap_or_default().filename {
        //     output_struct.hud_icon = Some(hud_icon);
        // }
        //
        // // 掉落信息
        // if let Some(drop_count_factor_on_death) = de.drop_count_factor_on_death {
        //     output_struct.drop_count_factor_on_death = Some(drop_count_factor_on_death);
        // }
        //
        // if let Some(drop_count_factor_on_player_death) = de.drop_count_factor_on_player_death {
        //     output_struct.drop_count_factor_on_player_death = Some(drop_count_factor_on_player_death);
        // }
        //
        // if let Some(time_to_live_out_in_the_open) = de.time_to_live_out_in_the_open {
        //     output_struct.time_to_live_out_in_the_open = Some(time_to_live_out_in_the_open);
        // }
        //
        // if let Some(player_death_drop_owner_lock_time) = de.player_death_drop_owner_lock_time {
        //     output_struct.player_death_drop_owner_lock_time = Some(player_death_drop_owner_lock_time);
        // }
        // // End
        //
        //
        // // 特殊信息
        // if let Some(specification) = de.specification {
        //     if let Some(retrigger_time) = specification.retrigger_time {
        //         output_struct.retrigger_time = Some(retrigger_time);
        //     }
        //
        //     if let Some(last_burst_retrigger_time) = specification.last_burst_retrigger_time {
        //         output_struct.last_burst_retrigger_time = Some(last_burst_retrigger_time);
        //     }
        //
        //     if let Some(accuracy_factory) = specification.accuracy_factor {
        //         output_struct.accuracy_factor = Some(accuracy_factory);
        //     }
        //
        //     if let Some(sustained_fire_grow_step) = specification.sustained_fire_grow_step {
        //         output_struct.sustained_fire_grow_step = Some(sustained_fire_grow_step);
        //     }
        //
        //     if let Some(sustated_fire_diminish_rate) = specification.sustained_fire_diminish_rate {
        //         output_struct.sustained_fire_diminish_rate = Some(sustated_fire_diminish_rate);
        //     }
        //
        //     if let Some(magazine_size) = specification.magazine_size {
        //         output_struct.magazine_size = Some(magazine_size);
        //     }
        //
        //     if let Some(can_shoot_standing) = specification.can_shoot_standing {
        //         output_struct.can_shoot_standing = Some(can_shoot_standing);
        //     }
        //
        //     if let Some(can_shoot_crouching) = specification.can_shoot_crouching {
        //         output_struct.can_shoot_crouching = Some(can_shoot_crouching);
        //     }
        //
        //     if let Some(can_shoot_prone) = specification.can_shoot_prone {
        //         output_struct.can_shoot_prone = Some(can_shoot_prone);
        //     }
        //
        //     if let Some(burst_shots) = specification.burst_shots {
        //         output_struct.burst_shots = Some(burst_shots);
        //     }
        //
        //     if let Some(sight_range_modifier) = specification.sight_range_modifier {
        //         output_struct.sight_range_modifier = Some(sight_range_modifier);
        //     }
        //
        //     if let Some(ai_sight_range_modifier) = specification.ai_sight_range_modifier {
        //         output_struct.ai_sight_range_modifier = Some(ai_sight_range_modifier);
        //     }
        //
        //     if let Some(stab_enabled) = specification.stab_enabled {
        //         output_struct.stab_enabled = Some(stab_enabled);
        //     }
        //
        //     if let Some(stab_range) = specification.stab_range {
        //         output_struct.stab_range = Some(stab_range);
        //     }
        //
        //     if let Some(reload_one_at_a_time) = specification.reload_one_at_a_time {
        //         output_struct.reload_one_at_a_time = Some(reload_one_at_a_time);
        //     }
        //
        //     if let Some(suppressed) = specification.suppressed {
        //         output_struct.suppressed = Some(suppressed);
        //     }
        //
        //     if let Some(name) = specification.name {
        //         output_struct.name = Some(name);
        //     }
        //
        //     if let Some(class) = specification.class {
        //         output_struct.class = Some(class);
        //     }
        //
        //     if let Some(slot) = specification.slot {
        //         output_struct.slot = Some(slot);
        //     }
        //
        //     if let Some(projectile_speed) = specification.projectile_speed {
        //         output_struct.projectile_speed = Some(projectile_speed);
        //     }
        //
        //     if let Some(projectiles_per_shot) = specification.projectiles_per_shot {
        //         output_struct.projectiles_per_shot = Some(projectiles_per_shot);
        //     }
        //
        //     if let Some(spread_range) = specification.spread_range {
        //         output_struct.spread_range = Some(spread_range);
        //     }
        //
        //     if let Some(barrel_offset) = specification.barrel_offset {
        //         output_struct.barrel_offset = Some(barrel_offset);
        //     }
        //
        //     if let Some(sight_height_offset) = specification.sight_height_offset {
        //         output_struct.sight_height_offset = Some(sight_height_offset);
        //     }
        // }
        //
        // // stance 信息
        // if let Some(stance_vec) = de.stance {
        //     for stance_item in stance_vec {
        //         match stance_item.state_key.as_str() {
        //             "running" => {
        //                 output_struct.running_accuracy = Some(stance_item.accuracy);
        //             },
        //             "walking" => {
        //                 output_struct.walking_accuracy = Some(stance_item.accuracy);
        //             },
        //             "crouching" => {
        //                 output_struct.crouching_accuracy = Some(stance_item.accuracy);
        //             },
        //             "crouch_moving" => {
        //                 output_struct.crouch_moving_accuracy = Some(stance_item.accuracy);
        //             },
        //             "standing" => {
        //                 output_struct.standing_accuracy = Some(stance_item.accuracy);
        //             },
        //             "prone"  => {
        //                 output_struct.prone_accuracy = Some(stance_item.accuracy);
        //             },
        //             "prone_moving" => {
        //                 output_struct.prone_moving_accuracy = Some(stance_item.accuracy);
        //             },
        //             "over_wall" => {
        //                 output_struct.over_wall_accuracy = Some(stance_item.accuracy);
        //             },
        //             _ => {
        //                 println!("Not matched state key: {}", stance_item.state_key);
        //             }
        //         }
        //     }
        // }

        // println!("===Parsing file completed===");
        // writer.serialize(output_struct);
    }

    // writer.flush().expect("flush error");

    println!("Task completed, fileName: {}", output_file_name);
}
