#!/bin/bash
cd "$(dirname $0)"
mkdir -p gen/
convert src/earth_albedo.png -resize 1080x540 gen/earth_albedo.png
convert src/earth_bathymetry.jpg -channel B -separate -resize 1080x540 gen/earth_bathymetry.png
convert src/starmap.exr -gamma 1.25 gen/starmap_2020_8k.png
convert src/moon_albedo.tif -resize 1080x540 gen/moon_albedo.png
python to_cube.py gen/starmap_2020_8k.png 1024 gen/milky_way.png
