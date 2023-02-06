use std::collections::HashMap;
use nannou::App;
use crate::card;
use crate::card::QUADSIZE;
use crate::nuklid::Nuklid;

pub fn find_hovered_element(_app: &App, nuklids: &HashMap<i32, HashMap<i32, Nuklid>>) {
    let window_size = _app.main_window().inner_size_points();
    let unit_x: f32 = (_app.mouse.x - window_size.0 * -0.5) ;
    let unit_y: f32 = (_app.mouse.y - window_size.1 * -0.5);
    let card_count_x: i32 = ((unit_x / QUADSIZE as f32).round()-1.) as i32;
    let card_count_y: i32 = ((unit_y / QUADSIZE as f32).round()-1.) as i32;

    if !_app.mouse.buttons.left().is_down() {
        return;
    }
    println!("Mouse: y:{} x:{}", card_count_y, card_count_x);
    // if nuklids.len() <= card_count_x {
    //     return;
    // }
    // println!("{} {}", nuklids[card_count_x].name, unit_x);
    let x_achse_map = nuklids.get(&card_count_y);
    if x_achse_map.is_none() {
        return;
    }

    let nuklid = x_achse_map.unwrap().get(&card_count_x);
    if nuklid.is_none() {
        return;
    }

    let name = feroxide::superscript((nuklid.unwrap().neutronen + nuklid.unwrap().protonen) as u8) + &*nuklid.unwrap().name;
    println!("{}", name);
}