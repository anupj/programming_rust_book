//! Overall simulation control.
//!
//! The simulation algorithm is complex and has a lot of tweakable parameters.

use crate::plant_structures::{Fern, FernType};
use std::fs::File;
use std::time::Duration;

/// The simulated universe
pub struct Terrarium {
    ferns: Vec<Fern>,
}

impl Terrarium {
    /// Create a new empty Terrarium
    pub fn new() -> Terrarium {
        Terrarium { ferns: vec![] }
    }

    /// Load a terrarium from a `.tm` file
    pub fn load(filename: &str) -> Terrarium {
        /// This implementation is, like everything else in here
        /// completely bogus
        File::open(filename).unwrap();
        Terrarium {
            ferns: vec![Fern::new(FernType::Fiddlehead)],
        }
    }

    /// Get a reference to a fern inside the simulation
    pub fn fern(&self, index: usize) -> &Fern {
        &self.ferns[index]
    }

    #[allow(unused_variables)]
    /// Let the sun shine in and run the simulation for a
    /// given amount of time
    pub fn apply_sunlight(&mut self, time: Duration) {
        for f in &mut self.ferns {
            for s in &mut f.stems {
                s.furled = false;
            }
        }
    }
}
