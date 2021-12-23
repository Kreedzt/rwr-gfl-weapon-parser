extern crate csv;

mod decode;
mod model;
mod parse;

use crate::model::{Output};
use crate::parse::{parse_empty_event, parse_normal_event};
use chrono::prelude::*;
use csv::Writer;
use quick_xml::{events::Event, Reader};
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

    let mut extra_msg_list = Vec::new();

    let total = entries.len();

    for (index, path) in entries.into_iter().enumerate() {
        println!("process: {} / {}", index + 1, total);
        println!("===Starting parsing file: {}===", path.display());

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

        println!("===parse completed===");

        writer.serialize(output_struct.clone());
    }

    writer.flush().expect("flush error");

    println!("===extra_msg_list===");

    for item in extra_msg_list {
        println!("{}", item);
    }

    println!("===extra_msg_list output completed===");

    println!("Task completed, fileName: {}", output_file_name);
}
