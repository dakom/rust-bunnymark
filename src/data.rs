#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32
}

impl Point {

    pub fn new_random() -> Self {
        Self { x: js_sys::Math::random() as f32, y: js_sys::Math::random() as f32}
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Area {
    pub width: f32,
    pub height: f32 
}

pub const QUAD_GEOM_UNIT: [f32; 8] = [
    0.0, 1.0, // top-left
    0.0, 0.0, //bottom-left
    1.0, 1.0, // top-right
    1.0, 0.0, // bottom-right
];
