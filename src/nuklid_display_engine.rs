#![deny(unsafe_code)]

use nannou::Draw;
use nannou::prelude::{BLACK, Srgb, ToPrimitive, WHITE};
use nannou::text::FontSize;

use crate::nuklid::Nuklid;

pub const BACKGROUND: Srgb<u8> = BLACK;
pub const OUTER_SCALE: f32 = 0.95;
pub const INNER_SCALE: f32 = 0.82;

pub fn draw_nuklid_map(draw: &Draw, nuklids: &Vec<(u8, Vec<Option<Nuklid>>)>, &square_size: &f32,
                       translation: &(f32, f32), &window_size: &(u32, u32), ) {

    //Zeichne nur alle Reihen die auch überhaupt auf dem Bildschirm angezeigt werden
    //Mit Y Verschiebung/Constrains
    let x_lower_bound: u8 = (translation.0 / square_size) as u8;
    let x_upper_bound: u8 = (x_lower_bound as u32 + 2u32 + (window_size.0 / square_size as u32))
        .to_u8()
        .unwrap_or(u8::MAX);
    let y_lower_bound: u8 = (translation.1 / square_size) as u8;
    let y_upper_bound: u8 = (y_lower_bound as u32 + 2u32 + (window_size.1 / square_size as u32))
        .to_u8()
        .unwrap_or(u8::MAX);

    for i in y_lower_bound..y_upper_bound {
        let x_vec = match nuklids.get(i as usize) {
            None => continue,
            Some(x) => x,
        };
        //Mit X Verschiebung/Constrains
        for j in x_lower_bound..x_upper_bound {
            let index = j as i16 - x_vec.0 as i16;
            if index < 0 { continue; }
            let nuklid = match x_vec.1.get((index) as usize) {
                Some(Some(x)) => x,
                Some(None) => continue,
                None => {
                    continue;
                }
            };
            nuklid.draw(draw, &square_size, None);
        }
    }
}

pub fn draw_card(
    draw: &Draw,
    x: f32,
    y: f32,
    square_size: &f32,
    text: &str,
    tile_color: Srgb<u8>,
    font_size_fac: &f32,
) {
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
