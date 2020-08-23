use pyo3::prelude::*;
use std::f64::consts::PI;
use pyo3::wrap_pyfunction;

#[pyfunction]
fn hello() -> PyResult<()> {
    println!("Hello, world!");
    Ok(())
}

#[pyfunction]
fn pi_times( n: usize ) -> PyResult<Vec<f64>> {
    Ok(
        (0..n).map(|i| i as f64 * PI).collect()
    )
}

#[pymodule]
fn test01(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!( hello ))?;
    m.add_wrapped(wrap_pyfunction!( pi_times ))?;

    Ok(())
}