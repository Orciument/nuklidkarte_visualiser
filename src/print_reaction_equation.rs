use std::any::Any;
use std::collections::HashMap;
use nannou::{App, Draw};
use nannou::color::{BLACK, DIMGRAY, RED};
use nannou::prelude::{pt2, ToPrimitive, WHITE};
use nannou::text::FontSize;
use crate::{Model, subsup, translate_origin, translate_to_view};
use crate::nuklid::{Nuklid, ZerfallsArt};
use crate::nuklid::ZerfallsArt::{SF, Stable, Unknown};
use crate::nuklid_display_engine::BACKGROUND;
use crate::subsup::super_ignore_unable;

pub(crate) fn print_equation(app: &App, model: &Model, nuklid: &Nuklid, lifetime: u8) {
    if lifetime <= 0 { return; }

    //Return if Element is Stable of has no Path
    match nuklid.zerfalls_art {
        SF | Stable | Unknown => { return; }
        _ => {}
    }

    //TODO Find Child Koords
    let child_p_n = ((nuklid.protonen as i16 + nuklid.zerfalls_art.delta_prot() as i16) as u8, (nuklid.neutronen as i16 + nuklid.zerfalls_art.delta_neut() as i16) as u8);

    //TODO Find Child
    let child = match (
        match model.nuklids.get(&child_p_n.0) {
            Some(x) => x,
            None => { return; }
        }
    ).get(&child_p_n.1) {
        Some(x) => x,
        None => { return; }
    };

    //TODO Print Reaction
    //println!("Z: {}", nuklid.zerfalls_art);
    println!("{} -> {}", nuklid, child);

    //TODO Recursion!
    print_equation(app, model, child, lifetime - 1);
    // draw_arrows(app, model, &nuklid, &child_p_n.0, &child_p_n.1);
}

pub fn draw_reaction(draw: &Draw, square_size: &f32, nuklid: &Nuklid, nuklids: &HashMap<u8, HashMap<u8, Nuklid>>, lifetime: u8) {
    if lifetime <= 0 { return; }
    //TODO Find Child Koords
    let child_p_n = ((nuklid.protonen as i16 + nuklid.zerfalls_art.delta_prot() as i16) as u8, (nuklid.neutronen as i16 + nuklid.zerfalls_art.delta_neut() as i16) as u8);

    //TODO Find Child
    let child = match (
        match nuklids.get(&child_p_n.0) {
            Some(x) => x,
            None => { return; }
        }
    ).get(&child_p_n.1) {
        Some(x) => x,
        None => { return; }
    };
    draw_arrows(&draw, square_size, nuklid, &child.neutronen, &child.protonen);
    draw_reaction(draw, square_size, child, nuklids, lifetime-1);
}

fn draw_arrows(draw: &Draw, square_size: &f32, nuklid: &Nuklid, nt: &u8, pt: &u8) {
    //TODO Cache some recalculated values
    //TODO Calc weight based on square_size, so that it sqales with the zoom
    //TODO give the line a black outline
    //TODO give the black outline a white outline, like the cursor

    draw.x_y(square_size*0.5, square_size*0.5)
        .z(30.)
        .line()
        .start(pt2(nuklid.neutronen as f32 * square_size, nuklid.protonen as f32 * square_size))
        .end(pt2(*nt as f32 *square_size, *pt as f32 * square_size))
        .color(BLACK)
        .weight(15.)
    ;

    draw.x_y(square_size*0.5, square_size*0.5)
        .z(30.)

        .line()
        .start(pt2(nuklid.neutronen as f32 * square_size, nuklid.protonen as f32 * square_size))
        .end(pt2(*nt as f32 *square_size, *pt as f32 * square_size))
        .color(nuklid.zerfalls_art.color())
        .weight(5.)
    ;

    // draw.x_y(square_size*0.5, square_size*0.5)
        // .z(30.)
        // .tri().

        // .ellipse()
        // .x_y(nuklid.neutronen as f32 * square_size, nuklid.protonen as f32 * square_size)
        // .color(BLACK)
        // .w_h(5., 5.);
    let inner_square_w_h = square_size * 0.5;
    draw.x_y(square_size*0.5, square_size*0.5)
        .ellipse()
        .w_h(inner_square_w_h, inner_square_w_h)
        .x_y(nuklid.neutronen as f32 * square_size, nuklid.protonen as f32 * square_size)
        .z(40.)
        .color(BACKGROUND);


    let super_string = super_ignore_unable((nuklid.neutronen as u16 + nuklid.protonen as u16).to_string());
    let name = super_string + &*nuklid.name;

    draw.x_y(square_size*0.5, square_size*0.5)
        .text(&*name)
        .x_y(nuklid.neutronen as f32 * square_size, nuklid.protonen as f32 * square_size)
        .center_justify()
        .font_size((square_size / 3.) as FontSize)
        .color(WHITE);
}