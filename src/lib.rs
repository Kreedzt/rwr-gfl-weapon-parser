mod decode;
mod model;
mod parse;

use crate::model::Output;
use crate::parse::{parse_empty_event, parse_normal_event, parse_translation_empty};
// use crate::decode::read_file_decode_to_utf8;
use chrono::prelude::*;
use csv::Writer;
use quick_xml::{events::Event, Reader};
use std::collections::HashMap;
use std::{fs, io};

pub fn export_to_file(
    folder_path: &str,
    translation_path: &str
) -> Result<String, ()> {
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
                parse_translation_empty(e, &mut reader, &mut translation_map, &mut extra_msg_list);
            }
            Ok(Event::Text(e)) => {
                // holder
                println!("text: {}", e.unescape_and_decode(&reader).unwrap());
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
        // let last_path = path.display().to_string().split("\\").last().unwrap();
        let path_string = path.display().to_string();
        let path_list = path_string.split("\\").collect::<Vec<_>>();

        let last_path = path_list.last().unwrap();
        println!("===Starting parsing file: {}===", last_path);

        let res_str =
            decode::read_file_decode_to_utf8(&path.into_os_string().into_string().unwrap()).unwrap_or("".to_string());

        let mut reader = Reader::from_str(&res_str);
        reader.trim_text(true);
        let mut buf = Vec::new();

        let mut output_weapons_vec: Vec<Output> = vec![];
        let mut output_struct = Output::default();
        output_struct.source_file_name = last_path.to_string();

        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    parse_normal_event(e, &mut reader, &mut output_struct, &mut extra_msg_list);
                }
                // 闭合标签
                Ok(Event::Empty(ref e)) => {
                    parse_empty_event(e, &mut reader, &mut output_struct, &mut extra_msg_list);
                }
                Ok(Event::Text(e)) => {
                    println!("text: {}", e.unescape_and_decode(&reader).unwrap());
                }
                Ok(Event::End(ref e)) => {
                    match e.name() {
                        // 若为 weapon 结束标签, 表示已经结束一项 weapon
                        b"weapon" => {
                            output_weapons_vec.push(output_struct);
                            output_struct = Output::default();
                            output_struct.source_file_name = last_path.to_string();
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

        for mut output_item in output_weapons_vec  {
            // 若包含 file， 表明存在模板
            if let Some(ref file) = output_item.weapon_template_file {
                println!("===found template: {}, ready to parse===", file);
                let template_file_name_path = format!("{}\\{}", folder_path, file);
                let res_str = decode::read_file_decode_to_utf8(&template_file_name_path).unwrap_or("".to_string());

                let mut reader = Reader::from_str(&res_str);
                reader.trim_text(true);
                let mut buf = Vec::new();

                loop {
                    match reader.read_event(&mut buf) {
                        Ok(Event::Start(ref e)) => {
                            parse_normal_event(e, &mut reader, &mut output_item, &mut extra_msg_list);
                        }
                        // 闭合标签
                        Ok(Event::Empty(ref e)) => {
                            parse_empty_event(e, &mut reader, &mut output_item, &mut extra_msg_list);
                        }
                        Ok(Event::Text(e)) => {
                            println!("text: {}", e.unescape_and_decode(&reader).unwrap());
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

            writer.serialize(output_item.clone()).unwrap();
        }

        println!("===parse completed===");
    }

    writer.flush().expect("flush error");

    println!("===extra_msg_list===");

    for item in extra_msg_list {
        println!("{}", item);
    }

    println!("===extra_msg_list output completed===");

    println!("translation total text count: {}", translation_map.len());

    println!("Task completed, fileName: {}", output_file_name);

    Ok(output_file_name)
}
