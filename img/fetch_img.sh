#!/bin/bash
cd "$(dirname $0)"
mkdir -p src/
curl -o src/earth_bathymetry.jpg -s https://eoimages.gsfc.nasa.gov/images/imagerecords/73000/73963/gebco_08_rev_bath_3600x1800_color.jpg
curl -o src/earth_albedo.png -s https://eoimages.gsfc.nasa.gov/images/imagerecords/74000/74092/world.200407.3x5400x2700.png
curl -o src/earth_clouds.jpg -s https://eoimages.gsfc.nasa.gov/images/imagerecords/57000/57747/cloud_combined_2048.jpg
curl -o src/starmap.exr -s https://svs.gsfc.nasa.gov/vis/a000000/a004800/a004851/starmap_2020_8k.exr
curl -o src/moon_albedo.tif -s https://svs.gsfc.nasa.gov/vis/a000000/a004700/a004720/lroc_color_poles_4k.tif
