use numpy::ndarray::Array3;
use numpy::ndarray::ArrayView3;

use crate::colorspace::{convert, Color, ColorSpace};

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

            let c_rgb = saturate_single_rgb((r, g, b).into(), satmult);

            out[(0, i, j)] = c_rgb.one;
            out[(1, i, j)] = c_rgb.two;
            out[(2, i, j)] = c_rgb.three;
        }
    }

    out
}

pub fn saturate_single_rgb(input: Color, satmult: f64) -> Color {
    let mut c_lch = convert(input, ColorSpace::RGB, ColorSpace::LCH);
    c_lch.two *= satmult;
    convert(c_lch, ColorSpace::LCH, ColorSpace::RGB)
}

#[cfg(test)]
mod test {
    use approx::assert_relative_eq;

    use crate::colorspace::Color;

    use super::*;

    #[test]
    fn test_saturate_rgb() {
        let rgb = Color::new(0.392156, 0.776470, 0.164705);
        let out_rgb = saturate_single_rgb(rgb, 0.0);
        assert_relative_eq!(out_rgb.one, out_rgb.two, epsilon = 0.1);
        assert_relative_eq!(out_rgb.two, out_rgb.three, epsilon = 0.1);
    }
}
