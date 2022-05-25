// SPDX-License-Identifier: GPL-3.0-only
extern crate csv;

mod decode;
mod model;
mod parse;

use crate::model::Output;
use chrono::prelude::*;
use rwr_gfl_weapon_parser::export_to_file;
use std::path::PathBuf;
use structopt::StructOpt;

fn main() {
    let local = Local::now();

    let current_time = local.format("%Y-%m-%d-%H-%M-%S").to_string();

    let opt = model::Opt::from_args();
    let folder_path: PathBuf = opt.input;
    let translation_path: PathBuf = opt.translation;

    println!("translation_path: {}", translation_path.to_str().unwrap());

    println!("current_time: {}", current_time);

    let folder_path = folder_path.to_str().unwrap();
    println!("folder_path: {}", folder_path);


    // 解析中文翻译
    let translation_path = translation_path.to_str().unwrap();

    let output = export_to_file(folder_path, translation_path);

    println!("Output fileName: {}", output.unwrap());
}
