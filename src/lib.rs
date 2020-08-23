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

#[pyfunction]
fn get_length(a: &str) -> PyResult<usize> {
    Ok(a.len())
}

#[pyfunction]
fn multiply_array(a: Vec<usize>, b: usize) -> PyResult<Vec<usize>> {
    Ok(a.iter().map(|v| v * b).collect())
}

#[pyfunction]
fn multiply_tuple(a: (usize, usize), b: usize) -> PyResult<(usize, usize)> {
    Ok((a.0 * b, a.1 * b))
}

#[pymodule]
fn test01(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!( hello ))?;
    m.add_wrapped(wrap_pyfunction!( pi_times ))?;
    m.add_wrapped(wrap_pyfunction!( get_length ))?;
    m.add_wrapped(wrap_pyfunction!( multiply_array ))?;
    m.add_wrapped(wrap_pyfunction!( multiply_tuple ))?;

    Ok(())
}