#!/bin/bash
cd "$(dirname $0)"
mkdir -p gen/
convert \( src/earth_albedo.png -resize 3600x1800 \) \( src/earth_bathymetry.jpg -channel B -separate -negate \) -compose CopyOpacity -composite gen/earth.png
convert src/starmap.exr -gamma 1.25 gen/starmap_2020_8k.png
convert src/moon_albedo.tif -resize 3600x1800 gen/moon.png
python to_cube.py gen/starmap_2020_8k.png 1024 gen/milky_way.png
