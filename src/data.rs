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

pub const QUAD_GEOM_UNIT: [f32; 8] = [
    0.0, 1.0, // top-left
    0.0, 0.0, //bottom-left
    1.0, 1.0, // top-right
    1.0, 0.0, // bottom-right
];
