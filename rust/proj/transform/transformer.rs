use super::TransformCoordinates;

/// A Projection Transformer
#[derive(Debug)]
pub struct Transformer {}
impl Transformer {
    /// Create a new Transformer
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {}
    }
    /// Move forward from source projection to destination projection
    pub fn forward<P: TransformCoordinates>(&self, p: &P) -> P {
        p.clone()
    }
    /// Move forward from source projection to destination projection in place
    pub fn forward_mut<P: TransformCoordinates>(&self, _p: &mut P) {}
    /// Move backward from destination projection to source projection
    pub fn inverse<P: TransformCoordinates>(&self, p: &P) -> P {
        p.clone()
    }
    /// Move backward from destination projection to source projection in place
    pub fn inverse_mut<P: TransformCoordinates>(&self, _p: &mut P) {}
}
