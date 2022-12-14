//! Stems hold the weight of the plant and are largely responsible for its
//! shape. Parameters on `Stem` (not `Leaf`) are responsible for pinnation,
//! the feathery leaf structure that's the most immediately recognizable
//! property of ferns.

// in plant_structures/stems.rs
pub mod phloem;
pub mod xylem;

pub struct Stem {
    pub furled: bool,
}

pub type StemSet = Vec<Stem>; // not sure if I like calling a `Vec` a "Set"
