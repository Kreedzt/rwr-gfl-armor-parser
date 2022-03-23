use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "armor-parser", about = "rwr gf mod armor data parser")]
pub struct Opt {
    /// armor folder full path
    #[structopt(short, long, parse(from_os_str))]
    pub input: PathBuf,
    /// cn translation file full path
    #[structopt(short, long, parse(from_os_str))]
    pub translation: PathBuf
}

// 适用于 CSV 输出的内容
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Output {
    pub source_file_name: String,

    // carry_item content
    pub key: Option<String>,
    pub name: Option<String>,
    pub transform_on_consume: Option<String>,
    pub slot: Option<i8>,
    pub drop_count_factor_on_death: Option<f32>,
    // pub drop_count_factor_on_player_death: Option<f32>,
    pub time_to_live_out_in_the_open: Option<f32>,
    pub player_death_drop_owner_lock_time: Option<f32>,

    // modifier content
    pub modifier_hit_success_probability: Option<String>,
    pub modifier_detectability: Option<String>,
    pub modifier_night_detectability: Option<String>,
    pub modifier_speed: Option<String>,

    pub hud_icon: Option<String>,


    // 猜测: 中文名称
    pub cn_name: Option<String>
}

impl Default for Output {
    fn default() -> Self {
        Output {
            source_file_name: "".to_string(),
            key: None,
            name: None,
            transform_on_consume: None,
            slot: None,
            drop_count_factor_on_death: None,
            time_to_live_out_in_the_open: None,
            player_death_drop_owner_lock_time: None,
            modifier_hit_success_probability: None,
            modifier_detectability: None,
            modifier_night_detectability: None,
            modifier_speed: None,

            hud_icon: None,

            cn_name: None
        }
    }
}
