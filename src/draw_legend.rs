#![deny(unsafe_code)]

use nannou::color::BLACK;
use nannou::prelude::{pt2, WHITE};
use nannou::text::Font;
use nannou::App;
use nannou::Draw;

use crate::model::Model;
use crate::nuklid::Nuklid;
use crate::nuklid::ZerfallsArt::*;
use crate::nuklid_display_engine::draw_card;

pub fn draw_axes(draw: &Draw, square_size: &f32, &translation: &(f32, f32), &window_size: &(u32, u32)) {
    let x_rand = window_size.0 as f32 + translation.0;
    let y_rand = window_size.1 as f32 + translation.1;

    //Y Axes
    draw.line()
        .z(15.)
        .start(pt2(-1., -1.))
        .end(pt2(-1., y_rand))
        .color(WHITE)
        .caps_round();
    //X Axes
    draw.line()
        .z(15.)
        .start(pt2(-1., -1.))
        .end(pt2(x_rand, -1.))
        .color(WHITE)
        .caps_round();

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

        draw.line()
            .z(5.)
            .start(pt2(i as f32 * square_size + square_size * 0.5, -1.))
            .end(pt2(i as f32 * square_size + square_size * 0.5, -10.))
            .color(WHITE);
        draw.text(&*i.to_string())
            .z(5.)
            .x_y(i as f32 * square_size + square_size * 0.5, -15.)
            .color(WHITE)
            .center_justify();
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

        draw.line()
            .z(5.)
            .start(pt2(-1., i as f32 * square_size + square_size * 0.5))
            .end(pt2(-10., i as f32 * square_size + square_size * 0.5))
            .color(WHITE);
        draw.text(&*i.to_string())
            .z(5.)
            .x_y(-15., i as f32 * square_size + square_size * 0.5)
            .color(WHITE)
            .center_justify();
    }

    //Draw variable names on axes

    //Overdraw area next to border
    draw.rect()
        .z(20.)
        .x_y(x_rand, -1.)
        .w_h(60., 50.)
        .color(BLACK);
    draw.rect()
        .z(20.)
        .x_y(-1., y_rand)
        .w_h(50., 60.)
        .color(BLACK);
    //draw character
    draw.text("n")
        .z(20.)
        .x_y(x_rand - 20., -1. + 5.)
        .color(WHITE);
    draw.text("p").z(20.).x_y(-1., y_rand - 20.).color(WHITE);
}

pub fn draw_legend(draw: &Draw, square_size: &f32) {
    let alpha: Nuklid = Nuklid {
        zerfalls_art: Alpha,
        name: "Be".to_string(),
        life: "-".to_string(),
        neutronen: 4,
        protonen: 4,
    };
    let beta_minus: Nuklid = Nuklid {
        zerfalls_art: BetaMinus,
        name: "Li".to_string(),
        life: "-".to_string(),
        neutronen: 3,
        protonen: 5,
    };
    let beta_plus: Nuklid = Nuklid {
        zerfalls_art: BetaPlus,
        name: "O".to_string(),
        life: "-".to_string(),
        neutronen: 8,
        protonen: 7,
    };
    let stable: Nuklid = Nuklid {
        zerfalls_art: Stable,
        name: "He".to_string(),
        life: "-".to_string(),
        neutronen: 2,
        protonen: 2,
    };
    let p: Nuklid = Nuklid {
        zerfalls_art: P,
        name: "Mg".to_string(),
        life: "-".to_string(),
        neutronen: 12,
        protonen: 7,
    };
    let n: Nuklid = Nuklid {
        zerfalls_art: N,
        name: "H".to_string(),
        life: "-".to_string(),
        neutronen: 1,
        protonen: 3,
    };
    let _sf: Nuklid = Nuklid {
        zerfalls_art: SF,
        name: "H".to_string(),
        life: "-".to_string(),
        neutronen: 138,
        protonen: 96,
    };

    let font_data: &[u8] = include_bytes!("./font/bahnschrift.ttf");
    let font: Font = match Font::from_bytes(font_data) {
        Ok(x) => x,
        Err(_) => {
            return;
        }
    };

    let x_s = square_size * 3.5;
    let y_s = -square_size * 0.5 - 30.;
    alpha.draw(draw, square_size, Some((x_s, y_s)));
    draw.text("<- Alpha")
        .x_y(x_s + square_size * 1.5, y_s)
        .center_justify()
        .font(font.clone())
        .font_size((square_size * 0.4) as u32);

    beta_minus.draw(draw, square_size, Some((x_s, y_s - square_size)));
    draw.text("<- Beta-")
        .x_y(x_s + square_size * 1.5, y_s - square_size)
        .center_justify()
        .font(font.clone())
        .font_size((square_size * 0.4) as u32);

    beta_plus.draw(draw, square_size, Some((x_s, y_s - 2. * square_size)));
    draw.text("<- Beta+")
        .x_y(x_s + square_size * 1.5, y_s - 2. * square_size)
        .center_justify()
        .font(font.clone())
        .font_size((square_size * 0.4) as u32);

    stable.draw(draw, square_size, Some((x_s + 3. * square_size, y_s)));
    draw.text("<- Stable")
        .x_y(x_s + square_size * 4.6, y_s)
        .center_justify()
        .font(font.clone())
        .font_size((square_size * 0.4) as u32);

    p.draw(
        draw,
        square_size,
        Some((x_s + 3. * square_size, y_s - square_size)),
    );
    draw.text("<- P (Proton)")
        .x_y(x_s + square_size * 5., y_s - square_size)
        .center_justify()
        .font(font.clone())
        .font_size((square_size * 0.4) as u32);

    n.draw(
        draw,
        square_size,
        Some((x_s + 3. * square_size, y_s - 2. * square_size)),
    );
    draw.text("<- N (Neutron)")
        .x_y(x_s + square_size * 5.2, y_s - 2. * square_size)
        .center_justify()
        .font(font.clone())
        .font_size((square_size * 0.4) as u32);

    draw_card(
        draw,
        15.0 * square_size,
        y_s * 2.0,
        &(square_size * 2.0),
        "Sources\n[Link]",
        Stable.color(),
        &0.2,
    )
}

pub fn clicked_on_sources(app: &App, model: &mut Model) {
    let window_size = app.main_window().inner_size_points();
    // - Translated Origin * Flipped Origin + Translated Display (where the mouse is, while
    // rendering only what is visible is displayed and is shown in the bottom left corner as always
    // so we need to translate
    let corrected_x: f32 = (app.mouse.x - window_size.0 * -0.5) + model.translate.0;
    let corrected_y: f32 = (app.mouse.y - window_size.1 * -0.5) + model.translate.1;

    let sqs = model.square_size;
    //Middle x and y
    let m_y = (-sqs * 0.5 - 30.) * 2.;
    let m_x = 15.0 * sqs;
    //Bounds
    let lower_x = m_x - sqs;
    let upper_x = m_x + sqs;
    let lower_y = m_y - sqs;
    let upper_y = m_y + sqs;

    //Test if in bounds
    if corrected_x > upper_x || corrected_x < lower_x {
        return;
    }
    if corrected_y > upper_y || corrected_y < lower_y {
        return;
    }

    println!("Check the sources in your browser!");
    match open::that("https://github.com/Orciument/nuklidkarte_visualiser#sources") {
        Ok(_) => {
            // eprintln!("success")
        }
        Err(x) => {
            eprintln!("Error: {}", x)
        }
    };
}
