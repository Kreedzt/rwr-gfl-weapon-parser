extern crate csv;

mod decode;
mod model;
mod parse;

use crate::model::{Output};
use crate::parse::{parse_empty_event, parse_normal_event, parse_translation_empty};
use chrono::prelude::*;
use csv::Writer;
use quick_xml::{events::Event, Reader};
use std::collections::HashMap;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::{fs, io, str};
use std::borrow::Borrow;
use std::cell::RefCell;
use structopt::StructOpt;

fn main() {
    let local = Local::now();

    let current_time = local.format("%Y-%m-%d-%H-%M-%S").to_string();

    // let folder_path = r#"D:\SteamLibrary\steamapps\workshop\content\270150\2513537759\media\packages\Girls_FrontLine\weapons"#;

    let opt = model::Opt::from_args();
    let folder_path: PathBuf = opt.input;
    let translation_path: PathBuf = opt.translation;

    println!("translation_path: {}", translation_path.to_str().unwrap());

    println!("current_time: {}", current_time);

    println!("folder_path: {}", folder_path.to_str().unwrap());

    let mut extra_msg_list = Vec::new();

    // 解析中文翻译
    let translation_path = translation_path.to_str().unwrap();
    let res_str = decode::read_file_decode_to_utf8(translation_path).unwrap_or("".to_string());

    let mut reader = Reader::from_str(&res_str);
    reader.trim_text(true);
    let mut buf: Vec<u8> = Vec::new();
    let mut translation_map: HashMap<String, String> = HashMap::new();

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                // holder
            }
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
        let mut output_struct = Output::default();

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
                Ok(Event::Eof) => break,
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
        }

        // 若包含 file， 表明存在模板
        if let Some(ref file) = output_struct.weapon_template_file {
            println!("===found template: {}, ready to parse===", file);
            let template_file_name_path = format!("{}\\{}", folder_path.display().to_string(), file);
            let res_str = decode::read_file_decode_to_utf8(&template_file_name_path).unwrap_or("".to_string());

            let mut reader = Reader::from_str(&res_str);
            reader.trim_text(true);
            let mut buf = Vec::new();

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
                    Ok(Event::Eof) => break,
                    Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                    _ => (),
                }
            }
        }

        if let Some(s_name) = output_struct.name.clone() {
            output_struct.cn_name = translation_map.get(&s_name).map(|n| n.to_string());

            println!("===cn_name: {:?} ===", output_struct.cn_name);
        }

        writer.serialize(output_struct.clone());

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
}
