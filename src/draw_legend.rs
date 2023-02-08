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

pub fn draw_legend(draw: &Draw, square_size: &f32) {
    let alpha: Nuklid = Nuklid {
        zerfalls_art: ZerfallsArt::Alpha,
        name: "Be".to_string(),
        life: "-".to_string(),
        neutronen: 4,
        protonen: 4,
    };
    let beta_minus: Nuklid = Nuklid {
        zerfalls_art: ZerfallsArt::BetaMinus,
        name: "Li".to_string(),
        life: "-".to_string(),
        neutronen: 3,
        protonen: 5,
    };
    let beta_plus: Nuklid = Nuklid {
        zerfalls_art: ZerfallsArt::BetaPlus,
        name: "O".to_string(),
        life: "-".to_string(),
        neutronen: 8,
        protonen: 7,
    };
    let stable: Nuklid = Nuklid {
        zerfalls_art: ZerfallsArt::Stable,
        name: "He".to_string(),
        life: "-".to_string(),
        neutronen: 2,
        protonen: 2,
    };
    let p: Nuklid = Nuklid {
        zerfalls_art: ZerfallsArt::P,
        name: "Mg".to_string(),
        life: "-".to_string(),
        neutronen: 12,
        protonen: 7,
    };
    let n: Nuklid = Nuklid {
        zerfalls_art: ZerfallsArt::N,
        name: "H".to_string(),
        life: "-".to_string(),
        neutronen: 1,
        protonen: 3,
    };
    let sf: Nuklid = Nuklid {
        zerfalls_art: ZerfallsArt::SF,
        name: "H".to_string(),
        life: "-".to_string(),
        neutronen: 138,
        protonen: 96,
    };

    let font_data: &[u8] = include_bytes!("./font/bahnschrift.ttf");
    let font: Font = match Font::from_bytes(font_data) {
        Ok(x) => x,
        Err(_) => { return; }
    };


    let x_s = square_size * 3.5;
    let y_s = -square_size * 0.5 - 30.;
    draw_nuklid(draw, &alpha, x_s, y_s, square_size);
    draw.text("<- Alpha").x_y(x_s + square_size * 1.5, y_s).center_justify().font(font.clone()).font_size((square_size * 0.4) as u32);

    draw_nuklid(draw, &beta_minus, x_s, y_s - square_size, square_size);
    draw.text("<- Beta-").x_y(x_s + square_size * 1.5, y_s - square_size).center_justify().font(font.clone()).font_size((square_size * 0.4) as u32);

    draw_nuklid(draw, &beta_plus, x_s, y_s - 2. * square_size, square_size);
    draw.text("<- Beta+").x_y(x_s + square_size * 1.5, y_s - 2. * square_size).center_justify().font(font.clone()).font_size((square_size * 0.4) as u32);


    draw_nuklid(draw, &stable, x_s + 3. * square_size, y_s, square_size);
    draw.text("<- Stable").x_y(x_s + square_size * 4.6, y_s).center_justify().font(font.clone()).font_size((square_size * 0.4) as u32);

    draw_nuklid(draw, &p, x_s + 3. * square_size, y_s - square_size, square_size);
    draw.text("<- P (Proton)").x_y(x_s + square_size * 5., y_s - square_size).center_justify().font(font.clone()).font_size((square_size * 0.4) as u32);

    draw_nuklid(draw, &n, x_s + 3. * square_size, y_s - 2. * square_size, square_size);
    draw.text("<- N (Neutron)").x_y(x_s + square_size * 5.2, y_s - 2. * square_size).center_justify().font(font.clone()).font_size((square_size * 0.4) as u32);

    draw_card(draw, 15.0 * square_size, y_s * 2.0, &(square_size * 2.0), "Sources\n[Link]", Stable.color(), &0.2)
}