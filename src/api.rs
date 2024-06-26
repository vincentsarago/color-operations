use numpy::ndarray::Array3;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use crate::colorspace::{ColorSpace, LCHColor, RGBColor};
use numpy::{IntoPyArray, PyArray3, PyReadonlyArray3};

#[pyfunction]
pub fn convert_arr<'py>(
    py: Python<'py>,
    arr: PyReadonlyArray3<f64>,
    src: ColorSpace,
    dst: ColorSpace,
) -> PyResult<Bound<'py, PyArray3<f64>>> {
    let arr = arr.as_array();
    let shape = arr.shape();
    if shape[0] != 3 {
        return Err(PyValueError::new_err(
            "The 0th dimension must contain 3 bands",
        ));
    }

    let dim_i = shape[1];
    let dim_j = shape[2];

    let out = py.allow_threads(|| {
        let mut out = Array3::<f64>::zeros((3, dim_i, dim_j));
        for i in 0..dim_i {
            for j in 0..dim_j {
                let c1 = arr[(0, i, j)];
                let c2 = arr[(1, i, j)];
                let c3 = arr[(2, i, j)];

                let converted = crate::colorspace::convert((c1, c2, c3), src, dst);

                out[(0, i, j)] = converted.0;
                out[(1, i, j)] = converted.1;
                out[(2, i, j)] = converted.2;
            }
        }

        out
    });

    Ok(out.into_pyarray_bound(py))
}

#[pyfunction]
pub fn convert(
    one: f64,
    two: f64,
    three: f64,
    src: ColorSpace,
    dst: ColorSpace,
) -> (f64, f64, f64) {
    crate::colorspace::convert((one, two, three), src, dst)
}

/// Convert array of RGB -> LCH, adjust saturation, back to RGB
///
/// A special case of convert_arr with hardcoded color spaces and a bit of data manipulation inside
/// the loop.
#[pyfunction]
pub fn saturate_rgb<'py>(
    py: Python<'py>,
    arr: PyReadonlyArray3<f64>,
    satmult: f64,
) -> PyResult<Bound<'py, PyArray3<f64>>> {
    let arr = arr.as_array();
    let shape = arr.shape();
    if shape[0] != 3 {
        return Err(PyValueError::new_err(
            "The 0th dimension must contain 3 bands",
        ));
    }

    let dim_i = shape[1];
    let dim_j = shape[2];

    let out = py.allow_threads(|| {
        let mut out = Array3::<f64>::zeros((3, dim_i, dim_j));
        for i in 0..dim_i {
            for j in 0..dim_j {
                let r = arr[(0, i, j)];
                let g = arr[(1, i, j)];
                let b = arr[(2, i, j)];

                let rgb = RGBColor { r, g, b };
                let mut c_lch: LCHColor = rgb.into();
                c_lch.c *= satmult;
                let c_rgb: RGBColor = c_lch.into();

                out[(0, i, j)] = c_rgb.r;
                out[(1, i, j)] = c_rgb.g;
                out[(2, i, j)] = c_rgb.b;
            }
        }

        out
    });

    Ok(out.into_pyarray_bound(py))
}

/// A Python module implemented in Rust.
#[pymodule]
pub fn _rust(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(saturate_rgb, m)?)?;
    m.add_function(wrap_pyfunction!(convert, m)?)?;
    m.add_function(wrap_pyfunction!(convert_arr, m)?)?;
    Ok(())
}
