/// Calculates vertical fov if horizontal fov is known.
/// Aspect ratio is a ratio of the screen width to its height. 
pub fn vertical_fov_from_horizontal(horizontal_fov: f32, aspect_ratio: f32) -> f32 {
    (aspect_ratio * (0.5 * horizontal_fov).tan()).atan()
}

pub fn assert_eq_f32(lhs: f32, rhs: f32) {
    if (lhs - rhs).abs() > 1e-5 {
        panic!("{} != {}", lhs, rhs);
    } 
}

pub fn assert_eq_option_f32(lhs: Option<f32>, rhs: Option<f32>) {
    if let Some(lhs) = lhs {
        if let Some(rhs) = rhs {
            assert_eq_f32(lhs, rhs);
        }
        else {
            panic!("{} != {:?}", lhs, rhs);
        }
    }
    else if let Some(rhs) = rhs {
        panic!("{:?} != {}", lhs, rhs);
    }
}