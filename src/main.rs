// main.rs - Part of Water2D.

use pyo3::prelude::*;

mod _water2d;

fn main() -> () {}

#[pymodule]
fn water2d(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_class::<_water2d::engine::Water2D>()?;
    module.add_class::<_water2d::window::Window>()?;

    Ok(())
}
