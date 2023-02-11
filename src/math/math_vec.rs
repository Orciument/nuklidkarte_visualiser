use nannou::geom::Vec2;

//Functions that do Math Vector Operations on Rust Vec's (all Names are to be understand in the CContext of Math, not Rust)
//TODO sollte Vec2 erweitern, anstatt sein eigens Ding zu sein
pub fn scale_vec2(vec: &Vec2, factor: &f32) -> Vec2 {
    Vec2::new(vec.x * factor, vec.y * factor)
}
