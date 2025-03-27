"""Setup script."""

# The MIT License (MIT)

# Copyright (c) 2015 Mapbox

# Permission is hereby granted, free of charge, to any person obtaining a copy
# of this software and associated documentation files (the "Software"), to deal
# in the Software without restriction, including without limitation the rights
# to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
# copies of the Software, and to permit persons to whom the Software is
# furnished to do so, subject to the following conditions:

# The above copyright notice and this permission notice shall be included in all
# copies or substantial portions of the Software.

# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
# IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
# FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
# AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
# LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
# OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
# SOFTWARE.

import sys

from setuptools import find_packages, setup
from setuptools.extension import Extension

# Use Cython if available.
try:
    from Cython.Build import cythonize
except ImportError:
    cythonize = None

include_dirs = []
try:
    import numpy

    include_dirs.append(numpy.get_include())
except ImportError:
    print("Numpy and its headers are required to run setup(). Exiting.")
    sys.exit(1)


with open("README.md") as f:
    readme = f.read()


if cythonize and "clean" not in sys.argv:
    ext_modules = cythonize(
        [
            Extension(
                "color_operations.colorspace",
                ["color_operations/colorspace.pyx"],
                extra_compile_args=["-O2"],
            )
        ]
    )
else:
    ext_modules = [
        Extension("color_operations.colorspace", ["color_operations/colorspace.c"])
    ]


setup(
    name="color-operations",
    description="Apply basic color-oriented image operations.",
    long_description=readme,
    long_description_content_type="text/markdown",
    python_requires=">=3.9",
    classifiers=[
        "Development Status :: 5 - Production/Stable",
        "Intended Audience :: Developers",
        "Intended Audience :: Science/Research",
        "License :: OSI Approved :: MIT License",
        "Programming Language :: Cython",
        "Programming Language :: Python :: 3.9",
        "Programming Language :: Python :: 3.10",
        "Programming Language :: Python :: 3.11",
        "Programming Language :: Python :: 3.12",
        "Programming Language :: Python :: 3.13",
        "Topic :: Multimedia :: Graphics :: Graphics Conversion",
        "Topic :: Scientific/Engineering :: GIS",
    ],
    keywords="",
    author="Charlie Loyd",
    author_email="charlie@mapbox.com",
    maintainer="Vincent Sarago",
    maintainer_email="vincent@developmentseed.com",
    url="https://github.com/vincentsarago/color-operations",
    license="MIT",
    packages=find_packages(exclude=["tests"]),
    include_package_data=True,
    zip_safe=False,
    install_requires=["numpy"],
    ext_modules=ext_modules,
    include_dirs=include_dirs,
    extras_require={
        "test": [
            "pytest",
            "colormath==2.0.2",
            "pytest-cov",
        ],
        "dev": [
            "pre-commit",
            "bump-my-version",
        ],
    },
)
