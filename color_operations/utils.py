"""Color utilities."""

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

import re

import numpy

# The type to be used for all intermediate math
# operations. Should be a float because values will
# be scaled to the range 0..1 for all work.

math_type = numpy.float64

epsilon = numpy.finfo(math_type).eps


def to_math_type(arr):
    """Convert an array from native integer dtype range to 0..1 scaling down linearly."""
    max_int = numpy.iinfo(arr.dtype).max
    return arr.astype(math_type) / max_int


def scale_dtype(arr, dtype):
    """Convert an array from 0..1 to dtype, scaling up linearly."""
    max_int = numpy.iinfo(dtype).max
    return (arr * max_int).astype(dtype)


def magick_to_operations(convert_opts):  # noqa: C901
    """Translate a limited subset of imagemagick convert commands to color-operations operations.

    Parameters
    ----------
    convert_opts: String, imagemagick convert options

    Returns
    -------
    operations string, ordered rio color operations

    """
    ops = []

    def set_band(x):
        global bands

        if x.upper() == "RGB":
            x = "RGB"

        bands = x.upper()

    set_band("RGB")

    def append_sig(arg):
        global bands

        args = list(filter(None, re.split("[,x]+", arg)))

        if len(args) == 1:
            args.append(0.5)

        elif len(args) == 2:
            args[1] = float(args[1].replace("%", "")) / 100.0

        ops.append("sigmoidal {} {} {}".format(bands, *args))

    def append_gamma(arg):
        global bands
        ops.append("gamma {} {}".format(bands, arg))

    def append_sat(arg):
        args = list(filter(None, re.split("[,x]+", arg)))
        # ignore args[0]
        # convert to proportion
        prop = float(args[1]) / 100
        ops.append("saturation {}".format(prop))

    nextf = None
    for part in convert_opts.strip().split(" "):
        if part == "-channel":
            nextf = set_band

        elif part == "+channel":
            set_band("RGB")
            nextf = None

        elif part == "-sigmoidal-contrast":
            nextf = append_sig

        elif part == "-gamma":
            nextf = append_gamma

        elif part == "-modulate":
            nextf = append_sat

        else:
            if nextf:
                nextf(part)
            nextf = None

    return " ".join(ops)


magick_to_rio = magick_to_operations
