use lazy_static::lazy_static;

// Constants
const BINTERCEPT: f64 = 4.0 / 29.; // 0.137931
const DELTA: f64 = 6.0 / 29.; // 0.206896

// Can't use powf directly in const
// https://users.rust-lang.org/t/why-is-f64-powf-not-a-const-fn/31093/5
lazy_static! {
    pub static ref T0: f64 = DELTA.powi(3) ; // 0.008856
    pub static ref ALPHA: f64 = (DELTA.powi(-2)) / 3.;  // 7.787037
    pub static ref KAPPA: f64 = (29.0f64 / 3.).powi(3);  // 903.3
}

const THIRD: f64 = 1.0 / 3.;
const GAMMA: f64 = 2.2;
const XN: f64 = 0.95047;
// const YN: f64 = 1.0;
const ZN: f64 = 1.08883;
// const DENOM_N: f64 = XN + (15. * YN) + (3. * ZN);
// const UPRIME_N: f64 = (4. * XN) / DENOM_N;
// const VPRIME_N: f64 = (9. * YN) / DENOM_N;

/// Compile time option to use
/// sRGB companding (default, True) or simplified gamma (False)
/// sRGB companding is slightly slower but is more accurate at
/// the extreme ends of scale
/// Unit tests tuned to sRGB companding, change with caution
const SRGB_COMPAND: bool = true;

/// A color with three values
#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub one: f64,
    pub two: f64,
    pub three: f64,
}

impl Color {
    pub fn new(one: f64, two: f64, three: f64) -> Self {
        Color { one, two, three }
    }
}

impl From<Color> for (f64, f64, f64) {
    fn from(value: Color) -> Self {
        (value.one, value.two, value.three)
    }
}

impl From<(f64, f64, f64)> for Color {
    fn from(value: (f64, f64, f64)) -> Self {
        Color::new(value.0, value.1, value.2)
    }
}

#[allow(clippy::upper_case_acronyms)]
pub enum ColorSpace {
    RGB,
    LCH,
}

