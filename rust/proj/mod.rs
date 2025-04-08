/// Projection Parsing tools
pub mod parser;

pub use parser::*;
use s2json::VectorPoint;

/// A Projection Transform Definition
/// Temporary placeholder
#[derive(Debug)]
pub struct ProjectionTransformDefinition {}

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
    pub fn forward<M: Clone + Default>(&self, p: VectorPoint<M>) -> VectorPoint<M> {
        p
    }
    /// Move backward from destination projection to source projection
    pub fn inverse<M: Clone + Default>(&self, p: VectorPoint<M>) -> VectorPoint<M> {
        p
    }
}
