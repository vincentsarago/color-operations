use numpy::ndarray::Array3;
use numpy::ndarray::ArrayView3;

use crate::colorspace::{convert, ColorSpace};

pub fn saturate_rgb(arr: ArrayView3<'_, f64>, satmult: f64) -> Array3<f64> {
    let shape = arr.shape();
    assert_eq!(shape[0], 3, "The 0th dimension must contain 3 bands");

    let dim_i = shape[1];
    let dim_j = shape[2];

    let mut out = Array3::<f64>::zeros((3, dim_i, dim_j));
    for i in 0..dim_i {
        for j in 0..dim_j {
            let r = arr[(0, i, j)];
            let g = arr[(1, i, j)];
            let b = arr[(2, i, j)];

            let mut c_lch = convert((r, g, b).into(), ColorSpace::RGB, ColorSpace::LCH);
            c_lch.two *= satmult;
            let c_rgb = convert(c_lch, ColorSpace::LCH, ColorSpace::RGB);

            out[(0, i, j)] = c_rgb.one;
            out[(1, i, j)] = c_rgb.two;
            out[(2, i, j)] = c_rgb.three;
        }
    }

    out
}
