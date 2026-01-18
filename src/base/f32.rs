use macro_utils::VectorMath;
use macro_utils::mat_vec_mul;
use crate::base::matrix::Mat;

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

#[mat_vec_mul(Vec2)]
pub type Mat2 = Mat<f32,2,2>;

#[mat_vec_mul(Vec3)]
pub type Mat3 = Mat<f32, 3, 3>;

#[mat_vec_mul(Vec4)]
pub type Mat4 = Mat<f32, 4, 4>;