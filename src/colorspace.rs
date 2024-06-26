use lazy_static::lazy_static;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

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
const YN: f64 = 1.0;
const ZN: f64 = 1.08883;
const DENOM_N: f64 = XN + (15. * YN) + (3. * ZN);
const UPRIME_N: f64 = (4. * XN) / DENOM_N;
const VPRIME_N: f64 = (9. * YN) / DENOM_N;

/// Compile time option to use
/// sRGB companding (default, True) or simplified gamma (False)
/// sRGB companding is slightly slower but is more accurate at
/// the extreme ends of scale
/// Unit tests tuned to sRGB companding, change with caution
const SRGB_COMPAND: bool = true;

/// A color with three values
#[derive(Clone, Copy, Debug)]
#[allow(clippy::upper_case_acronyms)]
pub enum Color {
    RGB(RGBColor),
    XYZ(XYZColor),
    LAB(LABColor),
    LCH(LCHColor),
    LUV(LUVColor),
}

impl Color {
    pub fn new_rgb(r: f64, g: f64, b: f64) -> Self {
        Self::RGB((r, g, b).into())
    }

    pub fn new_xyz(x: f64, y: f64, z: f64) -> Self {
        Self::XYZ((x, y, z).into())
    }

    pub fn new_lab(l: f64, a: f64, b: f64) -> Self {
        Self::LAB((l, a, b).into())
    }

    pub fn new_lch(l: f64, c: f64, h: f64) -> Self {
        Self::LCH((l, c, h).into())
    }

    pub fn new_luv(l: f64, u: f64, v: f64) -> Self {
        Self::LUV((l, u, v).into())
    }
}

impl From<Color> for (f64, f64, f64) {
    fn from(value: Color) -> Self {
        match value {
            Color::RGB(c) => c.into(),
            Color::XYZ(c) => c.into(),
            Color::LAB(c) => c.into(),
            Color::LCH(c) => c.into(),
            Color::LUV(c) => c.into(),
        }
    }
}

