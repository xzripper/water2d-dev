// engine.rs - Part of Water2D.

use blue_engine::header::{Engine, WindowDescriptor as WD};
use pyo3::prelude::*;

#[derive(Debug, Clone, Copy)]
#[pyclass]
pub struct WindowSettings {
    /// The width of the window
    pub width: u32,
    /// The height of the window
    pub height: u32,
    /// The title of the window
    pub title: &'static str,
    /// Should the window contain the keys like minimize, maximize, or resize?
    pub decorations: bool,
    /// Should the window be resizable
    pub resizable: bool,
}
impl std::default::Default for WindowSettings {
    /// Will quickly create a window with default settings
    fn default() -> Self {
        Self {
            width: 800,
            height: 600,
            title: "Water 2D",
            decorations: true,
            resizable: true,
        }
    }
}

#[pyclass(unsendable)]
pub struct Water2D {
    engine: Engine,
}

#[pymethods]
impl Water2D {
    #[new]
    pub fn new(window: WindowSettings) -> PyResult<Water2D> {
        let t = Water2D {
            engine: Engine::new(WD {
                width: window.width,
                height: window.height,
                title: window.title,
                decorations: window.decorations,
                resizable: window.resizable,
                ..Default::default()
            })
            .expect("RUST::BACKEND::ENGINE / Failed to initialize window."),
        };

        Ok(t)
    }

    /*pub fn run(&self) {
        let Self { engine } = self;
        engine
            .clone()
            .update_loop(move |_, _, _, _, _| {})
            .expect("RUST::BACKEND::ENGINE / An unexpected error has been during window update.")
    } */
}
