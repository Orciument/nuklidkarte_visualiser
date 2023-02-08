#![deny(unsafe_code)]

use std::collections::HashMap;

use nannou::Draw;
use nannou::prelude::{BLACK, Srgb, ToPrimitive, WHITE};
use nannou::text::FontSize;

use crate::nuklid::Nuklid;
use crate::subsup;

pub const BACKGROUND: Srgb<u8> = BLACK;
pub const OUTER_SCALE: f32 = 0.95;
pub const INNER_SCALE: f32 = 0.82;

pub fn draw_nuklid_map(draw: &Draw, nuklids: &HashMap<u8, HashMap<u8, Nuklid>>, &square_size: &f32, translation: &(f32, f32), &window_size: &(u32, u32)) {
    //Zeichne nur alle Reihen die auch überhaupt auf dem Bildschirm angezeigt werden
    //Mit Y Verschiebung/Constrains
    let x_lower_bound: u8 = (translation.0 / square_size) as u8;
    let x_upper_bound: u8 = (x_lower_bound as u32 + 2u32 + (window_size.0 / square_size as u32)).to_u8().unwrap_or(u8::MAX);
    let y_lower_bound: u8 = (translation.1 / square_size) as u8;
    let y_upper_bound: u8 = (y_lower_bound as u32 + 2u32 + (window_size.1 / square_size as u32)).to_u8().unwrap_or(u8::MAX);

    for i in y_lower_bound..y_upper_bound {
        // println!("{}", i);
        let x_achsen_map = match nuklids.get(&(*&i)) {
            None => continue,
            Some(x) => x
        };
        //Mit X Verschiebung/Constrains
        for j in x_lower_bound..x_upper_bound {
            let nuklid = match x_achsen_map.get(&j) {
                None => continue,
                Some(x) => x
            };
            draw_nuklid(draw,
                        nuklid,
                        j as f32 * square_size + (square_size * 0.5),
                        i as f32 * square_size + (square_size * 0.5),
                        &square_size,
            );
        }
    }
}

pub fn draw_nuklid(draw: &Draw, nuklid: &Nuklid, x: f32, y: f32, square_size: &f32) {
    let super_string = subsup::super_ignore_unable((nuklid.neutronen as u16 + nuklid.protonen as u16).to_string());
    let name = super_string + &*nuklid.name;
    draw_card(draw, x, y, square_size, &name, nuklid.zerfalls_art.color(), &0.3)
}

pub fn draw_card(draw: &Draw, x: f32, y: f32, square_size: &f32, text: &str, tile_color: Srgb<u8>, font_size_fac: &f32) {
    // draw.quad()
    //     .w_h(square_size, square_size)
    //     .x_y(x,y)
    //     .color(DIMGRAY);

    let outer_square_w_h = square_size * OUTER_SCALE;
    draw.quad()
        .w_h(outer_square_w_h, outer_square_w_h)
        .x_y(x, y)
        .z(10.)
        .color(tile_color);

    let inner_square_w_h = square_size * INNER_SCALE;
    draw.quad()
        .w_h(inner_square_w_h, inner_square_w_h)
        .x_y(x, y)
        .z(20.)
        .color(BACKGROUND);

    draw.text(text)
        .x_y(x, y)
        .center_justify()
        .font_size((square_size * font_size_fac) as FontSize)
        .z(20.)
        .color(WHITE);
}
