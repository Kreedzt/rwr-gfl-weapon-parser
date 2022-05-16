use crate::Output;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use std::collections::HashMap;
use std::str;
use anyhow::{Result, anyhow};

fn parse_weapon(
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
            b"file" => {
                output_struct.weapon_template_file.get_or_insert(attr_value);
            }
            b"key" => {
                output_struct.key.get_or_insert(attr_value);
            }
            b"on_ground_up" => {
                output_struct.on_ground_up.get_or_insert(attr_value);
            }
            b"drop_count_factor_on_death" => {
                output_struct
                    .drop_count_factor_on_death
                    .get_or_insert(attr_value.parse()?);
            }
            b"drop_count_factor_on_player_death" => {
                output_struct
                    .drop_count_factor_on_player_death
                    .get_or_insert(attr_value.parse()?);
            }
            b"time_to_live_out_in_the_open" => {
                output_struct
                    .time_to_live_out_in_the_open
                    .get_or_insert(attr_value.parse()?);
            }
            b"player_death_drop_owner_lock_time" => {
                output_struct
                    .player_death_drop_owner_lock_time
                    .get_or_insert(attr_value.parse()?);
            }
            b"quality" => {
                output_struct.quality.get_or_insert(attr_value);
            }
            b"radius" => {
                output_struct
                    .radius
                    .get_or_insert(attr_value.parse()?);
            }
            b"transform_on_consume" => {
                output_struct.transform_on_consume.get_or_insert(attr_value);
            }
            _ => {
                let msg = format!(
                    "weapon attr: {} / {}",
                    str::from_utf8(attr_key)?,
                    attr_value
                );
                extra_msg_list.push(msg);
                // DEBUG
                // println!("Don't care weapon attr: {} {}", str::from_utf8(attr_key).unwrap(), attr_value);
            }
        }
    }

    Ok(())
}

