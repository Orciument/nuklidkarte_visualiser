use std::collections::HashMap;
use nannou::color::{GRAY, WHITE};
use nannou::Draw;
use nannou::prelude::{BLACK, Srgb};
use nannou::text::FontSize;
use crate::nuklid::{Nuklid};
use crate::nuklid::ZerfallsArt::*;
use crate::subsup;

const BACKGROUND: Srgb<u8> = BLACK;
const OUTER_SCALE: f32 = 0.95;
const INNER_SCALE: f32 = 0.82;

pub fn draw_nuklid_map(draw: &Draw, nuklids: &HashMap<i32, HashMap<i32, Nuklid>>, square_size: f32, translation: (f32, f32), window_size: (u32, u32)) {
    // let frame_start: SystemTime = SystemTime::now();

    //Zeichne nur alle Reihen die auch überhaupt auf dem Bildschirm angezeigt werden
    //Mit Y Verschiebung/Constrains
    let x_lower_bound: i32 = (translation.0 / square_size) as i32;
    let x_upper_bound: i32 = x_lower_bound + (window_size.0 as i32 / square_size as i32);
    let y_lower_bound: i32 = (translation.1 / square_size) as i32;
    let y_upper_bound: i32 = y_lower_bound + (window_size.1 as i32 / square_size as i32);

    for i in y_lower_bound..y_upper_bound + 2 {
        let opt_x_achsen_map = nuklids.get(&i);
        if opt_x_achsen_map.is_some() {
            //Mit X Verschiebung/Constrains
            for j in x_lower_bound..x_upper_bound + 2 {
                let opt_nuklid = opt_x_achsen_map.unwrap().get(&j);
                if opt_nuklid.is_some() {
                    let nuklid = opt_nuklid.unwrap();
                    draw_nuklid(draw,
                                nuklid,
                                j as f32 * square_size + (square_size * 0.5),
                                i as f32 * square_size + (square_size * 0.5),
                                square_size,
                    );
                }
            }
        }
        // println!("{} ms", frame_start.elapsed().unwrap().as_millis());
    }
}

fn draw_nuklid(draw: &Draw, nuklid: &Nuklid, x: f32, y: f32, square_size: f32) {
    let super_string = subsup::superscript_non_panic((nuklid.neutronen + nuklid.protonen).to_string());
    let name = super_string + &*nuklid.name;
    let color: Srgb<u8>;

    match nuklid.zerfalls_art {
        Alpha => color = Srgb { red: 254, green: 255, blue: 69, standard: Default::default() },
        BetaPlus => color = Srgb { red: 238, green: 58, blue: 250, standard: Default::default() },
        BetaMinus => color = Srgb { red: 65, green: 106, blue: 249, standard: Default::default() },
        Stable => color = WHITE,
        Unknown => color = GRAY,
    }
    draw_card(draw, x, y, square_size, &name, color)
}

fn draw_card(draw: &Draw, x: f32, y: f32, square_size: f32, text: &str, tile_color: Srgb<u8>) {
    let outer_square_w_h = square_size * OUTER_SCALE;
    draw.quad()
        .w_h(outer_square_w_h, outer_square_w_h)
        .x_y(x, y)
        .color(tile_color);

    let inner_square_w_h = square_size * INNER_SCALE;
    draw.quad()
        .w_h(inner_square_w_h, inner_square_w_h)
        .x_y(x, y)
        .color(BACKGROUND);

    draw.text(text)
        .x_y(x, y)
        .center_justify()
        .font_size((square_size / 3.) as FontSize)
        .color(WHITE);
}
