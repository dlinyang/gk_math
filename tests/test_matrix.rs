#[cfg(test)]
mod tests {
    #[test]
    fn test_f32_mat2() {
        use gk_math::base::f32::Mat2;
        
        let m1 = Mat2::new([[1.0, 2.0],[2.0, 1.0]]);
        let m2 = Mat2::new([[3.0, 4.0],[5.0, 6.0]]);

        assert_eq!(m1 + m2, m2 + m1);
    }

    #[test]
    fn test_f32_mat3() {
        use gk_math::base::f32::Mat3;

        let m1 = Mat3::new([[1.0, 2.0, 3.0],[1.0, 2.0, 3.0], [1.0, 2.0, 3.0]]);
        let m2 = Mat3::new([[2.0, 4.0, 6.0],[2.0, 4.0, 6.0], [2.0, 4.0, 6.0]]);

        assert_eq!(true, m1.is_square());
        assert_eq!(m1 * 2.0, m2);
        assert_eq!(m1 * m2, Mat3::new([[12.0, 24.0, 36.0], [12.0, 24.0, 36.0], [12.0, 24.0, 36.0]]));
    }

    #[test]
    fn test_f32_mat3_mul_vec3() {
        use gk_math::base::f32::{Mat3, Vec3};
        let m = Mat3::identity();
        let v = Vec3::new(1.0, 2.0, 3.0);

        assert_eq!(m * v, v);
    }
}