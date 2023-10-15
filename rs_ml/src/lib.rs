use pyo3::prelude::*;
extern crate supervised_algos;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string_sevens(a: usize, b: usize) -> PyResult<String> {
    Ok((supervised_algos::add_seven(a) + supervised_algos::add_seven(b)).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn rs_ml(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string_sevens, m)?)?;
    Ok(())
}
