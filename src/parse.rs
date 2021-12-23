use crate::Output;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use std::str;

fn parse_weapon(
    e: &BytesStart,
    reader: &mut Reader<&[u8]>,
    output_struct: &mut Output,
    extra_msg_list: &mut Vec<String>,
) {
    for attr in e.attributes() {
        let attr_unwrap_res = attr.unwrap();
        let attr_value = attr_unwrap_res.unescape_and_decode_value(&reader).unwrap();
        let attr_key = attr_unwrap_res.key;

        match attr_key {
            b"file" => {
                output_struct.weapon_template_file.get_or_insert(attr_value);
            }
            b"key" => {
                output_struct.key.get_or_insert(attr_value);
            }
            b"on_ground_up" => {
                // TODO
            }
            b"drop_count_factor_on_death" => {
                output_struct.drop_count_factor_on_player_death.get_or_insert(attr_value.parse().unwrap());
            }
            b"drop_count_factor_on_player_death" => {
                output_struct.drop_count_factor_on_player_death.get_or_insert(attr_value.parse().unwrap());
            }
            b"time_to_live_out_in_the_open" => {
                output_struct.time_to_live_out_in_the_open.get_or_insert(attr_value.parse().unwrap());
            }
            b"player_death_drop_owner_lock_time" => {
                output_struct.player_death_drop_owner_lock_time.get_or_insert(attr_value.parse().unwrap());
            }
            b"quality" => {
                output_struct.quality.get_or_insert(attr_value);
            }
            _ => {
                let msg = format!(
                    "weapon attr: {} / {}",
                    str::from_utf8(attr_key).unwrap(),
                    attr_value
                );
                extra_msg_list.push(msg);
                // DEBUG
                // println!("Don't care weapon attr: {} {}", str::from_utf8(attr_key).unwrap(), attr_value);
            }
        }
    }
}

