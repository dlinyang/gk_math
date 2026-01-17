use macro_utils::VectorMath;

#[derive(Clone, Copy, VectorMath)]
pub struct Vec2{
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Copy, VectorMath)]
pub struct  Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}


#[derive(Clone, Copy, VectorMath)]
pub struct  Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}