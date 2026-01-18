
#[cfg(test)]
mod tests {
    #[test]
    fn test_f32_vec2() {
        use gk_math::base::f32::Vec2;
        
        let v1 = Vec2::new(1.0, 2.0);
        let v2 = Vec2::new(3.0, 4.0);

        assert_eq!(v1 + v2, v2 + v1);
    }

    #[test]
    fn test_f32_vec3() {
        use gk_math::base::f32::Vec3;

        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(2.0, 4.0, 6.0);

        assert_eq!(v1 * 2.0, v2);
        assert_eq!(v1.dot(&v2), 28.0);
        assert_eq!(Vec3::cross(&v1, &v2), Vec3::new(0.0, 0.0, 0.0));
        assert_eq!(v1 * v2, Vec3::new(2.0, 8.0, 18.0));
        assert_eq!(v1.sum(), 6.0);
    }
}