// #[cfg(test)]
// mod test {
//     use approx::assert_relative_eq;

//     use crate::colorspace::Color;

//     use super::*;

//     #[test]
//     fn test_saturate_rgb() {
//         let rgb = Color::new(0.392156, 0.776470, 0.164705);
//         let out_rgb = saturate_single_rgb(rgb, 0.0);
//         assert_relative_eq!(out_rgb.one, out_rgb.two, epsilon = 0.1);
//         assert_relative_eq!(out_rgb.two, out_rgb.three, epsilon = 0.1);
//     }
// }
