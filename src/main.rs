#![deny(unsafe_code)]

mod nuklid;
mod nuklid_display_engine;
mod mouse_controller;
mod nuklid_json_deserializer;
mod subsup;
mod datastring;
mod print_reaction_equation;

use std::collections::HashMap;
use nannou::prelude::*;
use crate::nuklid::{Nuklid};
use crate::nuklid_json_deserializer::{deserialize_ad_to_map};
use crate::print_reaction_equation::draw_reaction;

//TODO Add Arrows to next Element
//TODO Write Reaction Equation to Command Line

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

#[allow(dead_code)]
pub struct Model {
    window: WindowId,
    nuklids: HashMap<u8, HashMap<u8, Nuklid>>,
    old_mouse_pos: (f32, f32),
    translate: (f32, f32),
    square_size: f32,
    selected_nuklid: (u8, u8),
}

fn model(app: &App) -> Model {
    let window = app.new_window()
        .view(view)
        .mouse_wheel(mouse_controller::scroll_scale_viewport)
        .mouse_pressed(mouse_controller::find_hovered_element)
        .build().unwrap();
    Model {
        window,
        nuklids: deserialize_ad_to_map(),
        old_mouse_pos: (app.mouse.x, app.mouse.y),
        translate: (0., 0.),
        square_size: 50.,
        selected_nuklid: (0, 0),
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    //TODO Eliminate Update and change these functions to be base on Events
    // mouse_controller::find_hovered_element(&_app, _model);
    mouse_controller::drag_viewport(&_app, _model);
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);
    let draw = translate_origin(&app);
    let view: Draw = translate_to_view(&draw, &model);

    //Draw Nuklids
    nuklid_display_engine::draw_nuklid_map(&view, &model.nuklids, model.square_size, model.translate, app.main_window().inner_size_pixels());
    if model.selected_nuklid != (0,0) {
        // draw.ellipse().x_y(500.,500.).w_h(50.,50.);
        // draw.line().x_y(500.,500.).end(pt2(200., 200.)).weight(10.).color(WHITE);
        let child = match (
            match model.nuklids.get(&model.selected_nuklid.1) {
                Some(x) => x,
                None => { return; }
            }
        ).get(&model.selected_nuklid.0) {
            Some(x) => x,
            None => { return; }
        };
        draw_reaction(&view, &model.square_size, child, &model.nuklids, 200);
    }


    //Draw color Test
    // draw_card(&draw, 100., 100., _model.square_size, "Tast", BetaPlus.color());
    // draw_card(&draw, 200., 200., _model.square_size, "Test", N.color());
    //End draw color test

    //Debugging stuff
    view.ellipse().w_h(5.0, 5.0).color(HOTPINK).caps_round();
    draw.text(&*("FPS: ".to_owned() + &app.fps().to_string())).x_y(110., 10.).color(WHITE).left_justify();

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