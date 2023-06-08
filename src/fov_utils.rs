/// Calculates vertical fov if horizontal fov is known.
/// Aspect ratio is a ratio of the screen width to its height. 
pub fn vertical_fov_from_horizontal(horizontal_fov: f32, aspect_ratio: f32) -> f32 {
    (aspect_ratio * (0.5 * horizontal_fov).tan()).atan()
}