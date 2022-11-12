// SPDX-License-Identifier: GPL-3.0-only
extern crate csv;

mod decode;
mod model;
mod parse;

use crate::model::Output;
use rwr_gfl_armor_parser::export_to_file;
use std::path::PathBuf;
use structopt::StructOpt;

fn main() {
    let opt = model::Opt::from_args();
    let folder_path: PathBuf = opt.input;
    let translation_path: PathBuf = opt.translation;

    let output = export_to_file(folder_path.to_str().unwrap(), translation_path.to_str().unwrap());

    println!("Output fileName: {}", output.unwrap());
}
