use super::constant::INVALID;
pub trait Grid: Send + Sync {
    fn get_end_x(&self) -> f64;
    fn get_end_y(&self) -> f64;
    fn get_invalid(&self) -> f64 {
        INVALID
    }
}
