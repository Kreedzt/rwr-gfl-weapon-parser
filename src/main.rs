extern crate csv;

mod decode;
mod model;
mod parse;

use crate::model::{Output, TemplateOutput};
use crate::parse::{
    convert_output_struct_to_map, parse_empty_event, parse_normal_event, parse_translation_empty,
};
use chrono::prelude::*;
use csv::Writer;
use indicatif::{ProgressBar, ProgressStyle};
use quick_xml::events::BytesEnd;
use quick_xml::{
    events::{BytesStart, Event},
    Reader,
};
use regex::Regex;
use std::collections::HashMap;
use std::io::Cursor;
use std::path::PathBuf;
use std::{fs, io};
use structopt::StructOpt;

fn main() {
    let local = Local::now();

    let current_time = local.format("%Y-%m-%d-%H-%M-%S").to_string();

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

    // 准备解析武器的 Vec
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

    // 进度条设置
    let pb_count: u64 = total.try_into().unwrap();
    let pb = ProgressBar::new(pb_count);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
            .progress_chars("##-"),
    );

    let mut all_output_res_vec = vec![];

    // 开始循环文件路径列表解析
    for (index, path) in entries.into_iter().enumerate() {
        let last_path = String::from(path.file_name().unwrap().to_str().unwrap());

        let res_str =
            decode::read_file_decode_to_utf8(&path.into_os_string().into_string().unwrap())
                .unwrap_or("".to_string());

        let mut reader = Reader::from_str(&res_str);
        reader.trim_text(true);
        let mut buf = Vec::new();

        let mut output_weapons_vec: Vec<Output> = vec![];
        let mut output_struct = Output::default();
        output_struct.source_file_name = last_path.clone();

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
                    // println!("text: {}", e.unescape_and_decode(&reader).unwrap());
                }
                Ok(Event::End(ref e)) => {
                    match e.name() {
                        // 若为 weapon 结束标签, 表示已经结束一项 weapon
                        b"weapon" => {
                            output_weapons_vec.push(output_struct);
                            output_struct = Output::default();
                            output_struct.source_file_name = last_path.clone();
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
            if let Some(ref file) = output_item.weapon_template_file {
                let template_file_name_path =
                    format!("{}\\{}", folder_path.display().to_string(), file);
                let res_str = decode::read_file_decode_to_utf8(&template_file_name_path)
                    .unwrap_or("".to_string());

                let mut reader = Reader::from_str(&res_str);
                reader.trim_text(true);
                let mut buf = Vec::new();

                loop {
                    match reader.read_event(&mut buf) {
                        Ok(Event::Start(ref e)) => {
                            parse_normal_event(
                                e,
                                &mut reader,
                                &mut output_item,
                                &mut extra_msg_list,
                            );
                        }
                        // 闭合标签
                        Ok(Event::Empty(ref e)) => {
                            parse_empty_event(
                                e,
                                &mut reader,
                                &mut output_item,
                                &mut extra_msg_list,
                            );
                        }
                        Ok(Event::Text(e)) => {
                            // println!("text: {}", e.unescape_and_decode(&reader).unwrap());
                        }
                        Ok(Event::Eof) => break,
                        Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                        _ => (),
                    }
                }
            }

            if let Some(s_name) = output_item.name.clone() {
                output_item.cn_name = translation_map.get(&s_name).map(|n| n.to_string());

                // println!("===cn_name: {:?} ===", output_item.cn_name);
            }

            all_output_res_vec.push(output_item.clone());
            writer.serialize(output_item.clone()).unwrap();
        }

        // println!("===parse completed===");

        // 更新进度条信息
        pb.set_message(last_path);
        pb.inc(1);
    }

    writer.flush().expect("flush error");

    pb.finish_with_message("done");

    println!("===extra_msg_list===");

    for item in extra_msg_list {
        println!("{}", item);
    }

    println!("===extra_msg_list output completed===");

    println!("translation total text count: {}", translation_map.len());

    println!("Task completed, fileName: {}", output_file_name);

    if let Some(template_path) = opt.template {
        println!("===Detected template, translation task started===");
        let template_content = fs::read_to_string(template_path).expect("can't read template file");

        let re = Regex::new(r"\{\{\w+\}\}").unwrap();

        let mut template_output_vec = vec![];

        println!("all_output_res_vec len: {}", all_output_res_vec.len());

        for weapon in all_output_res_vec {
            let cloned_weapon = weapon.clone();

            // 替换后的内容
            let mut res_str = template_content.clone();

            let field_map = convert_output_struct_to_map(cloned_weapon);

            let mut is_valid_translate = true;

            for cap in re.captures_iter(&template_content) {
                for con in cap.iter() {
                    if let Some(c) = con {
                        let after_slice = &template_content[c.start()..c.end()];

                        let after_replace = after_slice.replace("{", "").replace("}", "");

                        if let Some(map_value) = field_map.get(&*after_replace) {
                            if let Some(v) = map_value {
                                res_str = res_str.replace("{", "").replace("}", "");
                                res_str = res_str.replace(&after_replace, &v).to_owned();
                            } else {
                                is_valid_translate = false;
                            }
                        } else {
                            is_valid_translate = false;
                        }
                    } else {
                        is_valid_translate = false;
                    }
                }
            }

            if let Some(weapon_name) = weapon.name {
                if is_valid_translate {
                    template_output_vec.push(TemplateOutput {
                        key: weapon_name,
                        text: res_str,
                    });
                }
            }
        }

        // 准备写入到文件
        let translation_file_name = format!("translation-output-{}.xml", current_time);
        let mut writer = quick_xml::Writer::new_with_indent(Cursor::new(Vec::new()), b' ', 4);

        let translation_tag = BytesStart::owned(b"translation".to_owned(), "translation".len());
        writer.write_event(Event::Start(translation_tag)).unwrap();

        for output_item in template_output_vec {
            let mut text_tag = BytesStart::owned(b"text".to_owned(), "text".len());

            text_tag.push_attribute(("key", output_item.key.as_str()));
            text_tag.push_attribute(("text", output_item.text.as_str()));

            writer.write_event(Event::Empty(text_tag)).unwrap();
        }

        writer
            .write_event(Event::End(BytesEnd::borrowed(b"translation")))
            .unwrap();

        let result = String::from_utf8(writer.into_inner().into_inner()).unwrap();
        fs::write(translation_file_name.clone(), result).unwrap();

        println!("===Translation task completed===");
        println!("Output fileName: {}", translation_file_name);
    }
}
