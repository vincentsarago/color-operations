use numpy::ndarray::Array3;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use crate::colorspace::{LABColor, LCHColor, LUVColor, RGBColor, XYZColor};
use numpy::{IntoPyArray, PyArray3, PyReadonlyArray3};

#[pyclass]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
enum ColorSpace {
    rgb = 0,
    xyz = 1,
    lab = 2,
    lch = 3,
    luv = 4,
}

fn _convert(c: (f64, f64, f64), src: ColorSpace, dst: ColorSpace) -> (f64, f64, f64) {
    use ColorSpace::*;
    match (src, dst) {
        (rgb, rgb) => c,
        (xyz, xyz) => c,
        (lab, lab) => c,
        (lch, lch) => c,
        (luv, luv) => c,

        (rgb, lab) => LABColor::from(RGBColor::from(c)).into(),
        (rgb, lch) => LCHColor::from(RGBColor::from(c)).into(),
        (rgb, xyz) => XYZColor::from(RGBColor::from(c)).into(),
        (rgb, luv) => LUVColor::from(RGBColor::from(c)).into(),
        (xyz, lab) => LABColor::from(XYZColor::from(c)).into(),
        (xyz, lch) => LCHColor::from(XYZColor::from(c)).into(),
        (xyz, rgb) => RGBColor::from(XYZColor::from(c)).into(),
        (xyz, luv) => LUVColor::from(XYZColor::from(c)).into(),
        (lab, xyz) => XYZColor::from(LABColor::from(c)).into(),
        (lab, lch) => LCHColor::from(LABColor::from(c)).into(),
        (lab, rgb) => RGBColor::from(LABColor::from(c)).into(),
        (lab, luv) => LUVColor::from(LABColor::from(c)).into(),
        (lch, lab) => LABColor::from(LCHColor::from(c)).into(),
        (lch, xyz) => XYZColor::from(LCHColor::from(c)).into(),
        (lch, rgb) => RGBColor::from(LCHColor::from(c)).into(),
        (lch, luv) => LUVColor::from(LCHColor::from(c)).into(),
        (luv, lab) => LABColor::from(LUVColor::from(c)).into(),
        (luv, xyz) => XYZColor::from(LUVColor::from(c)).into(),
        (luv, rgb) => RGBColor::from(LUVColor::from(c)).into(),
        (luv, lch) => LCHColor::from(LUVColor::from(c)).into(),
    }
}

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

                let converted = _convert((c1, c2, c3), src, dst);

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
pub fn convert<'py>(
    one: f64,
    two: f64,
    three: f64,
    src: ColorSpace,
    dst: ColorSpace,
) -> (f64, f64, f64) {
    _convert((one, two, three), src, dst)
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
pub fn color_operations(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(saturate_rgb, m)?)?;
    Ok(())
}
