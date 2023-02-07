#![deny(unsafe_code)]
#![allow(unused)]

use std::collections::HashMap;
use nannou::App;
use nannou::event::{MouseButton, MouseScrollDelta, TouchPhase};
use nannou::event::MouseScrollDelta::LineDelta;
use nannou::window::MousePressedFn;
use crate::{Model, print_reaction_equation};
use crate::nuklid::{Nuklid};
use crate::subsup::super_ignore_unable;

pub fn find_hovered_element(app: &App, model: &mut Model, button: MouseButton) {
    if !app.mouse.buttons.left().is_down() {
        return;
    }

    let nuklids: &HashMap<u8, HashMap<u8, Nuklid>> = &model.nuklids;

    let window_size = app.main_window().inner_size_points();
    // - Translated Origin * Flipped Origin + Translated Display (where the mouse is, while
    // rendering only what is visible is displayed and is shown in the bottom left corner as always
    // so we need to translate
    let corrected_x: f32 = (app.mouse.x - window_size.0 * -0.5) + model.translate.0;
    let corrected_y: f32 = (app.mouse.y - window_size.1 * -0.5) + model.translate.1;
    let x_index: u8 = (corrected_x / model.square_size - 0.5).round() as u8;
    let y_index: u8 = (corrected_y / model.square_size - 0.5).round() as u8;

    if model.selected_nuklid == (x_index, y_index) {
        // eprintln!("deselected");
        model.selected_nuklid = (0, 0);
        return;
    }

    //If the Currently Clicked on Nuklide is the same as the already selected we deselect it
    model.selected_nuklid = (0, 0);

    //Check if Nuklid exists, and get if it exists
    let x_achse_map = match nuklids.get(&y_index) {
        Some(x) => x,
        None => { return; }
    };

    let nuklid = match x_achse_map.get(&x_index) {
        Some(x) => x,
        None => { return; }
    };

    model.selected_nuklid = (x_index, y_index);

    println!();
    print_reaction_equation::print_equation(app, model, &nuklid, 200);
}

pub fn drag_viewport(app: &App, model: &mut Model) {
    if app.mouse.buttons.right().is_down() {
        let delta_x: f32 = model.old_mouse_pos.0 - app.mouse.x;
        let delta_y: f32 = model.old_mouse_pos.1 - app.mouse.y;
        model.translate.0 += delta_x;
        model.translate.1 += delta_y;
    }
    model.old_mouse_pos.0 = app.mouse.x;
    model.old_mouse_pos.1 = app.mouse.y;
}

pub fn scroll_scale_viewport(app: &App, model: &mut Model, scroll_delta: MouseScrollDelta, touch: TouchPhase) {
    //Change the size of the Nuklid rectangle
    if let LineDelta(_, y) = scroll_delta {
        //Compute new Square size
        let new_size = model.square_size + (y * 2.);
        if new_size <= 0. { return; }
        let old_square_size = model.square_size;
        model.square_size = new_size;

        //Translate that the zoom Point is the Window corner not Koordinate(0.,0.)
        //Factor between the old and new Nuklid Square Size
        let scale_fac = model.square_size / old_square_size;
        //Scale Translation so that the left corner stays where it was
        let new_translate = (model.translate.0 * scale_fac, model.translate.1 * scale_fac);
        model.translate = new_translate;

        //Translate that the Zoom point is the middle of the Window

        let window_size = app.main_window().inner_size_points();
        //Number of Nuklids that fit on the screen bevor size change
        let fits_square_old = (window_size.0 / old_square_size, window_size.1 / old_square_size);
        //Number of Nuklids that fit on the screen after size change
        let fits_square_new = (window_size.0 / model.square_size, window_size.1 / model.square_size);
        //Amount of Nuklids that now do(not) fit
        let delta_fits = (fits_square_old.0 - fits_square_new.0, fits_square_old.1 - fits_square_new.1);
        //Amount of Pixel that we need to translate more than bevor
        let needed_px_change = (model.square_size * delta_fits.0 * 0.5, model.square_size * delta_fits.1 * 0.5);
        //Adjusted Translation
        let new_translate = (model.translate.0 + needed_px_change.0, model.translate.1 + needed_px_change.1);
        model.translate = new_translate;
    }
}