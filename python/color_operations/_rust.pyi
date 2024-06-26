from __future__ import annotations

from enum import IntEnum
from typing import Tuple

import numpy as np
from numpy.typing import NDArray

class ColorSpace(IntEnum):
    rgb = 0
    xyz = 1
    lab = 2
    lch = 3
    luv = 4

def convert_arr(
    arr: NDArray[np.float64], src: ColorSpace, dst: ColorSpace
) -> NDArray[np.float64]: ...
def convert(
    one: float, two: float, three: float, src: ColorSpace, dst: ColorSpace
) -> Tuple[float, float, float]: ...
def saturate_rgb(arr: NDArray[np.float64], satmult: float) -> NDArray[np.float64]: ...
