use numpy::{PyReadonlyArray1, PyReadonlyArray2};
use pyo3::prelude::*;
extern crate supervised_algos;

use ndarray::Array2;

#[pyfunction]
fn knn(x_train: PyReadonlyArray2<'_, f64>, y_train: PyReadonlyArray1<'_, f64>) -> PyResult<()> {
    let x_train_array: Array2<f64> = x_train.to_owned_array();
    let y_train_array = y_train.to_owned_array();

    println!("X_train shape: {:?}", x_train_array.dim());
    println!("y_train shape: {:?}", y_train_array.dim());

    Ok(())
}

/// A Python module implemented in Rust.
#[pymodule]
fn rs_ml(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(knn, m)?)?;
    Ok(())
}
