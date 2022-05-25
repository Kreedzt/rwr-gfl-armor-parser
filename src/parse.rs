// SPDX-License-Identifier: GPL-3.0-only
use crate::Output;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use std::collections::HashMap;
use std::str;
use anyhow::{Result, anyhow};

fn parse_carry_item(
    e: &BytesStart,
    reader: &mut Reader<&[u8]>,
    output_struct: &mut Output,
    extra_msg_list: &mut Vec<String>,
) -> Result<()> {
    for attr in e.attributes() {
        let attr_unwrap_res = attr?;
        let attr_value = attr_unwrap_res.unescape_and_decode_value(&reader)?;
        let attr_key = attr_unwrap_res.key;

        match attr_key {
            b"name" => {
                output_struct.name.get_or_insert(attr_value);
            }
            b"key" => {
                output_struct.key.get_or_insert(attr_value);
            }
            b"slot" => {
                output_struct.slot.get_or_insert(attr_value.parse()?);
            }
            b"drop_count_factor_on_death" => {
                output_struct.drop_count_factor_on_death.get_or_insert(attr_value.parse()?);
            }
            b"time_to_live_out_in_the_open" => {
                output_struct.time_to_live_out_in_the_open.get_or_insert(attr_value.parse()?);
            }
            b"player_death_drop_owner_lock_time" => {
                output_struct.player_death_drop_owner_lock_time.get_or_insert(attr_value.parse()?);
            }
            b"transform_on_consume" => {
                output_struct.transform_on_consume.get_or_insert(attr_value);
            }
            _ => {
                let msg = format!(
                    "armor attr: {} / {}",
                    str::from_utf8(attr_key)?,
                    attr_value
                );
                extra_msg_list.push(msg);
                // DEBUG
                // println!("Don't care armor attr: {} {}", str::from_utf8(attr_key).unwrap(), attr_value);
            }
        }
    }

    Ok(())
}

fn parse_hud_icon(
    e: &BytesStart,
    reader: &mut Reader<&[u8]>,
    output_struct: &mut Output,
    _extra_msg_list: &mut Vec<String>,
) -> Result<()> {
    for attr in e.attributes() {
        let attr_unwrap_res = attr?;
        let attr_value = attr_unwrap_res.unescape_and_decode_value(&reader)?;
        let attr_key = attr_unwrap_res.key;

        match attr_key {
            b"filename" => {
                output_struct.hud_icon.get_or_insert(attr_value);
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

    Ok(())
}

fn parse_modifier(
    e: &BytesStart,
    reader: &mut Reader<&[u8]>,
    output_struct: &mut Output,
    _extra_msg_list: &mut Vec<String>,
) -> Result<()> {
    // 记录上一次的 class, 使得下一次的 value 赋值
    let mut prev_class: Option<String> = None;

    for attr in e.attributes() {
        let attr_unwrap_res = attr?;
        let attr_value = attr_unwrap_res.unescape_and_decode_value(&reader)?;
        let attr_key = attr_unwrap_res.key;

        match attr_key {
            b"class" => {
                prev_class = Some(attr_value);
            }
            b"value" => {
                if let Some(class) = prev_class.clone() {
                    match class.as_str() {
                        "speed" => {
                            output_struct.modifier_speed.get_or_insert(attr_value);
                        }
                        "detectability" => {
                            output_struct.modifier_detectability.get_or_insert(attr_value);
                        }
                        "night_detectability" => {
                            output_struct.modifier_night_detectability.get_or_insert(attr_value);
                        }
                        "hit_success_probability" => {
                            output_struct.modifier_hit_success_probability.get_or_insert(attr_value);
                        }
                        _ => {
                            //
                        }
                    }
                }
            }
            _ => {
                //
            }
        }
    }

    Ok(())
}

pub fn parse_normal_event(
    e: &BytesStart,
    reader: &mut Reader<&[u8]>,
    output_struct: &mut Output,
    extra_msg_list: &mut Vec<String>,
) -> Result<()> {
    match e.name() {
        b"carry_item" => {
            parse_carry_item(e, reader, output_struct, extra_msg_list)?;
        }
        _ => {
            // DEBUG
            // println!(
            //     "Don't care tag: {}",
            //     str::from_utf8(e.name()).unwrap(),
            // );
        }
    }

    Ok(())
}

pub fn parse_empty_event(
    e: &BytesStart,
    reader: &mut Reader<&[u8]>,
    output_struct: &mut Output,
    extra_msg_list: &mut Vec<String>,
) -> Result<()> {
    match e.name() {
        b"hud_icon" => {
            parse_hud_icon(e, reader, output_struct, extra_msg_list)?;
        }
        b"commonness" => {
            // TODO
            // println!("TODO: commonness parse");
        }
        b"inventory" => {
            // TODO
            // println!("TODO: inventory parse");
        }
        b"modifier" => {
            parse_modifier(e, reader, output_struct, extra_msg_list)?;
        }
        _ => {
            // DEBUG
            // println!(
            //     "Don't care other tag name: {}",
            //     str::from_utf8(e.name()).unwrap()
            // );
        }
    }

    Ok(())
}

fn parse_translation_text(
    e: &BytesStart,
    reader: &mut Reader<&[u8]>,
    map: &mut HashMap<String, String>,
    _extra_msg_list: &mut Vec<String>
) -> Result<()> {
    let mut prev_text_key = String::new();
    for attr in e.attributes() {
        let attr_unwrap_res = attr?;
        let attr_value = attr_unwrap_res.unescape_and_decode_value(&reader)?;
        let attr_key = attr_unwrap_res.key;

        match attr_key {
            b"key" => {
                prev_text_key = attr_value;
            }
            b"text" => {
                if prev_text_key != "" {
                    map.insert(prev_text_key.clone(), attr_value);
                }
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

    Ok(())
}

pub fn parse_translation_empty(
    e: &BytesStart,
    reader: &mut Reader<&[u8]>,
    map: &mut HashMap<String, String>,
    extra_msg_list: &mut Vec<String>,
) -> Result<()> {
    match e.name() {
        b"text" => {
            parse_translation_text(e, reader, map, extra_msg_list)?;
        }
        _ => {
            // holder
        }
    }

    Ok(())
}
