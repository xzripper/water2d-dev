// window.rs - Part of Water2D.

use blue_engine::{
    header::{
        WindowDescriptor, PowerPreference
    }
};

use pyo3::prelude::*;

#[pyclass]
pub struct Window {
    #[pyo3(get, set)]
    pub title: String,

    #[pyo3(get, set)]
    pub window_width: u32,

    #[pyo3(get, set)]
    pub window_height: u32,

    #[pyo3(get, set)]
    pub resizable: bool,

    #[pyo3[get]]
    _window: WindowDescriptor
}

#[pymethods]
impl Window {
    #[new]
    pub fn new(title: String, width: u32, height: u32, resizable: bool) -> Window {
        Window {
            title: title,
            window_width: width,
            window_height: height,
            resizable: resizable,

            _window: WindowDescriptor {
                width: width,
                height: height,
                title: Box::leak(title.into_boxed_str()),
                decorations: true,
                resizable: resizable,
                power_preference: PowerPreference::LowPower // Let user choose.
            }
        }
    }
}
