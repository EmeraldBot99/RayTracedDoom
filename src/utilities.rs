
pub struct Utilities;

impl Utilities {
    pub fn normalize(value: u32, min: u32, max: u32) -> f32 {
        (value - min) as f32 / (max - min) as f32
    }

}
