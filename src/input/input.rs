// Input interface
pub trait Input {
    // Computes fan strength
    fn compute(&mut self) -> f64;
}
