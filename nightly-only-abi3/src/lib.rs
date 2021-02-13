//#![feature(box_syntax)] // Make this crate nightly only

use pyo3::prelude::*;

#[pyclass]
struct DummyClass {}

#[pymethods]
impl DummyClass {
    #[staticmethod]
    fn get_42() -> PyResult<usize> {
        Ok(42)
    }
}

#[pymodule]
fn nightly_only_abi3(_py: Python, m: &PyModule) -> PyResult<()> {
    //let _five = box 5;

    m.add_class::<DummyClass>()?;
    m.add("fourtytwo", 42)?;

    Ok(())
}