fn parse_tag(
    e: &BytesStart,
    reader: &mut Reader<&[u8]>,
    output_struct: &mut Output,
    extra_msg_list: &mut Vec<String>,
) {
    for attr in e.attributes() {
        let attr_unwrap_res = attr.unwrap();
        let attr_value = attr_unwrap_res.unescape_and_decode_value(&reader).unwrap();
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
}

fn parse_specification(
    e: &BytesStart,
    reader: &mut Reader<&[u8]>,
    output_struct: &mut Output,
    extra_msg_list: &mut Vec<String>,
) {
    for attr in e.attributes() {
        let attr_unwrap_res = attr.unwrap();
        let attr_value = attr_unwrap_res.unescape_and_decode_value(&reader).unwrap();
        let attr_key = attr_unwrap_res.key;

        match attr_key {
            b"retrigger_time" => {
                output_struct.retrigger_time.get_or_insert(attr_value.parse().unwrap());
            }
            b"last_burst_retrigger_time" => {
                output_struct.last_burst_retrigger_time.get_or_insert(attr_value.parse().unwrap());
            }
            b"accuracy_factor" => {
                output_struct.accuracy_factor.get_or_insert(attr_value.parse().unwrap());
            }
            b"spread_range" => {
                output_struct.spread_range.get_or_insert(attr_value.parse().unwrap());
            }
            b"sustained_fire_grow_step" => {
                output_struct.sustained_fire_grow_step.get_or_insert(attr_value.parse().unwrap());
            }
            b"sustained_fire_diminish_rate" => {
                output_struct.sustained_fire_diminish_rate.get_or_insert(attr_value.parse().unwrap());
            }
            b"sight_range_modifier" => {
                output_struct.sight_range_modifier.get_or_insert(attr_value.parse().unwrap());
            }
            b"ai_sight_range_modifier" => {
                output_struct.ai_sight_range_modifier.get_or_insert(attr_value.parse().unwrap());
            }
            b"magazine_size" => {
                output_struct.magazine_size.get_or_insert(attr_value.parse().unwrap());
            }
            b"can_shoot_standing" => {
                output_struct.can_shoot_standing.get_or_insert(attr_value.parse().unwrap());
            }
            b"can_shoot_crouching" => {
                output_struct.can_shoot_crouching.get_or_insert(attr_value.parse().unwrap());
            }
            b"can_shoot_prone" => {
                output_struct.can_shoot_prone.get_or_insert(attr_value.parse().unwrap());
            }
            b"suppressed" => {
                output_struct.suppressed.get_or_insert(attr_value.parse().unwrap());
            }
            b"stab_enabled" => {
                output_struct.stab_enabled.get_or_insert(attr_value.parse().unwrap());
            }
            b"stab_range" => {
                output_struct.stab_range.get_or_insert(attr_value.parse().unwrap());
            }
            b"reload_one_at_a_time" => {
                output_struct.reload_one_at_a_time.get_or_insert(attr_value.parse().unwrap());
            }
            b"name" => {
                output_struct.name.get_or_insert(attr_value);
            }
            b"class" => {
                output_struct.class.get_or_insert(attr_value.parse().unwrap());
            },
            b"projectile_speed" => {
                output_struct.projectile_speed.get_or_insert(attr_value.parse().unwrap());
            }
            b"slot" => {
                output_struct.slot.get_or_insert(attr_value.parse().unwrap());
            }
            b"barrel_offset_3d" => {
                // TODO :DEBUG
                // println!("TODO barrel_offset_3d");
            }
            b"projectiles_per_shot" => {
                output_struct.projectiles_per_shot.get_or_insert(attr_value.parse().unwrap());
            }
            b"burst_shots" => {
                output_struct.burst_shots.get_or_insert(attr_value.parse().unwrap());
            }
            b"sight_height_offset" => {
                output_struct.sight_height_offset.get_or_insert(attr_value);
            }
            b"carry_in_two_hands" => {
                output_struct.carry_in_two_hands.get_or_insert(attr_value.parse().unwrap());
            }
            b"barrel_offset" => {
                output_struct.barrel_offset.get_or_insert(attr_value.parse().unwrap());
            }
            _ => {
                let msg = format!(
                    "specification attr: {} / {}",
                    str::from_utf8(attr_key).unwrap(),
                    attr_value
                );
                extra_msg_list.push(msg);
            }
        }
    }
}

fn parse_hud_icon(
    e: &BytesStart,
    reader: &mut Reader<&[u8]>,
    output_struct: &mut Output,
    extra_msg_list: &mut Vec<String>,
) {
    for attr in e.attributes() {
        let attr_unwrap_res = attr.unwrap();
        let attr_value = attr_unwrap_res.unescape_and_decode_value(&reader).unwrap();
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
}

fn parse_stance(
    e: &BytesStart,
    reader: &mut Reader<&[u8]>,
    output_struct: &mut Output,
    extra_msg_list: &mut Vec<String>,
) {
    // 记录上一次的 state_key, 使得下一次的 accuracy 赋值
    let mut prev_state_key: Option<String> = None;

    for attr in e.attributes() {
        let attr_unwrap_res = attr.unwrap();
        let attr_value = attr_unwrap_res.unescape_and_decode_value(&reader).unwrap();
        let attr_key = attr_unwrap_res.key;

        match attr_key {
            b"state_key" => {
                prev_state_key = Some(attr_value);
            }
            b"accuracy" => {
                if let Some(record_state_key) = prev_state_key.clone() {
                    match record_state_key.as_str() {
                        "running" => {
                            output_struct.running_accuracy.get_or_insert(attr_value.parse().unwrap());
                        }
                        "walking" => {
                            output_struct.walking_accuracy.get_or_insert(attr_value.parse().unwrap());
                        }
                        "crouch_moving" => {
                            output_struct.crouch_moving_accuracy.get_or_insert(attr_value.parse().unwrap());
                        }
                        "prone_moving" => {
                            output_struct.prone_moving_accuracy.get_or_insert(attr_value.parse().unwrap());
                        }
                        "standing" => {
                            output_struct.standing_accuracy.get_or_insert(attr_value.parse().unwrap());
                        }
                        "crouching" => {
                            output_struct.crouching_accuracy.get_or_insert(attr_value.parse().unwrap());
                        }
                        "prone" => {
                            output_struct.prone_accuracy.get_or_insert(attr_value.parse().unwrap());
                        }
                        "over_wall" => {
                            output_struct.over_wall_accuracy.get_or_insert(attr_value.parse().unwrap());
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
}

fn parse_modifier(
    e: &BytesStart,
    reader: &mut Reader<&[u8]>,
    output_struct: &mut Output,
    extra_msg_list: &mut Vec<String>,
) {
    for attr in e.attributes() {
        let attr_unwrap_res = attr.unwrap();
        let attr_value = attr_unwrap_res.unescape_and_decode_value(&reader).unwrap();
        let attr_key = attr_unwrap_res.key;

        match attr_key {
            b"speed" => {
                output_struct.modifier_speed.get_or_insert(attr_value.parse().unwrap());
            }
            _ => {
                // DEBUG
                // println!(
                //     "Don't care modifier attr: {} {}",
                //     str::from_utf8(attr_key).unwrap(),
                //     attr_value
                // );
            }
        }
    }
}

pub fn parse_normal_event(
    e: &BytesStart,
    reader: &mut Reader<&[u8]>,
    output_struct: &mut Output,
    extra_msg_list: &mut Vec<String>,
) {
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
}

pub fn parse_empty_event(
    e: &BytesStart,
    reader: &mut Reader<&[u8]>,
    output_struct: &mut Output,
    extra_msg_list: &mut Vec<String>,
) {
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
            // TODO
            // println!("TODO: result parse");
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
        _ => {
            // DEBUG
            // println!(
            //     "Don't care other tag name: {}",
            //     str::from_utf8(e.name()).unwrap()
            // );
        }
    }
}

pub fn parse_event_item() {}
