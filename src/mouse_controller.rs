#![deny(unsafe_code)]

use std::collections::HashMap;

use nannou::App;
use nannou::event::{MouseButton, MouseScrollDelta, TouchPhase};
use nannou::event::MouseScrollDelta::LineDelta;
use nannou::geom::Point2;

use crate::{Model, print_reaction_equation};
use crate::draw_legend::clicked_on_sources;
use crate::nuklid::Nuklid;
use crate::nuklid::ZerfallsArt::*;

pub fn mouse_clicked(app: &App, model: &mut Model, mouse_button: MouseButton) {
    find_hovered_element(app, model);
    clicked_on_sources(app, model);
    pressed_middle(model, mouse_button);
}

fn pressed_middle(model: &mut Model, mouse_button: MouseButton) {
    if mouse_button != MouseButton::Middle { return; }

    let nuklid = match &model.selected_nuklid {
        None => return,
        Some(x) => x
    };
    println!();
    print_reaction_equation::print_equation(model, nuklid, 200);
}

pub fn mouse_moved(app: &App, model: &mut Model, point: Point2) {
    drag_viewport(app, model);
}

pub fn mouse_scroll(app: &App, model: &mut Model, scroll_delta: MouseScrollDelta, touch_phase: TouchPhase) {
    scroll_scale_viewport(app, model, scroll_delta);
}


fn find_hovered_element(app: &App, model: &mut Model) {
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

    if let Some(sel) = &model.selected_nuklid {
        if sel.protonen == y_index && sel.neutronen == x_index {
            model.selected_nuklid = None;
            return;
        }
    }

    //If the Currently Clicked on Nuklide is the same as the already selected we deselect it
    model.selected_nuklid = None;

    //Check if Nuklid exists, and get if it exists
    let x_achse_map = match nuklids.get(&y_index) {
        Some(x) => x,
        None => { return; }
    };

    let nuklid = match x_achse_map.get(&x_index) {
        Some(x) => x,
        None => { return; }
    };

    model.selected_nuklid = Some(nuklid.clone());
    model.reaction_chain = advance_decay_chain(vec![nuklid.clone()], nuklids);
    eprintln!("{:?}", model.reaction_chain);
    //TODO hier die Chain finden!
}

fn advance_decay_chain(mut vec: Vec<Nuklid>, map: &HashMap<u8, HashMap<u8, Nuklid>>) -> Vec<Nuklid> {
    let parent = match vec.last() {
        None => return vec![],
        Some(x) => x
    };

    //TODO should be able to make the addition in i8 instead
    let child_p_n = (
        (parent.protonen as i16 + parent.zerfalls_art.delta_prot() as i16) as u8,
        (parent.neutronen as i16 + parent.zerfalls_art.delta_neut() as i16) as u8
    );

    let child = match (
        match map.get(&child_p_n.0) {
            Some(x) => x,
            None => return vec![],
        }
    ).get(&child_p_n.1) {
        Some(x) => x,
        None => return vec![],
    };
    vec.push(child.clone());

    //Return if Element is Stable of has no Path
    match &child.zerfalls_art {
        SF | Stable | Unknown => { return vec; }
        _ => advance_decay_chain(vec, map)
    }
}

fn drag_viewport(app: &App, model: &mut Model) {
    if app.mouse.buttons.right().is_down() {
        let delta_x: f32 = model.old_mouse_pos.0 - app.mouse.x;
        let delta_y: f32 = model.old_mouse_pos.1 - app.mouse.y;
        model.translate.0 += delta_x;
        model.translate.1 += delta_y;

        //Limit translation
        if model.translate.0 > model.square_size * 180. {
            model.translate.0 = model.square_size * 180.;
        } else if model.translate.0 < -200. {
            model.translate.0 = -200.;
        }

        if model.translate.1 > model.square_size * 120. {
            model.translate.1 = model.square_size * 120.;
        } else if model.translate.1 < -200. {
            model.translate.1 = -200.;
        }
    }
    model.old_mouse_pos.0 = app.mouse.x;
    model.old_mouse_pos.1 = app.mouse.y;
}

fn scroll_scale_viewport(app: &App, model: &mut Model, scroll_delta: MouseScrollDelta) {
    //Change the size of the Nuklid rectangle
    if let LineDelta(_, y) = scroll_delta {
        //Compute new Square size
        let new_size = model.square_size + (y * 2.);
        if new_size <= 4. { return; }
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

        //Limit translation
        if model.translate.0 > model.square_size * 200. {
            model.translate.0 = model.square_size * 200.;
        } else if model.translate.0 < -100. {
            model.translate.0 = -100.;
        }

        if model.translate.1 > model.square_size * 200. {
            model.translate.1 = model.square_size * 200.;
        } else if model.translate.1 < -100. {
            model.translate.1 = -100.;
        }
    }
}