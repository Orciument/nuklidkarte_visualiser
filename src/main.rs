#![deny(unsafe_code)]

use nannou::prelude::*;
use crate::data::nuklid_json_deserializer::new_deserialize;
use crate::draw_legend::{draw_axes, draw_legend};
use crate::model::Model;
use crate::mouse_controller::*;
use crate::nuklid_display_engine::draw_nuklid_map;
use crate::print_reaction_equation::draw_reaction;

mod data;
mod draw_legend;
mod math;
mod model;
mod mouse_controller;
mod nuklid;
mod nuklid_display_engine;
mod print_reaction_equation;
mod subsup;

#[macro_export]
macro_rules! unwrap_or_return {
    ($opt:expr) => {
        match $opt {
            Ok(x) => x,
            Err(_) => {
                return;
            }
        };
    };
}

fn main() {
    nannou::app(model).run();
    //TODO make textures out of tiles at startup and render textrue
    //maybe just overlap the once selected
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
        nuklids: new_deserialize(),
        old_mouse_pos: (app.mouse.x, app.mouse.y),
        translate: (0., 0.),
        square_size: 50.,
        reaction_chain: vec![],
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);
    let draw = translate_origin(&app);
    let view: Draw = translate_to_view(&draw, &model);

    //Draw Nuklids
    draw_nuklid_map(&view, &model.nuklids, &model.square_size, &model.translate, &app.main_window().inner_size_pixels());
    draw_reaction(&view, &model.square_size, &model.reaction_chain);

    draw_axes(&view, &model.square_size, &model.translate, &app.main_window().inner_size_pixels());
    draw_legend(&view, &model.square_size);

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