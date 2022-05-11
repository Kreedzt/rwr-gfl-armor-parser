extern crate csv;

mod decode;
mod model;
mod parse;

use crate::model::Output;
use armor_parser::export_to_file;
use std::path::PathBuf;
use structopt::StructOpt;

fn main() {
    let opt = model::Opt::from_args();
    let folder_path: PathBuf = opt.input;
    let translation_path: PathBuf = opt.translation;

    export_to_file(folder_path.to_str().unwrap(), translation_path.to_str().unwrap()).unwrap();
}
