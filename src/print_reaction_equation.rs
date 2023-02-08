use std::collections::HashMap;
use nannou::{App, Draw};
use nannou::color::{BLACK};
use nannou::geom::vec2;
use nannou::prelude::{pt2, WHITE};
use nannou::text::FontSize;
use crate::{Model};
use crate::math_vec::scale_vec2;
use crate::nuklid::Nuklid;
use crate::nuklid::ZerfallsArt::{SF, Stable, Unknown};
use crate::subsup::super_ignore_unable;

pub(crate) fn print_equation(app: &App, model: &Model, nuklid: &Nuklid, lifetime: u8) {
    if lifetime <= 0 { return; }

    //Return if Element is Stable of has no Path
    match nuklid.zerfalls_art {
        SF | Stable | Unknown => { return; }
        _ => {}
    }

    let child_p_n = ((nuklid.protonen as i16 + nuklid.zerfalls_art.delta_prot() as i16) as u8, (nuklid.neutronen as i16 + nuklid.zerfalls_art.delta_neut() as i16) as u8);

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

    //Recursion!
    print_equation(app, model, child, lifetime - 1);
}

pub fn draw_reaction(draw: &Draw, square_size: &f32, nuklid: &Nuklid, nuklids: &HashMap<u8, HashMap<u8, Nuklid>>, lifetime: u8) {
    if lifetime <= 0 { return; }
    //Find Child Koords
    let child_p_n = ((nuklid.protonen as i16 + nuklid.zerfalls_art.delta_prot() as i16) as u8, (nuklid.neutronen as i16 + nuklid.zerfalls_art.delta_neut() as i16) as u8);

    //Find Child
    let child = match (
        match nuklids.get(&child_p_n.0) {
            Some(x) => x,
            None => { return; }
        }
    ).get(&child_p_n.1) {
        Some(x) => x,
        None => { return; }
    };

    overdraw_nuklid(&draw, square_size, nuklid);
    match nuklid.zerfalls_art {
        Stable | Unknown | SF => {}
        _ => {
            draw_arrows(&draw, square_size, nuklid, &child.neutronen, &child.protonen);
        }
    }
    draw_reaction(draw, square_size, child, nuklids, lifetime - 1);
}

fn draw_arrows(draw: &Draw, square_size: &f32, nuklid: &Nuklid, nt: &u8, pt: &u8) {
    //Translated Draw
    let draw_t = draw.x_y(square_size * 0.5, square_size * 0.5).to_owned();

    let p_start = pt2(nuklid.neutronen as f32 * square_size, nuklid.protonen as f32 * square_size);
    let p_end = pt2(*nt as f32 * square_size, *pt as f32 * square_size);

    match nuklid.zerfalls_art {
        Stable | Unknown | SF => {}
        _ => {
            // Draw line
            draw_t.z(30.).line()
                .start(p_start)
                .end(p_end)
                .color(BLACK)
                .weight(square_size * 0.3)
            ;

            //Vector between mid points of parent and child nuclide
            let delta_vec = vec2(p_end.x - p_start.x, p_end.y - p_start.y);
            //length of Vector delta_vec
            let l_delta_vec = f32::sqrt((delta_vec.x * delta_vec.x) + (delta_vec.y * delta_vec.y));
            //normalise Vector
            let n_delta_vec = delta_vec.normalize();
            //distance from the mid of the square to the target radius
            let radius = square_size * 0.25;

            //New target length of the vector between the points
            let l_new_vec = l_delta_vec - 2. * radius;
            //location vector to new start point
            let new_start =  scale_vec2(&n_delta_vec, &radius) + p_start;
            //location vector to the new end point
            let new_end = scale_vec2(&n_delta_vec, &l_new_vec) + new_start;

            //TODO draw unter diesem Pfeil einen anderen Schwarzen, der die Line ersetzt
            // dieser sollte Im head genau so breit sein wir im Body
            // Der Head sollte etwas (1/2 || 1/3 Pfeil dicke) nach hinten verschoben sein
            // sollte auch etwas drüber hinaus gehen, aber unter schrift liegen

            // draw_t.z(30.).arrow()
            //     .start(new_start)
            //     .end(scale_vec2(&n_delta_vec, &(&l_new_vec + square_size * 0.1)) + new_start)
            //     .color(BLACK)
            //     .weight(square_size * 0.3)
            //     .head_width(square_size * 0.15)
            //     .head_length(square_size * 0.5)
            //     .start_cap_round()
            // ;

            // draw_t.z(50.).arrow()
            //     .start(new_start + 0.1)
            //     .end(new_end + 1.1)
            //     .color(WHITE)
            //     .weight(square_size * 0.11)
            //     .head_width(square_size * 0.11)
            //     .head_length(square_size * 0.41)
            //     .caps_round()
            // ;
            draw_t.z(50.).arrow()
                .start(new_start)
                .end(new_end)
                .color(nuklid.zerfalls_art.color())
                .weight(square_size * 0.1)
                .head_width(square_size * 0.1)
                .head_length(square_size * 0.4)
                .caps_round()
            ;
        }
    }
}

fn overdraw_nuklid(draw: &Draw, square_size: &f32, nuklid: &Nuklid) {
    //Translated Draw
    let draw_t = draw.x_y(square_size * 0.5, square_size * 0.5).to_owned();

    let p_nuklid = pt2(nuklid.neutronen as f32 * square_size, nuklid.protonen as f32 * square_size);

    // draw.x_y(square_size*0.5, square_size*0.5)
    // .z(30.)
    // .tri().

    // .ellipse()
    // .x_y(nuklid.neutronen as f32 * square_size, nuklid.protonen as f32 * square_size)
    // .color(BLACK)
    // .w_h(5., 5.);

    //Over Draw the Reacting Nuklids
    draw_t.z(25.).quad()
        .xy(p_nuklid)
        .w_h(*square_size, *square_size)
        .color(BLACK);

    let inner_square_w_h = square_size * 0.9;
    draw_t.z(40.).ellipse()
        .xy(p_nuklid)
        .w_h(*square_size, *square_size)
        .color(nuklid.zerfalls_art.color());

    draw_t.z(40.).ellipse()
        .xy(p_nuklid)
        .w_h(inner_square_w_h, inner_square_w_h)
        .color(BLACK);


    let super_string = super_ignore_unable((nuklid.neutronen as u16 + nuklid.protonen as u16).to_string());
    let name = super_string + &*nuklid.name;
    draw_t.z(40.).text(&*name)
        .xy(p_nuklid)
        .center_justify()
        .font_size((square_size / 3.) as FontSize)
        .color(WHITE);
}