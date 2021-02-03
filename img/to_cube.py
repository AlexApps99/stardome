from sys import argv
import numpy as np
from PIL import Image
import py360convert

def ccw(m):
    return np.rot90(m, axes=(1,0))

def to_gl_format(out):
    o = np.concatenate([ccw(np.flipud(out["F"])), ccw(out["B"]), np.flipud(out["L"]), out["R"], ccw(out["U"]), ccw(np.flipud(out["D"]))])
    return Image.fromarray(o.astype(np.uint8))

if __name__ == "__main__" and len(argv) == 4 and argv[2].isdecimal():
    img = np.array(Image.open(argv[1]))
    if len(img.shape) == 2:
        img = img[..., None]
    out = py360convert.e2c(img, face_w=int(argv[2]), cube_format="dict")
    to_gl_format(out).save(argv[3])
