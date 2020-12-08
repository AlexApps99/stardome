#!/bin/bash
cd "$(dirname $0)"
mkdir -p gen/
convert src/earth_albedo.png -resize 1080x540 gen/earth_albedo.png
convert src/earth_bathymetry.jpg -channel B -separate -resize 1080x540 gen/earth_bathymetry.png
python to_cube.py src/milky_way.jpg gen/milky_way.png
