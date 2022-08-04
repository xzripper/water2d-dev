// engine.rs - Part of Water2D.

use blue_engine::{
    header::{
        Engine, WindowDescriptor
    }
};

use pyo3::prelude::*;

use std::sync::{
    Arc, Mutex
};

#[pyclass]
pub struct Water2D {
}

#[pymethods]
impl Water2D {
    #[new]
    pub fn new(window: WindowDescriptor) -> Water2D {
        Water2D {
            engine: Engine::new(window).expect("RUST::BACKEND::ENGINE / Failed to initialize window.")
        }
    }

    pub fn run(&self) -> () {
        self.engine
            .update_loop(move | _, _, _, _, _ | {})
            .expect("RUST::BACKEND::ENGINE / An unexpected error has been during window update.")
    }
}
