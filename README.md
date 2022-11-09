# color-operations

<p align="center">
  <p align="center">Apply basic color-oriented image operations.</p>
</p>

<p align="center">
  <a href="https://github.com/vincentsarago/color-operations/actions?query=workflow%3ACI" target="_blank">
      <img src="https://github.com/vincentsarago/color-operations/workflows/CI/badge.svg" alt="Test">
  </a>
  <a href="https://codecov.io/gh/vincentsarago/color-operations" target="_blank">
      <img src="https://codecov.io/gh/vincentsarago/color-operations/branch/main/graph/badge.svg" alt="Coverage">
  </a>
  <a href="https://pypi.org/project/color-operations" target="_blank">
      <img src="https://img.shields.io/pypi/v/color-operations?color=%2334D058&label=pypi%20package" alt="Package version">
  </a>
  <a href="https://github.com/vincentsarago/color-operations/blob/main/LICENSE" target="_blank">
      <img src="https://img.shields.io/github/license/vincentsarago/color-operations.svg" alt="license">
  </a>
</p>

Lightweight version of [**rio-color**](https://github.com/mapbox/rio-color) but removing rasterio dependency.

## Install

You can install color-operations using pip

```
pip install -U pip
pip install color-operations
```

Build from source

```
git checkout https://github.com/vincentsarago/color-operations.git
cd color-operations
pip install -U pip
pip install -e .
```

## Operations

**Gamma** adjustment adjusts RGB values according to a power law, effectively brightening or darkening the midtones. It can be very effective in satellite imagery for reducing atmospheric haze in the blue and green bands.

**Sigmoidal** contrast adjustment can alter the contrast and brightness of an image in a way that
matches human's non-linear visual perception. It works well to increase contrast without blowing out the very dark shadows or already-bright parts of the image.

**Saturation** can be thought of as the "colorfulness" of a pixel. Highly saturated colors are intense and almost cartoon-like, low saturation is more muted, closer to black and white. You can adjust saturation independently of brightness and hue but the data must be transformed into a different color space.

Ref https://github.com/mapbox/rio-color/blob/master/README.md

## Examples

#### Sigmoidal

Contrast

![sigmoidal_contrast](img/sigmoidal_contrast.jpg)

Bias

![sigmoidal_bias](img/sigmoidal_bias.jpg)

#### Gamma

Red

![gamma_red](img/gamma_red.jpg)

Green

![gamma_green](img/gamma_green.jpg)

Blue

![gamma_blue](img/gamma_blue.jpg)

#### Saturation

![saturation](img/saturation.jpg)


#### Combinations of operations

![combos](img/combos.jpg)

Ref https://github.com/mapbox/rio-color/blob/master/README.md

## Python API

#### `color_operations.operations`

The following functions accept and return numpy `ndarrays`. The arrays are assumed to be scaled 0 to 1. In some cases, the input array is assumed to be in the RGB colorspace.

All arrays use rasterio ordering with the shape as (bands, columns, rows). Be aware that other image processing software may use the (columns, rows, bands) axis order.

* `sigmoidal(arr, contrast, bias)`
* `gamma(arr, g)`
* `saturation(rgb, proportion)`
* `simple_atmo(rgb, haze, contrast, bias)`

The `color_operations.operations.parse_operations` function takes an *operations string* and
returns a list of python functions which can be applied to an array.

```python
from color_operations import parse_operations

ops = "gamma b 1.85, gamma rg 1.95, sigmoidal rgb 35 0.13, saturation 1.15"

assert arr.shape[0] == 3
assert arr.min() >= 0
assert arr.max() <= 1

for func in parse_operations(ops):
    arr = func(arr)
```

This provides a tiny domain specific language (DSL) to allow you
to compose ordered chains of image manipulations using the above operations.
For more information on operation strings, see the `rio color` command line help.

#### `color_operations.colorspace`

The `colorspace` module provides functions for converting scalars and numpy arrays between different colorspaces.

```python
from color_operations.colorspace import ColorSpace as cs  # enum defining available color spaces
from color_operations.colorspace import convert, convert_arr

convert_arr(array, src=cs.rgb, dst=cs.lch) # for arrays
...

convert(r, g, b, src=cs.rgb, dst=cs.lch)  # for scalars
...

dict(cs.__members__)  # can convert to/from any of these color spaces
{
    'rgb': <ColorSpace.rgb: 0>,
    'xyz': <ColorSpace.xyz: 1>,
    'lab': <ColorSpace.lab: 2>,
    'lch': <ColorSpace.lch: 3>,
    'luv': <ColorSpace.luv: 4>
}
```
