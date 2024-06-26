"""test color_operations.utils.py."""
# The MIT License (MIT)
#
# Copyright (c) 2015 Mapbox
#
# Permission is hereby granted, free of charge, to any person obtaining a copy
# of this software and associated documentation files (the "Software"), to deal
# in the Software without restriction, including without limitation the rights
# to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
# copies of the Software, and to permit persons to whom the Software is
# furnished to do so, subject to the following conditions:
#
# The above copyright notice and this permission notice shall be included in all
# copies or substantial portions of the Software.
#
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
# IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
# FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
# AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
# LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
# OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
# SOFTWARE.

import numpy as np
import pytest

from color_operations.utils import magick_to_rio, math_type, scale_dtype, to_math_type


@pytest.fixture
def arr():
    return (
        np.array(
            [[[1, 2], [3, 4]], [[5, 6], [7, 8]], [[9, 10], [11, 12]], [[0, 0], [0, 0]]]
        ).astype("uint8")
        * 10
    )


def test_to_math_type(arr):
    x = to_math_type(arr)
    assert x.dtype == math_type
    assert x.max() <= 1.0
    assert x.min() >= 0.0


def test_scale_dtype():
    arr = np.array([0.0, 1.0]).astype(math_type)
    x = scale_dtype(arr, "uint8")
    assert x.max() == 255
    assert x.min() == 0
    x = scale_dtype(arr, "uint16")
    assert x.max() == 65535
    assert x.min() == 0


def test_scale_round_trip(arr):
    x = to_math_type(arr)
    y = scale_dtype(x, arr.dtype)
    assert np.array_equal(arr, y)


def test_magick_to_rio():
    ops = magick_to_rio(
        "-channel B -sigmoidal-contrast 4 -gamma 0.95 "
        "-channel r -gamma 1.10 "
        "-channel rgb -sigmoidal-contrast 1x55% "
        "-channel G -gamma 0.9 "
        "-modulate 100,125 "
        "+channel -sigmoidal-contrast 3,40% "
        "-modulate 222,135 "
    )

    assert ops == " ".join(
        [
            "sigmoidal B 4 0.5",
            "gamma B 0.95",
            "gamma R 1.10",
            "sigmoidal RGB 1 0.55",
            "gamma G 0.9",
            "saturation 1.25",
            "sigmoidal RGB 3 0.4",
            "saturation 1.35",
        ]
    )
