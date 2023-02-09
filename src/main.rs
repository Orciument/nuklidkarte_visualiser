#![deny(unsafe_code)]

use std::collections::HashMap;

use nannou::prelude::*;

use crate::draw_legend::{draw_axes, draw_legend};
use crate::mouse_controller::*;
use crate::nuklid::Nuklid;
use crate::nuklid::ZerfallsArt::BetaPlus;
use crate::nuklid_display_engine::draw_card;
use crate::nuklid_json_deserializer::deserialize_ad_to_map;
use crate::print_reaction_equation::draw_reaction;

mod nuklid;
mod nuklid_display_engine;
mod mouse_controller;
mod nuklid_json_deserializer;
mod subsup;
mod datastring;
mod print_reaction_equation;
mod math_vec;
mod draw_legend;

#[macro_export]
macro_rules! unwrap_or_return {
    ($opt:expr) => {
        match $opt {
        Ok(x) => x,
        Err(_) => { return; }
    };
    };
}

fn main() {
    nannou::app(model)
        .run();
}

#[allow(dead_code)]
pub struct Model {
    window: WindowId,
    nuklids: HashMap<u8, HashMap<u8, Nuklid>>,
    old_mouse_pos: (f32, f32),
    translate: (f32, f32),
    square_size: f32,
    selected_nuklid: Option<Nuklid>,
    reaction_chain: Vec<Nuklid>,
}

fn model(app: &App) -> Model {
    let window = app.new_window()
        .view(view)
        .mouse_wheel(mouse_scroll)
        .mouse_pressed(mouse_clicked)
        .mouse_moved(mouse_moved)
        .build().unwrap();
    Model {
        window,
        nuklids: deserialize_ad_to_map(),
        old_mouse_pos: (app.mouse.x, app.mouse.y),
        translate: (0., 0.),
        square_size: 50.,
        selected_nuklid: None,
        reaction_chain: vec![],
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);
    let draw = translate_origin(&app);
    let view: Draw = translate_to_view(&draw, &model);

    //Draw Nuklids
    nuklid_display_engine::draw_nuklid_map(&view, &model.nuklids, &model.square_size, &model.translate, &app.main_window().inner_size_pixels());
     if let Some(sel) = &model.selected_nuklid {
        draw_reaction(&view, &model.square_size, sel, &model.nuklids, 200);
    }

    draw_axes(&view, &model.square_size, &model.translate, &app.main_window().inner_size_pixels());
    draw_legend(&view, &model.square_size);

    //Draw color Test
    draw_card(&view, -100., -100., &model.square_size, "Tast", BetaPlus.color(), &0.3);
    // draw_card(&draw, 200., 200., &model.square_size, "Test", N.color());
    //End draw color test

    //Debugging stuff
    view.ellipse().w_h(5.0, 5.0).color(HOTPINK).caps_round();
    draw.text(&*("FPS: ".to_owned() + &app.fps().to_string())).x_y(110., 10.).z(100.).color(WHITE).left_justify();

    // put everything on the frame
    view.to_frame(app, &frame).unwrap();
}

pub fn translate_origin(app: &App) -> Draw {
    let window_size = app.main_window().inner_size_points(); //Screen Size (Pixel)
    app.draw().x_y(window_size.0 * -0.5, window_size.1 * -0.5)//Translate Origin
}

pub fn translate_to_view(draw: &Draw, model: &Model) -> Draw {
    draw.x_y(-model.translate.0, -model.translate.1) //Translate View Translation;
}