fn parse_tag(
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
            b"name" => {
                output_struct.tag.get_or_insert(attr_value);
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

fn parse_specification(
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
            b"retrigger_time" => {
                output_struct
                    .retrigger_time
                    .get_or_insert(attr_value.parse()?);
            }
            b"last_burst_retrigger_time" => {
                output_struct
                    .last_burst_retrigger_time
                    .get_or_insert(attr_value.parse()?);
            }
            b"accuracy_factor" => {
                output_struct
                    .accuracy_factor
                    .get_or_insert(attr_value.parse()?);
            }
            b"spread_range" => {
                output_struct
                    .spread_range
                    .get_or_insert(attr_value.parse()?);
            }
            b"sustained_fire_grow_step" => {
                output_struct
                    .sustained_fire_grow_step
                    .get_or_insert(attr_value.parse()?);
            }
            b"sustained_fire_diminish_rate" => {
                output_struct
                    .sustained_fire_diminish_rate
                    .get_or_insert(attr_value.parse()?);
            }
            b"sight_range_modifier" => {
                output_struct
                    .sight_range_modifier
                    .get_or_insert(attr_value.parse()?);
            }
            b"ai_sight_range_modifier" => {
                output_struct
                    .ai_sight_range_modifier
                    .get_or_insert(attr_value.parse()?);
            }
            b"magazine_size" => {
                output_struct
                    .magazine_size
                    .get_or_insert(attr_value.parse()?);
            }
            b"can_shoot_standing" => {
                output_struct
                    .can_shoot_standing
                    .get_or_insert(attr_value.parse()?);
            }
            b"can_shoot_crouching" => {
                output_struct
                    .can_shoot_crouching
                    .get_or_insert(attr_value.parse()?);
            }
            b"can_shoot_prone" => {
                output_struct
                    .can_shoot_prone
                    .get_or_insert(attr_value.parse()?);
            }
            b"suppressed" => {
                output_struct
                    .suppressed
                    .get_or_insert(attr_value.parse()?);
            }
            b"stab_enabled" => {
                output_struct
                    .stab_enabled
                    .get_or_insert(attr_value.parse()?);
            }
            b"stab_range" => {
                output_struct
                    .stab_range
                    .get_or_insert(attr_value.parse()?);
            }
            b"reload_one_at_a_time" => {
                output_struct
                    .reload_one_at_a_time
                    .get_or_insert(attr_value.parse()?);
            }
            b"name" => {
                output_struct.name.get_or_insert(attr_value);
            }
            b"class" => {
                output_struct
                    .class
                    .get_or_insert(attr_value.parse()?);
            }
            b"projectile_speed" => {
                output_struct
                    .projectile_speed
                    .get_or_insert(attr_value.parse()?);
            }
            b"slot" => {
                output_struct
                    .slot
                    .get_or_insert(attr_value.parse()?);
            }
            b"barrel_offset_3d" => {
                output_struct.barrel_offset_3d.get_or_insert(attr_value);
            }
            b"projectiles_per_shot" => {
                output_struct
                    .projectiles_per_shot
                    .get_or_insert(attr_value.parse()?);
            }
            b"burst_shots" => {
                output_struct
                    .burst_shots
                    .get_or_insert(attr_value.parse()?);
            }
            b"sight_height_offset" => {
                output_struct.sight_height_offset.get_or_insert(attr_value);
            }
            b"carry_in_two_hands" => {
                output_struct
                    .carry_in_two_hands
                    .get_or_insert(attr_value.parse()?);
            }
            b"barrel_offset" => {
                output_struct
                    .barrel_offset
                    .get_or_insert(attr_value.parse()?);
            }
            b"use_basic_muzzle_smoke_effect" => {
                output_struct
                    .use_basic_muzzle_smoke_effect
                    .get_or_insert(attr_value.parse()?);
            }
            b"spawn_instance_class" => {
                output_struct.spawn_instance_class.get_or_insert(attr_value);
            }
            b"spawn_instance_key" => {
                output_struct.spawn_instance_key.get_or_insert(attr_value);
            }
            b"consume" => {
                output_struct
                    .consume
                    .get_or_insert(attr_value.parse()?);
            }
            b"deployment" => {
                output_struct
                    .deployment
                    .get_or_insert(attr_value.parse()?);
            }
            b"stance_accuracy_rate" => {
                output_struct
                    .stance_accuracy_rate
                    .get_or_insert(attr_value.parse()?);
            }
            b"barrel_offset_ed" => {
                output_struct.barrel_offset_ed.get_or_insert(attr_value);
            }
            b"success_probability" => {
                output_struct
                    .success_probability
                    .get_or_insert(attr_value.parse()?);
            }
            b"range" => {
                output_struct
                    .range
                    .get_or_insert(attr_value.parse()?);
            }
            b"character_state" => {
                output_struct.character_state.get_or_insert(attr_value);
            }
            b"cover_deployment" => {
                output_struct
                    .cover_deployment
                    .get_or_insert(attr_value.parse()?);
            }
            b"affect_characters" => {
                output_struct
                    .affect_characters
                    .get_or_insert(attr_value.parse()?);
            }
            b"affect_vehicles" => {
                output_struct
                    .affect_vehicles
                    .get_or_insert(attr_value.parse()?);
            }
            b"damage" => {
                output_struct
                    .damage
                    .get_or_insert(attr_value.parse()?);
            }
            b"untransform_equipment_class" => {
                output_struct
                    .untransform_equipment_class
                    .get_or_insert(attr_value);
            }
            b"untransform_count" => {
                output_struct
                    .untransform_count
                    .get_or_insert(attr_value.parse()?);
            }
            b"solt" => {
                output_struct
                    .solt
                    .get_or_insert(attr_value.parse()?);
            }
            _ => {
                let msg = format!(
                    "specification attr: {} / {}",
                    str::from_utf8(attr_key)?,
                    attr_value
                );
                extra_msg_list.push(msg);
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

fn parse_stance(
    e: &BytesStart,
    reader: &mut Reader<&[u8]>,
    output_struct: &mut Output,
    _extra_msg_list: &mut Vec<String>,
) -> Result<()> {
    // 记录上一次的 state_key, 使得下一次的 accuracy 赋值
    let mut prev_state_key: Option<String> = None;

    for attr in e.attributes() {
        let attr_unwrap_res = attr?;
        let attr_value = attr_unwrap_res.unescape_and_decode_value(&reader)?;
        let attr_key = attr_unwrap_res.key;

        match attr_key {
            b"state_key" => {
                prev_state_key = Some(attr_value);
            }
            b"accuracy" => {
                if let Some(record_state_key) = prev_state_key.clone() {
                    match record_state_key.as_str() {
                        "running" => {
                            output_struct
                                .running_accuracy
                                .get_or_insert(attr_value.parse()?);
                        }
                        "walking" => {
                            output_struct
                                .walking_accuracy
                                .get_or_insert(attr_value.parse()?);
                        }
                        "crouch_moving" => {
                            output_struct
                                .crouch_moving_accuracy
                                .get_or_insert(attr_value.parse()?);
                        }
                        "prone_moving" => {
                            output_struct
                                .prone_moving_accuracy
                                .get_or_insert(attr_value.parse()?);
                        }
                        "standing" => {
                            output_struct
                                .standing_accuracy
                                .get_or_insert(attr_value.parse()?);
                        }
                        "crouching" => {
                            output_struct
                                .crouching_accuracy
                                .get_or_insert(attr_value.parse()?);
                        }
                        "prone" => {
                            output_struct
                                .prone_accuracy
                                .get_or_insert(attr_value.parse()?);
                        }
                        "over_wall" => {
                            output_struct
                                .over_wall_accuracy
                                .get_or_insert(attr_value.parse()?);
                        }
                        _ => {
                            // DEBUG
                            // println!("Don't care accuracy: {:?} {}",
                            //          prev_state_key,
                            //          attr_value);
                        }
                    }
                }
            }
            _ => {
                // DEBUG
                // println!(
                //     "Don't care stance attr: {} {}",
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
    extra_msg_list: &mut Vec<String>,
) -> Result<()> {
    // 记录上一次的 class, 使得下一次的 accuracy 赋值
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
                if let Some(modifier_class_key) = prev_class.clone() {
                    match modifier_class_key.as_str() {
                        "speed" => {
                            output_struct.modifier_speed.get_or_insert(attr_value);
                        }
                        _ => {
                            let msg =
                                format!("modifier value extra class: {}", modifier_class_key,);
                            extra_msg_list.push(msg);
                        }
                    }
                }
            }
            _ => {
                let msg = format!(
                    "modifier attr: {} / {}",
                    str::from_utf8(attr_key)?,
                    attr_value
                );
                extra_msg_list.push(msg);
            }
        }
    }

    Ok(())
}

pub fn parse_result(
    e: &BytesStart,
    reader: &mut Reader<&[u8]>,
    output_struct: &mut Output,
    extra_msg_list: &mut Vec<String>,
) -> Result<()> {
    let mut prev_class: Option<String> = None;

    for attr in e.attributes() {
        let attr_unwrap_res = attr?;
        let attr_value = attr_unwrap_res.unescape_and_decode_value(&reader)?;
        let attr_key = attr_unwrap_res.key;

        match attr_key {
            b"class" => {
                prev_class = Some(attr_value);
            }
            b"kill_probability" => {
                if let Some(class) = prev_class.clone() {
                    match class.as_str() {
                        "hit" => {
                            output_struct
                                .result_hit_kill_probability
                                .get_or_insert(attr_value.parse()?);
                        }
                        _ => {
                            let msg = format!("result kill_probability extra class: {}", class);
                            extra_msg_list.push(msg);
                        }
                    }
                }
            }
            b"kill_decay_start_time" => {
                if let Some(class) = prev_class.clone() {
                    match class.as_str() {
                        "hit" => {
                            output_struct
                                .result_hit_kill_decay_start_time
                                .get_or_insert(attr_value.parse()?);
                        }
                        _ => {
                            let msg =
                                format!("result kill_decay_start_time extra class: {}", class);
                            extra_msg_list.push(msg);
                        }
                    }
                }
            }
            b"kill_decay_end_time" => {
                if let Some(class) = prev_class.clone() {
                    match class.as_str() {
                        "hit" => {
                            output_struct
                                .result_hit_kill_decay_end_time
                                .get_or_insert(attr_value.parse()?);
                        }
                        _ => {
                            let msg = format!("result kill_decay_end_time extra class: {}", class);
                            extra_msg_list.push(msg);
                        }
                    }
                }
            }
            _ => {
                let msg = format!(
                    "result attr: {} / {}",
                    str::from_utf8(attr_key)?,
                    attr_value
                );

                extra_msg_list.push(msg);
            }
        }
    }

    Ok(())
}

pub fn parse_next_in_chain(
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
            b"key" => {
                output_struct.next_in_chain_key.get_or_insert(attr_value);
            }
            b"share_ammo" => {
                output_struct
                    .next_in_chain_share_ammo
                    .get_or_insert(attr_value.parse()?);
            }
            _ => {
                let msg = format!(
                    "next_in_chain extra attr: {} / {}",
                    str::from_utf8(attr_key)?,
                    attr_value
                );

                extra_msg_list.push(msg);
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
        b"weapon" => {
            parse_weapon(e, reader, output_struct, extra_msg_list);
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
        b"tag" => {
            parse_tag(e, reader, output_struct, extra_msg_list);
        }
        b"specification" => {
            parse_specification(e, reader, output_struct, extra_msg_list);
        }
        b"model" => {
            // TODO
            // println!("TODO: model parse");
        }
        b"hud_icon" => {
            parse_hud_icon(e, reader, output_struct, extra_msg_list);
        }
        b"commonness" => {
            // TODO
            // println!("TODO: commonness parse");
        }
        b"inventory" => {
            // TODO
            // println!("TODO: inventory parse");
        }
        b"ballistics" => {
            // TODO
            // println!("TODO: ballistics  parse");
        }
        b"result" => {
            parse_result(e, reader, output_struct, extra_msg_list);
        }
        b"effect" => {
            // TODO
            // println!("TODO: effect  parse");
        }
        b"sound" => {
            // TODO
            // println!("TODO: sound  parse");
        }
        b"stance" => {
            parse_stance(e, reader, output_struct, extra_msg_list);
        }
        b"modifier" => {
            parse_modifier(e, reader, output_struct, extra_msg_list);
        }
        b"next_in_chain" => {
            parse_next_in_chain(e, reader, output_struct, extra_msg_list);
        }
        _ => {
            // DEBUG
            // println!(
            //     "Don't care other tag name: {}",
            //     str::from_utf8(e.name()).unwrap()
            // );
        }
    };

    Ok(())
}

// pub fn parse_event_item() {}

fn parse_translation_text(
    e: &BytesStart,
    reader: &mut Reader<&[u8]>,
    map: &mut HashMap<String, String>,
    _extra_msg_list: &mut Vec<String>,
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
            parse_translation_text(e, reader, map, extra_msg_list);
        }
        _ => {
            // holder
        }
    }

    Ok(())
}

pub fn convert_output_struct_to_map(weapon: Output) -> HashMap<&'static str, Option<String>> {
    let mut field_map: HashMap<&str, Option<String>> = HashMap::new();

    // 映射所有可见属性
    field_map.insert("source_file_name", Some(weapon.source_file_name));
    field_map.insert("weapon_template_file", weapon.weapon_template_file);
    field_map.insert("radius", weapon.radius.map(|radius| radius.to_string()));
    field_map.insert("transform_on_consume", weapon.transform_on_consume);
    field_map.insert("on_ground_up", weapon.on_ground_up);
    field_map.insert("hud_icon", weapon.hud_icon);
    field_map.insert("tag", weapon.tag);
    field_map.insert(
        "drop_count_factor_on_death",
        weapon
            .drop_count_factor_on_death
            .map(|count| count.to_string()),
    );
    field_map.insert(
        "drop_count_factor_on_player_death",
        weapon
            .drop_count_factor_on_player_death
            .map(|count| count.to_string()),
    );
    field_map.insert(
        "time_to_live_out_in_the_open",
        weapon
            .time_to_live_out_in_the_open
            .map(|time| time.to_string()),
    );
    field_map.insert(
        "player_death_drop_owner_lock_time",
        weapon
            .player_death_drop_owner_lock_time
            .map(|time| time.to_string()),
    );
    field_map.insert("quality", weapon.quality);
    field_map.insert(
        "carry_in_two_hands",
        weapon.carry_in_two_hands.map(|c| c.to_string()),
    );
    field_map.insert(
        "retrigger_time",
        weapon.retrigger_time.map(|time| time.to_string()),
    );
    field_map.insert(
        "last_burst_retrigger_time",
        weapon
            .last_burst_retrigger_time
            .map(|time| time.to_string()),
    );
    field_map.insert(
        "accuracy_factor",
        weapon.accuracy_factor.map(|a| a.to_string()),
    );
    field_map.insert(
        "sustained_fire_grow_step",
        weapon.sustained_fire_grow_step.map(|s| s.to_string()),
    );
    field_map.insert(
        "sustained_fire_diminish_rate",
        weapon.sustained_fire_diminish_rate.map(|r| r.to_string()),
    );
    field_map.insert("magazine_size", weapon.magazine_size.map(|s| s.to_string()));
    field_map.insert(
        "can_shoot_standing",
        weapon.can_shoot_standing.map(|c| c.to_string()),
    );
    field_map.insert(
        "can_shoot_crouching",
        weapon.can_shoot_crouching.map(|c| c.to_string()),
    );
    field_map.insert(
        "can_shoot_prone",
        weapon.can_shoot_prone.map(|c| c.to_string()),
    );
    field_map.insert("burst_shots", weapon.burst_shots.map(|s| s.to_string()));
    field_map.insert(
        "sight_range_modifier",
        weapon.sight_range_modifier.map(|m| m.to_string()),
    );
    field_map.insert(
        "ai_sight_range_modifier",
        weapon.ai_sight_range_modifier.map(|m| m.to_string()),
    );
    field_map.insert("stab_enabled", weapon.stab_enabled.map(|s| s.to_string()));
    field_map.insert("stab_range", weapon.stab_range.map(|r| r.to_string()));
    field_map.insert(
        "reload_one_at_a_time",
        weapon.reload_one_at_a_time.map(|r| r.to_string()),
    );
    field_map.insert("suppressed", weapon.suppressed.map(|s| s.to_string()));
    field_map.insert("name", weapon.name);
    field_map.insert("class", weapon.class.map(|c| c.to_string()));
    field_map.insert("slot", weapon.slot.map(|s| s.to_string()));
    field_map.insert(
        "projectile_speed",
        weapon.projectile_speed.map(|s| s.to_string()),
    );
    field_map.insert(
        "projectiles_per_shot",
        weapon.projectiles_per_shot.map(|s| s.to_string()),
    );
    field_map.insert("spread_range", weapon.spread_range.map(|s| s.to_string()));
    field_map.insert("barrel_offset", weapon.barrel_offset.map(|o| o.to_string()));
    field_map.insert("sight_height_offset", weapon.sight_height_offset);
    field_map.insert(
        "running_accuracy",
        weapon.running_accuracy.map(|a| a.to_string()),
    );
    field_map.insert(
        "walking_accuracy",
        weapon.walking_accuracy.map(|a| a.to_string()),
    );
    field_map.insert(
        "crouch_moving_accuracy",
        weapon.crouch_moving_accuracy.map(|a| a.to_string()),
    );
    field_map.insert(
        "standing_accuracy",
        weapon.standing_accuracy.map(|a| a.to_string()),
    );
    field_map.insert(
        "crouching_accuracy",
        weapon.crouching_accuracy.map(|a| a.to_string()),
    );
    field_map.insert(
        "prone_accuracy",
        weapon.prone_accuracy.map(|a| a.to_string()),
    );
    field_map.insert(
        "prone_moving_accuracy",
        weapon.prone_moving_accuracy.map(|a| a.to_string()),
    );
    field_map.insert(
        "over_wall_accuracy",
        weapon.over_wall_accuracy.map(|a| a.to_string()),
    );
    field_map.insert("next_in_chain_key", weapon.next_in_chain_key);
    field_map.insert(
        "next_in_chain_share_ammo",
        weapon.next_in_chain_share_ammo.map(|s| s.to_string()),
    );
    field_map.insert("modifier_speed", weapon.modifier_speed);
    field_map.insert(
        "result_hit_kill_probability",
        weapon.result_hit_kill_probability.map(|p| p.to_string()),
    );
    field_map.insert(
        "result_hit_kill_decay_start_time",
        weapon
            .result_hit_kill_decay_start_time
            .map(|t| t.to_string()),
    );
    field_map.insert(
        "result_hit_kill_decay_end_time",
        weapon.result_hit_kill_decay_end_time.map(|t| t.to_string()),
    );
    field_map.insert(
        "use_basic_muzzle_smoke_effect",
        weapon.use_basic_muzzle_smoke_effect.map(|e| e.to_string()),
    );
    field_map.insert("spawn_instance_class", weapon.spawn_instance_class);
    field_map.insert("spawn_instance_key", weapon.spawn_instance_key);
    field_map.insert("consume", weapon.consume.map(|c| c.to_string()));
    field_map.insert("deployment", weapon.deployment.map(|d| d.to_string()));
    field_map.insert(
        "stance_accuracy_rate",
        weapon.stance_accuracy_rate.map(|r| r.to_string()),
    );
    field_map.insert("barrel_offset_ed", weapon.barrel_offset_ed);
    field_map.insert(
        "success_probability",
        weapon.success_probability.map(|p| p.to_string()),
    );
    field_map.insert("range", weapon.range.map(|r| r.to_string()));
    field_map.insert("character_state", weapon.character_state);
    field_map.insert(
        "cover_deployment",
        weapon.cover_deployment.map(|c| c.to_string()),
    );
    field_map.insert(
        "affect_characters",
        weapon.affect_characters.map(|a| a.to_string()),
    );
    field_map.insert(
        "affect_vehicles",
        weapon.affect_vehicles.map(|a| a.to_string()),
    );
    field_map.insert("damage", weapon.damage.map(|d| d.to_string()));
    field_map.insert(
        "untransform_equipment_class",
        weapon.untransform_equipment_class,
    );
    field_map.insert(
        "untransform_count",
        weapon.untransform_count.map(|c| c.to_string()),
    );
    field_map.insert("solt", weapon.solt.map(|s| s.to_string()));
    field_map.insert("barrel_offset_3d", weapon.barrel_offset_3d);
    field_map.insert("cn_name", weapon.cn_name);

    field_map
}
