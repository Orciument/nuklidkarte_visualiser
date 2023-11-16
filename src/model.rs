use nannou::prelude::WindowId;
use crate::nuklid::Nuklid;

pub struct Model {
    pub window: WindowId,
    pub nuklids: Vec<(u8, Vec<Option<Nuklid>>)>,
    pub old_mouse_pos: (f32, f32),
    pub translate: (f32, f32),
    pub square_size: f32,
    pub reaction_chain: Vec<Nuklid>,
}
