"""color-operations"""

from .operations import gamma, parse_operations, saturation, sigmoidal
from .utils import magick_to_operations, scale_dtype, to_math_type

__version__ = "0.2.0"


__all__ = [
    "sigmoidal",
    "gamma",
    "saturation",
    "parse_operations",
    "to_math_type",
    "scale_dtype",
    "magick_to_operations",
]
