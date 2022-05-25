// SPDX-License-Identifier: GPL-3.0-only
mod decode;
mod model;
mod parse;

use crate::model::Output;
use crate::parse::{parse_empty_event, parse_normal_event, parse_translation_empty};
use chrono::prelude::*;
use csv::Writer;
use quick_xml::{events::Event, Reader};
use std::collections::HashMap;
use std::{fs, io};
use anyhow::{Result, anyhow};

pub fn export_to_file(
    folder_path: &str,
    translation_path: &str
) -> Result<String> {
    let local = Local::now();

    let current_time = local.format("%Y-%m-%d-%H-%M-%S").to_string();

    let mut extra_msg_list = Vec::new();

    // 解析中文翻译
    let res_str = decode::read_file_decode_to_utf8(translation_path).unwrap_or("".to_string());

    let mut reader = Reader::from_str(&res_str);
    reader.trim_text(true);
    let mut buf: Vec<u8> = Vec::new();
    let mut translation_map: HashMap<String, String> = HashMap::new();

    loop {
        match reader.read_event(&mut buf) {
            // Ok(Event::Start(ref e)) => {
            //     // holder
            // }
            // 闭合标签
            Ok(Event::Empty(ref e)) => {
                parse_translation_empty(e, &mut reader, &mut translation_map, &mut extra_msg_list)?;
            }
            Ok(Event::Text(e)) => {
                // holder
                println!("text: {:?}", e.unescape_and_decode(&reader));
            }
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
    }

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

        let file_name_option = path.file_name().unwrap();
        let file_name = String::from(file_name_option.to_str().unwrap());
        println!("===Starting parsing file: {}===", file_name);

        let res_str =
            decode::read_file_decode_to_utf8(&path.into_os_string().into_string().unwrap()).unwrap_or("".to_string());

        let mut reader = Reader::from_str(&res_str);
        reader.trim_text(true);
        let mut buf = Vec::new();

        let mut output_weapons_vec: Vec<Output> = vec![];
        let mut output_struct = Output::default();
        output_struct.source_file_name = file_name.clone();

        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    parse_normal_event(e, &mut reader, &mut output_struct, &mut extra_msg_list).map_err(|err| {
                        anyhow!("Err in file: {}, parse_normal_event \n {:?}", file_name, err)
                    })?;
                }
                // 闭合标签
                Ok(Event::Empty(ref e)) => {
                    parse_empty_event(e, &mut reader, &mut output_struct, &mut extra_msg_list).map_err(|err| {
                        anyhow!("Err in file: {}, parse_empty_event \n {:?}", file_name, err)
                    })?
                }
                Ok(Event::Text(e)) => {
                    println!("text: {:?}", e.unescape_and_decode(&reader));
                }
                Ok(Event::End(ref e)) => {
                    match e.name() {
                        // 若为 weapon 结束标签, 表示已经结束一项 weapon
                        b"weapon" => {
                            output_weapons_vec.push(output_struct);
                            output_struct = Output::default();
                            output_struct.source_file_name = file_name.clone();
                        }
                        _ => {
                            // holder
                        }
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
        }

        for mut output_item in output_weapons_vec {
            // 若包含 file， 表明存在模板
            if let Some(file) = output_item.weapon_template_file.clone() {
                println!("===found template: {}, ready to parse===", file);
                let template_file_name_path = format!("{}\\{}", folder_path, file);
                let res_str = decode::read_file_decode_to_utf8(&template_file_name_path).map_err(|err| {
                    anyhow!("Err in path: {}, read_file_decode_to_utf8 \n {:?}", file, err)
                })?;

                let mut reader = Reader::from_str(&res_str);
                reader.trim_text(true);
                let mut buf = Vec::new();

                loop {
                    match reader.read_event(&mut buf) {
                        Ok(Event::Start(ref e)) => {
                            parse_normal_event(e, &mut reader, &mut output_item, &mut extra_msg_list).map_err(|err| {
                                anyhow!("Err in file: {}, parse_normal_event \n {:?}", file, err)
                            })?
                        }
                        // 闭合标签
                        Ok(Event::Empty(ref e)) => {
                            parse_empty_event(e, &mut reader, &mut output_item, &mut extra_msg_list).map_err(|err| {
                                anyhow!("Err in file: {}, parse_empty_event \n {:?}", file, err)
                            })?
                        }
                        Ok(Event::Text(e)) => {
                            println!("text: {:?}", e.unescape_and_decode(&reader));
                        }
                        Ok(Event::Eof) => break,
                        Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                        _ => (),
                    }
                }
            }

            if let Some(s_name) = output_item.name.clone() {
                output_item.cn_name = translation_map.get(&s_name).map(|n| n.to_string());

                println!("===cn_name: {:?} ===", output_item.cn_name);
            }


            writer.serialize(output_item)?;
        }

        println!("===parse completed===");
    }

    writer.flush()?;

    println!("===extra_msg_list===");

    for item in extra_msg_list {
        println!("{}", item);
    }

    println!("===extra_msg_list output completed===");

    println!("translation total text count: {}", translation_map.len());

    println!("Task completed, fileName: {}", output_file_name);

    Ok(output_file_name)
}
