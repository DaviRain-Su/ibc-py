use pyo3::prelude::*;

pub mod applications;
pub mod clients;
pub mod core;
pub mod hosts;
pub mod singer;
pub mod utils;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn ibc_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_class::<singer::Signer>()?;
    Ok(())
}
