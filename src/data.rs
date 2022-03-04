#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64
}

impl Point {

    pub fn new_random() -> Self {
        Self { x: js_sys::Math::random(), y: js_sys::Math::random() }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Area {
    pub width: u32,
    pub height: u32 
}

pub const QUAD_UVS: [f32; 12] = [
    0.0, 0.0, //bottom-left
    0.0, 1.0, // left-top
    1.0, 0.0, // right-bottom
    1.0, 0.0, // right-bottom
    0.0, 1.0, // left-top
    1.0, 1.0, // right-top
];
