use numpy::ndarray::ArrayView3;

pub fn saturate_rgb(arr: ArrayView3<'_, f64>, satmult: f64) {

}

// cpdef np.ndarray[FLOAT_t, ndim=3] saturate_rgb(np.ndarray[FLOAT_t, ndim=3] arr, double satmult):
//     """Convert array of RGB -> LCH, adjust saturation, back to RGB
//     A special case of convert_arr with hardcoded color spaces and
//     a bit of data manipulation inside the loop.
//     """
//     cdef double r, g, b
//     cdef color c_lch
//     cdef color c_rgb

//     if arr.shape[0] != 3:
//         raise ValueError("The 0th dimension must contain 3 bands")

//     I = arr.shape[1]
//     J = arr.shape[2]

//     cdef np.ndarray[FLOAT_t, ndim=3] out = np.empty(shape=(3, I, J))

//     for i in range(I):
//         for j in range(J):
//             r = arr[0, i, j]
//             g = arr[1, i, j]
//             b = arr[2, i, j]

//             c_lch = _convert(r, g, b, RGB, LCH)
//             c_lch.two *= satmult
//             c_rgb = _convert(c_lch.one, c_lch.two, c_lch.three, LCH, RGB)

//             out[0, i, j] = c_rgb.one
//             out[1, i, j] = c_rgb.two
//             out[2, i, j] = c_rgb.three

//     return out