/// A Color defined by Red, Green, Blue
#[derive(Clone, Copy, Debug)]
pub struct RGBColor {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

/// A Color defined by X, Y, Z
#[derive(Clone, Copy, Debug)]
pub struct XYZColor {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl From<RGBColor> for XYZColor {
    fn from(value: RGBColor) -> Self {
        let r = value.red;
        let g = value.green;
        let b = value.blue;

        // Convert RGB to linear scale
        let (rl, gl, bl) = if SRGB_COMPAND {
            let rl = if r <= 0.04045 {
                r / 12.92
            } else {
                ((r + 0.055) / 1.055).powf(2.4)
            };

            let gl = if g <= 0.04045 {
                g / 12.92
            } else {
                ((g + 0.055) / 1.055).powf(2.4)
            };

            let bl = if b <= 0.04045 {
                b / 12.92
            } else {
                ((b + 0.055) / 1.055).powf(2.4)
            };

            (rl, gl, bl)
        } else {
            // Use "simplified sRGB"
            (r.powf(GAMMA), g.powf(GAMMA), b.powf(GAMMA))
        };

        // matrix mult for srgb->xyz,
        // includes adjustment for reference white
        let x = ((rl * 0.4124564) + (gl * 0.3575761) + (bl * 0.1804375)) / XN;
        let y = (rl * 0.2126729) + (gl * 0.7151522) + (bl * 0.0721750);
        let z = ((rl * 0.0193339) + (gl * 0.1191920) + (bl * 0.9503041)) / ZN;

        Self { x, y, z }
    }
}

/// Convert a color from one color space to another
pub fn convert(c: Color, src: ColorSpace, dst: ColorSpace) -> Color {
    match (src, dst) {
        (ColorSpace::RGB, ColorSpace::RGB) => c,
        (ColorSpace::RGB, ColorSpace::LCH) => {
            let c = rgb_to_xyz(c);
            let c = xyz_to_lab(c);
            lab_to_lch(c)
        }
        (ColorSpace::LCH, ColorSpace::LCH) => c,
        (ColorSpace::LCH, ColorSpace::RGB) => lch_to_rgb(c),
    }
}

#[inline(always)]
fn rgb_to_xyz(c: Color) -> Color {
    let (r, g, b) = c.into();

    // Convert RGB to linear scale
    let (rl, gl, bl) = if SRGB_COMPAND {
        let rl = if r <= 0.04045 {
            r / 12.92
        } else {
            ((r + 0.055) / 1.055).powf(2.4)
        };

        let gl = if g <= 0.04045 {
            g / 12.92
        } else {
            ((g + 0.055) / 1.055).powf(2.4)
        };

        let bl = if b <= 0.04045 {
            b / 12.92
        } else {
            ((b + 0.055) / 1.055).powf(2.4)
        };

        (rl, gl, bl)
    } else {
        // Use "simplified sRGB"
        (r.powf(GAMMA), g.powf(GAMMA), b.powf(GAMMA))
    };

    // matrix mult for srgb->xyz,
    // includes adjustment for reference white
    let x = ((rl * 0.4124564) + (gl * 0.3575761) + (bl * 0.1804375)) / XN;
    let y = (rl * 0.2126729) + (gl * 0.7151522) + (bl * 0.0721750);
    let z = ((rl * 0.0193339) + (gl * 0.1191920) + (bl * 0.9503041)) / ZN;

    (x, y, z).into()
}

#[inline(always)]
fn xyz_to_lab(c: Color) -> Color {
    let (x, y, z) = c.into();

    // convert XYZ to LAB colorspace

    let fx = if x > *T0 {
        x.powf(THIRD)
    } else {
        (*ALPHA * x) + BINTERCEPT
    };

    let fy = if y > *T0 {
        y.powf(THIRD)
    } else {
        (*ALPHA * y) + BINTERCEPT
    };

    let fz = if z > *T0 {
        z.powf(THIRD)
    } else {
        (*ALPHA * z) + BINTERCEPT
    };

    let l = (116. * fy) - 16.;
    let a = 500. * (fx - fy);
    let b = 200. * (fy - fz);

    (l, a, b).into()
}

#[inline(always)]
fn lab_to_lch(c: Color) -> Color {
    let (l, a, b) = c.into();
    (l, ((a * a) + (b * b)).powf(0.5), b.atan2(a)).into()
}

#[inline(always)]
fn lch_to_lab(c: Color) -> Color {
    let (l, c, h) = c.into();

    let a = c * h.cos();
    let b = c * h.sin();

    (l, a, b).into()
}

#[inline(always)]
fn lab_to_xyz(c: Color) -> Color {
    let (l, a, b) = c.into();

    let tx = ((l + 16.) / 116.0) + (a / 500.0);
    let x = if tx > DELTA {
        tx.powi(3)
    } else {
        3. * DELTA * DELTA * (tx - BINTERCEPT)
    };

    let ty = (l + 16.) / 116.0;
    let y = if ty > DELTA {
        ty.powi(3)
    } else {
        3. * DELTA * DELTA * (ty - BINTERCEPT)
    };

    let tz = ((l + 16.) / 116.0) - (b / 200.0);
    let z = if tz > DELTA {
        tz.powi(3)
    } else {
        3. * DELTA * DELTA * (tz - BINTERCEPT)
    };

    // Reference illuminant
    (x, y, z).into()
}

#[inline(always)]
fn xyz_to_rgb(c: Color) -> Color {
    let (x, y, z) = c.into();

    // uses reference white d65
    let x = x * XN;
    let z = z * ZN;

    // XYZ to sRGB
    // expanded matrix multiplication
    let rlin = (x * 3.2404542) + (y * -1.5371385) + (z * -0.4985314);
    let glin = (x * -0.9692660) + (y * 1.8760108) + (z * 0.0415560);
    let blin = (x * 0.0556434) + (y * -0.2040259) + (z * 1.0572252);

    let (r, g, b) = if SRGB_COMPAND {
        let r = if rlin <= 0.0031308 {
            12.92 * rlin
        } else {
            (1.055 * (rlin.powf(1. / 2.4))) - 0.055
        };
        let g = if glin <= 0.0031308 {
            12.92 * glin
        } else {
            (1.055 * (glin.powf(1. / 2.4))) - 0.055
        };
        let b = if blin <= 0.0031308 {
            12.92 * blin
        } else {
            (1.055 * (blin.powf(1. / 2.4))) - 0.055
        };
        (r, g, b)
    } else {
        // Use simplified sRGB
        let r = rlin.powf(1. / GAMMA);
        let g = glin.powf(1. / GAMMA);
        let b = blin.powf(1. / GAMMA);
        (r, g, b)
    };

    // constrain to 0..1 to deal with any float drift
    let r = r.clamp(0.0, 1.0);
    let g = g.clamp(0.0, 1.0);
    let b = b.clamp(0.0, 1.0);

    (r, g, b).into()
}

/// Convert LCH color to RGB color
fn lch_to_rgb(c: Color) -> Color {
    let c = lch_to_lab(c);
    let c = lab_to_xyz(c);
    xyz_to_rgb(c)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tests() -> Vec<(Color, Color)> {
        vec![
            ((0., 0., 0.).into(), (0., 0., 0.).into()),
            ((1.0, 0., 0.).into(), (53.2, 104.6, 0.7).into()),
            (
                (0.392156, 0.776470, 0.164705).into(),
                (71.7, 83.5, 2.3).into(),
            ),
            (
                (0.0392, 0.1960, 0.3529).into(),
                (20.3517, 27.8757, -1.4612).into(),
            ),
            (
                (0.0456, 0.1929, 0.3941).into(),
                (20.8945, 34.9429, -1.3244).into(),
            ),
            ((1.0, 1.0, 1.0).into(), (100., 0., 2.8).into()),
        ]
    }

    fn color_near(a: Color, b: Color, tol: (f64, f64, f64)) -> bool {
        if (a.one - b.one).abs() > tol.0 {
            return false;
        }

        if (a.two - b.two).abs() > tol.1 {
            return false;
        }

        if (a.three - b.three).abs() > tol.2 {
            return false;
        }

        true
    }

    #[test]
    fn test_lch_to_rgb() {
        let tests = tests();
        for (rgb, lch) in tests {
            let argb = lch_to_rgb(lch);
            assert!(color_near(argb, rgb, (1.0, 1.0, 0.1)));
        }
    }
}
