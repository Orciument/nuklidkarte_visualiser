use std::collections::HashMap;

use criterion::{Criterion, criterion_group, criterion_main};
use nannou::prelude::WindowId;
use nannou::rand::random_range;

use nuklidkarte_visualiser::data::nuklid_json_deserializer::{deserialize_to_map};
use nuklidkarte_visualiser::model::Model;
use nuklidkarte_visualiser::mouse_controller::advance_decay_chain;
use nuklidkarte_visualiser::nuklid::Nuklid;

pub fn bench_nuklide_finder(c: &mut Criterion) {
    let mut m = Model {
        window: unsafe { WindowId::dummy() },
        nuklids: deserialize_to_map(),
        old_mouse_pos: (0., 0.),
        translate: (0., 0.),
        square_size: 50.,
        reaction_chain: vec![],
    };

    c.bench_function("Hash Map", |x| {
        x.iter(||
            find_hash_map(
                (random_range(-960., 960.), random_range(-513., 513.)),
                (1920., 1027.), &mut m,
            )
        )
    });
}

fn find_hash_map(mouse_p: (f32, f32), window_size: (f32, f32), model: &mut Model) {
    let nuklids: &HashMap<u8, HashMap<u8, Nuklid>> = &model.nuklids;

    // - Translated Origin * Flipped Origin + Translated Display (where the mouse is, while
    // rendering only what is visible is displayed and is shown in the bottom left corner as always
    // so we need to translate
    let corrected_x: f32 = (mouse_p.0 - window_size.0 * -0.5) + model.translate.0;
    let corrected_y: f32 = (mouse_p.1 - window_size.1 * -0.5) + model.translate.1;
    let x_index: u8 = (corrected_x / model.square_size - 0.5).round() as u8;
    let y_index: u8 = (corrected_y / model.square_size - 0.5).round() as u8;

    //Check if this Nuklide is already selected, if so, unselect it, return
    if let Some(sel) = model.reaction_chain.first() {
        if sel.protonen == y_index && sel.neutronen == x_index {
            model.reaction_chain.clear();
            return;
        }
    }

    //Unselect Nuklid in case the selection of the new Nuklid fails, e.g. if the is none
    model.reaction_chain.clear();

    //Check if Nuklid exists, and get if it exists
    let nuklid = match (match nuklids.get(&y_index) {
        Some(x) => x,
        None => return,
    })
        .get(&x_index)
    {
        Some(x) => x,
        None => return,
    };

    //Save the new selection (and chain)
    model.reaction_chain = advance_decay_chain(vec![nuklid.clone()], nuklids);
}
criterion_group!(benches, bench_nuklide_finder);
criterion_main!(benches);
