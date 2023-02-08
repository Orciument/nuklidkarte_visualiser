#![deny(unsafe_code)]

use nannou::color::BLACK;
use nannou::Draw;
use nannou::prelude::{pt2, WHITE};

pub fn draw_axes(draw: &Draw, square_size: &f32, &translation: &(f32, f32), &window_size: &(u32, u32)) {
    let x_rand = window_size.0 as f32 + translation.0;
    let y_rand = window_size.1 as f32 + translation.1;

    //Y Axes
    draw.line().z(15.)
        .start(pt2(-1., -1.))
        .end(pt2(-1., y_rand))
        .color(WHITE)
        .caps_round()
    ;
    //X Axes
    draw.line().z(15.)
        .start(pt2(-1., -1.))
        .end(pt2(x_rand, -1.))
        .color(WHITE)
        .caps_round()
    ;

    //X Axes
    for i in 0..((x_rand / square_size) - 1.) as i32 {
        //Only show every second number if they would be to overlapping
        if square_size < &20.0 && i % 2 != 0 {
            continue;
        }
        //Only show forth second number if they would be to overlapping
        if square_size < &7.0 && i % 4 != 0 {
            continue;
        }

        draw.line().z(5.)
            .start(pt2(i as f32 * square_size + square_size * 0.5, -1.))
            .end(pt2(i as f32 * square_size + square_size * 0.5, -10.))
            .color(WHITE)
        ;
        draw.text(&*i.to_string()).z(5.)
            .x_y(i as f32 * square_size + square_size * 0.5, -15.)
            .color(WHITE)
            .center_justify()
        ;
    }

    //Y Axes
    for i in 0..((y_rand / square_size) - 1.) as i32 {
        //Only show every second number if they would be to overlapping
        if square_size < &20.0 && i % 2 != 0 {
            continue;
        }
        //Only show forth second number if they would be to overlapping
        if square_size < &7.0 && i % 4 != 0 {
            continue;
        }

        draw.line().z(5.)
            .start(pt2(-1., i as f32 * square_size + square_size * 0.5))
            .end(pt2(-10., i as f32 * square_size + square_size * 0.5))
            .color(WHITE)
        ;
        draw.text(&*i.to_string()).z(5.)
            .x_y(-15., i as f32 * square_size + square_size * 0.5)
            .color(WHITE)
            .center_justify()
        ;
    }

    //Draw variable names on axes

    //Overdraw area next to border
    draw.rect().z(20.)
        .x_y(x_rand, -1.)
        .w_h(60., 50.)
        .color(BLACK)
    ;
    draw.rect().z(20.)
        .x_y(-1., y_rand)
        .w_h(50., 60.)
        .color(BLACK)
    ;
    //draw character
    draw.text("p").z(20.)
        .x_y(x_rand - 20., -1. + 5.)
        .color(WHITE)
    ;
    draw.text("n").z(20.)
        .x_y(-1., y_rand - 20.)
        .color(WHITE)
    ;
}