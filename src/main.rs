extern crate csv;

mod decode;
mod model;

use crate::model::{Output, Weapon};
use chrono::prelude::*;
use csv::Writer;
use quick_xml::{events::Event, Reader};
use serde::__private::de::IdentifierDeserializer;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::{fs, io, str};
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
    let mut writer = Writer::from_path(&output_file_name).expect("can't output file");

    let total = entries.len();

    for (index, path) in entries.into_iter().enumerate() {
        println!("process: {} / {}", index + 1, total);
        println!(
            "===Starting parsing file: {}===",
            path.display().to_string()
        );

        let res_str =
            decode::read_file_decode_to_utf8(&path.into_os_string().into_string().unwrap());

        let mut reader = Reader::from_str(&res_str);
        reader.trim_text(true);

        let mut buf = Vec::new();

        let mut weapon_template_file_name: Option<String> = None;
        let mut output_struct = Output::default();

        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    match e.name() {
                        b"weapon" => {
                            for attr in e.attributes() {
                                let attr_unwrap_res = attr.unwrap();
                                let attr_value =
                                    attr_unwrap_res.unescape_and_decode_value(&reader).unwrap();
                                let attr_key = attr_unwrap_res.key;

                                match attr_key {
                                    b"file" => {
                                        weapon_template_file_name = Some(attr_value);
                                    }
                                    b"key" => {
                                        output_struct.key = Some(attr_value);
                                    }
                                    b"on_ground_up" => {
                                        // TODO
                                    }
                                    b"drop_count_factor_on_death" => {
                                        output_struct.drop_count_factor_on_player_death =
                                            Some(attr_value.parse().unwrap());
                                    }
                                    b"drop_count_factor_on_player_death" => {
                                        output_struct.drop_count_factor_on_player_death =
                                            Some(attr_value.parse().unwrap());
                                    }
                                    b"time_to_live_out_in_the_open" => {
                                        output_struct.time_to_live_out_in_the_open =
                                            Some(attr_value.parse().unwrap());
                                    }
                                    b"player_death_drop_owner_lock_time" => {
                                        output_struct.player_death_drop_owner_lock_time =
                                            Some(attr_value.parse().unwrap());
                                    }
                                    _ => {
                                        // DEBUG
                                        // println!("Don't care weapon attr: {} {}", str::from_utf8(attr_key).unwrap(), attr_value);
                                    }
                                }
                            }
                        }
                        _ => {
                            // DEBUG
                            // println!(
                            //     "Don't care tag: {}",
                            //     str::from_utf8(e.name()).unwrap(),
                            // );
                        }
                    }
                }
                // 闭合标签
                Ok(Event::Empty(ref e)) => {
                    match e.name() {
                        b"tag" => {
                            for attr in e.attributes() {
                                let attr_unwrap_res = attr.unwrap();
                                let attr_value =
                                    attr_unwrap_res.unescape_and_decode_value(&reader).unwrap();
                                let attr_key = attr_unwrap_res.key;

                                match attr_key {
                                    b"name" => {
                                        output_struct.tag = Some(attr_value);
                                    }
                                    _ => {
                                        // DEBUG
                                        // println!(
                                        //     "Don't care tag attr: {} {}",
                                        //     str::from_utf8(attr_key).unwrap(),
                                        //     attr_value
                                        // );
                                    }
                                }
                            }
                        }
                        b"specification" => {
                            for attr in e.attributes() {
                                let attr_unwrap_res = attr.unwrap();
                                let attr_value =
                                    attr_unwrap_res.unescape_and_decode_value(&reader).unwrap();
                                let attr_key = attr_unwrap_res.key;

                                match attr_key {
                                    b"retrigger_time" => {
                                        output_struct.retrigger_time =
                                            Some(attr_value.parse().unwrap());
                                    }
                                    b"last_burst_retrigger_time" => {
                                        output_struct.last_burst_retrigger_time =
                                            Some(attr_value.parse().unwrap());
                                    }
                                    b"accuracy_factor" => {
                                        output_struct.accuracy_factor =
                                            Some(attr_value.parse().unwrap());
                                    }
                                    b"spread_range" => {
                                        output_struct.spread_range =
                                            Some(attr_value.parse().unwrap());
                                    }
                                    b"sustained_fire_grow_step" => {
                                        output_struct.sustained_fire_grow_step =
                                            Some(attr_value.parse().unwrap());
                                    }
                                    b"sustained_fire_diminish_rate" => {
                                        output_struct.sustained_fire_diminish_rate =
                                            Some(attr_value.parse().unwrap());
                                    }
                                    b"sight_range_modifier" => {
                                        output_struct.sight_range_modifier =
                                            Some(attr_value.parse().unwrap());
                                    }
                                    b"ai_sight_range_modifier" => {
                                        output_struct.ai_sight_range_modifier =
                                            Some(attr_value.parse().unwrap());
                                    }
                                    b"magazine_size" => {
                                        output_struct.magazine_size =
                                            Some(attr_value.parse().unwrap());
                                    }
                                    b"can_shoot_standing" => {
                                        output_struct.can_shoot_standing =
                                            Some(attr_value.parse().unwrap());
                                    }
                                    b"can_shoot_crouching" => {
                                        output_struct.can_shoot_crouching =
                                            Some(attr_value.parse().unwrap());
                                    }
                                    b"can_shoot_prone" => {
                                        output_struct.can_shoot_prone =
                                            Some(attr_value.parse().unwrap());
                                    }
                                    b"suppressed" => {
                                        output_struct.suppressed =
                                            Some(attr_value.parse().unwrap());
                                    }
                                    b"stab_enabled" => {
                                        output_struct.stab_enabled =
                                            Some(attr_value.parse().unwrap());
                                    }
                                    b"stab_range" => {
                                        output_struct.stab_range =
                                            Some(attr_value.parse().unwrap());
                                    }
                                    b"reload_one_at_a_time" => {
                                        output_struct.reload_one_at_a_time =
                                            Some(attr_value.parse().unwrap());
                                    }
                                    b"name" => {
                                        output_struct.name = Some(attr_value);
                                    }
                                    b"class" => {
                                        output_struct.class = Some(attr_value.parse().unwrap())
                                    }
                                    b"projectile_speed" => {
                                        output_struct.projectile_speed =
                                            Some(attr_value.parse().unwrap())
                                    }
                                    b"slot" => {
                                        output_struct.slot = Some(attr_value.parse().unwrap());
                                    }
                                    // TODO
                                    b"barrel_offset_3d" => {
                                        println!("TODO barrel_offset_3d");
                                    }
                                    b"projectiles_per_shot" => {
                                        output_struct.projectiles_per_shot =
                                            Some(attr_value.parse().unwrap());
                                    }
                                    b"burst_shots" => {
                                        output_struct.burst_shots =
                                            Some(attr_value.parse().unwrap());
                                    }
                                    b"sight_height_offset" => {
                                        output_struct.sight_height_offset = Some(attr_value);
                                    }
                                    _ => {
                                        println!(
                                            "Don't care specification attr: {} {}",
                                            str::from_utf8(attr_key).unwrap(),
                                            attr_value
                                        );
                                    }
                                }
                            }
                        }
                        b"model" => {
                            // TODO
                            // println!("TODO: model parse");
                        }
                        b"hud_icon" => {
                            for attr in e.attributes() {
                                let attr_unwrap_res = attr.unwrap();
                                let attr_value =
                                    attr_unwrap_res.unescape_and_decode_value(&reader).unwrap();
                                let attr_key = attr_unwrap_res.key;

                                match attr_key {
                                    b"filename" => {
                                        output_struct.hud_icon = Some(attr_value);
                                    }
                                    _ => {
                                        // DEBUG
                                        // println!(
                                        //     "Don't care hdy_icon attr: {} {}",
                                        //     str::from_utf8(attr_key).unwrap(),
                                        //     attr_value
                                        // );
                                    }
                                }
                            }
                        }
                        b"commonness" => {
                            // TODO
                            // println!("TODO: commonness parse");
                        }
                        b"inventory" => {
                            // TODO
                            // println!("TODO: inventory parse");
                        }
                        b"ballistics" => {
                            // TODO
                            // println!("TODO: ballistics  parse");
                        }
                        b"result" => {
                            // TODO
                            // println!("TODO: result parse");
                        }
                        b"effect" => {
                            // TODO
                            // println!("TODO: effect  parse");
                        }
                        b"sound" => {
                            // TODO
                            // println!("TODO: sound  parse");
                        }
                        b"stance" => {
                            // 记录上一次的 state_key, 使得下一次的 accuracy 赋值
                            let mut prev_state_key: Option<String> = None;

                            for attr in e.attributes() {
                                let attr_unwrap_res = attr.unwrap();
                                let attr_value =
                                    attr_unwrap_res.unescape_and_decode_value(&reader).unwrap();
                                let attr_key = attr_unwrap_res.key;


                                match attr_key {
                                    b"state_key" => {
                                        prev_state_key = Some(attr_value);
                                    }
                                    b"accuracy" => {
                                        if let Some(record_state_key) = prev_state_key.clone() {
                                            match record_state_key.as_str() {
                                                "running" => {
                                                    output_struct.running_accuracy =
                                                        Some(attr_value.parse().unwrap());
                                                }
                                                "walking" => {
                                                    output_struct.walking_accuracy =
                                                        Some(attr_value.parse().unwrap());
                                                }
                                                "crouch_moving" => {
                                                    output_struct.crouch_moving_accuracy =
                                                        Some(attr_value.parse().unwrap());
                                                }
                                                "prone_moving" => {
                                                    output_struct.prone_moving_accuracy =
                                                        Some(attr_value.parse().unwrap());
                                                }
                                                "standing" => {
                                                    output_struct.standing_accuracy =
                                                        Some(attr_value.parse().unwrap());
                                                }
                                                "crouching" => {
                                                    output_struct.crouching_accuracy =
                                                        Some(attr_value.parse().unwrap());
                                                }
                                                "prone" => {
                                                    output_struct.prone_accuracy =
                                                        Some(attr_value.parse().unwrap());
                                                }
                                                "over_wall" => {
                                                    output_struct.over_wall_accuracy =
                                                        Some(attr_value.parse().unwrap());
                                                }
                                                _ => {
                                                    // DEBUG
                                                    // println!("Don't care accuracy: {:?} {}",
                                                    //          prev_state_key,
                                                    //          attr_value);
                                                }
                                            }
                                        }
                                    }
                                    _ => {
                                        // DEBUG
                                        // println!(
                                        //     "Don't care stance attr: {} {}",
                                        //     str::from_utf8(attr_key).unwrap(),
                                        //     attr_value
                                        // );
                                    }
                                }
                            }
                        }
                        b"modifier" => {
                            for attr in e.attributes() {
                                let attr_unwrap_res = attr.unwrap();
                                let attr_value =
                                    attr_unwrap_res.unescape_and_decode_value(&reader).unwrap();
                                let attr_key = attr_unwrap_res.key;

                                match attr_key {
                                    b"speed" => {
                                        output_struct.modifier_speed = Some(attr_value.parse().unwrap());
                                    }
                                    _ => {
                                        // DEBUG
                                        // println!(
                                        //     "Don't care modifier attr: {} {}",
                                        //     str::from_utf8(attr_key).unwrap(),
                                        //     attr_value
                                        // );
                                    }
                                }
                            }
                        }
                        _ => {
                            // DEBUG
                            // println!(
                            //     "Don't care other tag name: {}",
                            //     str::from_utf8(e.name()).unwrap()
                            // );
                        }
                    }
                }
                Ok(Event::Text(e)) => {
                    println!("text: {}", e.unescape_and_decode(&reader).unwrap());
                }
                Ok(Event::Eof) => break,
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
        }
        println!("===parse completed===");

        // 若包含 file， 表明存在模板
        if let Some(file) = weapon_template_file_name {
            // TODO
        }

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

        writer.serialize(output_struct);
    }

    writer.flush().expect("flush error");

    println!("Task completed, fileName: {}", output_file_name);
}
