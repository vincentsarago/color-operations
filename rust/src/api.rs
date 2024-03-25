use numpy::{IntoPyArray, PyArray3, PyReadonlyArray3};
use pyo3::prelude::*;

/// Convert array of RGB -> LCH, adjust saturation, back to RGB
///
/// A special case of convert_arr with hardcoded color spaces and a bit of data manipulation inside
/// the loop.
#[pyfunction]
fn saturate_rgb<'py>(
    py: Python<'py>,
    arr: PyReadonlyArray3<f64>,
    satmult: f64,
) -> &'py PyArray3<f64> {
    let arr = arr.as_array();
    let out = crate::saturation::saturate_rgb(arr, satmult);
    out.into_pyarray(py)
}

/// A Python module implemented in Rust.
#[pymodule]
pub fn color_operations(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(saturate_rgb, m)?)?;
    Ok(())
}
