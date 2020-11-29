#!/bin/sh
convert world.png -resize 1080x540 warudo.png
convert bathymetry.jpg -channel B -separate -resize 1080x540 bath.png