/// A Color defined by Red, Green, Blue
#[derive(Clone, Copy, Debug)]
pub struct RGBColor {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl From<(f64, f64, f64)> for RGBColor {
    fn from((r, g, b): (f64, f64, f64)) -> Self {
        RGBColor { r, g, b }
    }
}

impl From<RGBColor> for (f64, f64, f64) {
    fn from(value: RGBColor) -> Self {
        (value.r, value.g, value.b)
    }
}

/// A Color defined by X, Y, Z
#[derive(Clone, Copy, Debug)]
pub struct XYZColor {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl From<(f64, f64, f64)> for XYZColor {
    fn from((x, y, z): (f64, f64, f64)) -> Self {
        XYZColor { x, y, z }
    }
}

impl From<XYZColor> for (f64, f64, f64) {
    fn from(value: XYZColor) -> Self {
        (value.x, value.y, value.z)
    }
}

/// A Color defined by LAB
#[derive(Clone, Copy, Debug)]
pub struct LABColor {
    pub l: f64,
    pub a: f64,
    pub b: f64,
}

impl From<(f64, f64, f64)> for LABColor {
    fn from((l, a, b): (f64, f64, f64)) -> Self {
        LABColor { l, a, b }
    }
}

impl From<LABColor> for (f64, f64, f64) {
    fn from(value: LABColor) -> Self {
        (value.l, value.a, value.b)
    }
}

/// A Color defined by LCH
#[derive(Clone, Copy, Debug)]
pub struct LCHColor {
    pub l: f64,
    pub c: f64,
    pub h: f64,
}

impl From<(f64, f64, f64)> for LCHColor {
    fn from((l, c, h): (f64, f64, f64)) -> Self {
        LCHColor { l, c, h }
    }
}

impl From<LCHColor> for (f64, f64, f64) {
    fn from(value: LCHColor) -> Self {
        (value.l, value.c, value.h)
    }
}

/// A Color defined by LUV
#[derive(Clone, Copy, Debug)]
pub struct LUVColor {
    pub l: f64,
    pub u: f64,
    pub v: f64,
}

impl From<(f64, f64, f64)> for LUVColor {
    fn from((l, u, v): (f64, f64, f64)) -> Self {
        LUVColor { l, u, v }
    }
}

impl From<LUVColor> for (f64, f64, f64) {
    fn from(value: LUVColor) -> Self {
        (value.l, value.u, value.v)
    }
}

// Color space transformations

impl From<RGBColor> for LABColor {
    fn from(value: RGBColor) -> Self {
        let c: XYZColor = value.into();
        c.into()
    }
}

impl From<RGBColor> for LCHColor {
    fn from(value: RGBColor) -> Self {
        let c1: XYZColor = value.into();
        let c2: LABColor = c1.into();
        c2.into()
    }
}

impl From<RGBColor> for XYZColor {
    #[inline(always)]
    fn from(value: RGBColor) -> Self {
        let r = value.r;
        let g = value.g;
        let b = value.b;

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

impl From<RGBColor> for LUVColor {
    fn from(value: RGBColor) -> Self {
        let c1: XYZColor = value.into();
        c1.into()
    }
}

impl From<XYZColor> for LABColor {
    #[inline(always)]
    fn from(value: XYZColor) -> Self {
        let x = value.x;
        let y = value.y;
        let z = value.z;

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

        Self { l, a, b }
    }
}

impl From<XYZColor> for LCHColor {
    fn from(value: XYZColor) -> Self {
        let c1: LABColor = value.into();
        c1.into()
    }
}

impl From<XYZColor> for RGBColor {
    #[inline(always)]
    fn from(value: XYZColor) -> Self {
        let x = value.x;
        let y = value.y;
        let z = value.z;

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

        Self { r, g, b }
    }
}

impl From<XYZColor> for LUVColor {
    fn from(value: XYZColor) -> Self {
        let x = value.x;
        let y = value.y;
        let z = value.z;

        let denom = x + (15. * y) + (3. * z);
        let uprime = (4. * x) / denom;
        let vprime = (9. * y) / denom;

        let y = y / YN;

        let l = if y <= *T0 {
            *KAPPA * y
        } else {
            (116. * (y.powf(THIRD))) - 16.
        };

        let u = 13. * l * (uprime - UPRIME_N);
        let v = 13. * l * (vprime - VPRIME_N);
        Self { l, u, v }
    }
}

impl From<LABColor> for XYZColor {
    #[inline(always)]
    fn from(value: LABColor) -> Self {
        let l = value.l;
        let a = value.a;
        let b = value.b;

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
        Self { x, y, z }
    }
}

impl From<LABColor> for LCHColor {
    #[inline(always)]
    fn from(value: LABColor) -> Self {
        let l = value.l;
        let a = value.a;
        let b = value.b;

        let c = ((a * a) + (b * b)).powf(0.5);
        let h = b.atan2(a);
        Self { l, c, h }
    }
}

impl From<LABColor> for RGBColor {
    fn from(value: LABColor) -> Self {
        let c1: XYZColor = value.into();
        c1.into()
    }
}

impl From<LABColor> for LUVColor {
    fn from(value: LABColor) -> Self {
        let c1: XYZColor = value.into();
        c1.into()
    }
}

impl From<LCHColor> for LABColor {
    #[inline(always)]
    fn from(value: LCHColor) -> Self {
        let l = value.l;
        let c = value.c;
        let h = value.h;

        let a = c * h.cos();
        let b = c * h.sin();

        Self { l, a, b }
    }
}

impl From<LCHColor> for XYZColor {
    fn from(value: LCHColor) -> Self {
        let c1: LABColor = value.into();
        c1.into()
    }
}

impl From<LCHColor> for RGBColor {
    #[inline(always)]
    fn from(value: LCHColor) -> Self {
        let c1: LABColor = value.into();
        let c2: XYZColor = c1.into();
        c2.into()
    }
}

impl From<LCHColor> for LUVColor {
    fn from(value: LCHColor) -> Self {
        let c1: LABColor = value.into();
        let c2: XYZColor = c1.into();
        c2.into()
    }
}

impl From<LUVColor> for LABColor {
    fn from(value: LUVColor) -> Self {
        let c1: XYZColor = value.into();
        c1.into()
    }
}

impl From<LUVColor> for XYZColor {
    fn from(value: LUVColor) -> Self {
        let l = value.l;
        let u = value.u;
        let v = value.v;

        if l == 0.0 {
            return Self {
                x: 0.,
                y: 0.,
                z: 0.,
            };
        }

        let uprime = (u / (13. * l)) + UPRIME_N;
        let vprime = (v / (13. * l)) + VPRIME_N;

        let y = if l <= 8.0 {
            l / *KAPPA
        } else {
            ((l + 16.) / 116.0).powf(3.)
        };

        let x = y * ((9. * uprime) / (4. * vprime));
        let z = y * ((12. - (3. * uprime) - (20. * vprime)) / (4. * vprime));

        Self { x, y, z }
    }
}

impl From<LUVColor> for RGBColor {
    fn from(value: LUVColor) -> Self {
        let c1: XYZColor = value.into();
        c1.into()
    }
}

impl From<LUVColor> for LCHColor {
    fn from(value: LUVColor) -> Self {
        let c1: XYZColor = value.into();
        let c2: LABColor = c1.into();
        c2.into()
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
pub enum ColorSpace {
    rgb = 0,
    xyz = 1,
    lab = 2,
    lch = 3,
    luv = 4,
}

impl<'py> FromPyObject<'py> for ColorSpace {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        let val: i64 = ob.extract()?;
        match val {
            0 => Ok(ColorSpace::rgb),
            1 => Ok(ColorSpace::xyz),
            2 => Ok(ColorSpace::lab),
            3 => Ok(ColorSpace::lch),
            4 => Ok(ColorSpace::luv),
            _ => Err(PyValueError::new_err(format!(
                "Unknown color enum value {}",
                val
            ))),
        }
    }
}

pub fn convert(c: (f64, f64, f64), src: ColorSpace, dst: ColorSpace) -> (f64, f64, f64) {
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

#[cfg(test)]
mod tests {
    use super::*;

    // fn tests() -> Vec<(Color, Color)> {
    //     vec![
    //         ((0., 0., 0.).into(), (0., 0., 0.).into()),
    //         ((1.0, 0., 0.).into(), (53.2, 104.6, 0.7).into()),
    //         (
    //             (0.392156, 0.776470, 0.164705).into(),
    //             (71.7, 83.5, 2.3).into(),
    //         ),
    //         (
    //             (0.0392, 0.1960, 0.3529).into(),
    //             (20.3517, 27.8757, -1.4612).into(),
    //         ),
    //         (
    //             (0.0456, 0.1929, 0.3941).into(),
    //             (20.8945, 34.9429, -1.3244).into(),
    //         ),
    //         ((1.0, 1.0, 1.0).into(), (100., 0., 2.8).into()),
    //     ]
    // }

    #[allow(dead_code)]
    fn color_near(a: Color, b: Color, tol: (f64, f64, f64)) -> bool {
        let c1: (f64, f64, f64) = a.into();
        let c2: (f64, f64, f64) = b.into();

        if (c1.0 - c2.0).abs() > tol.0 {
            return false;
        }

        if (c1.1 - c2.1).abs() > tol.1 {
            return false;
        }

        if (c1.2 - c2.2).abs() > tol.2 {
            return false;
        }

        true
    }

    // #[test]
    // fn test_lch_to_rgb() {
    //     let tests = tests();
    //     for (rgb, lch) in tests {
    //         let argb = lch_to_rgb(lch);
    //         assert!(color_near(argb, rgb, (1.0, 1.0, 0.1)));
    //     }
    // }
}
