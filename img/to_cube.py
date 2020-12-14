from sys import argv
if __name__ == "__main__" and len(argv) == 4 and argv[2].isdecimal():
    import numpy as np
    from PIL import Image
    import py360convert
    img = np.array(Image.open(argv[1]))
    if len(img.shape) == 2:
        img = img[..., None]
    out = py360convert.e2c(img, face_w=int(argv[2]), cube_format="dict")
    out = np.concatenate([np.flip(out["L"], 1), out["R"], np.flip(out["U"], (0, 1)), np.flip(out["D"], 1), np.flip(out["F"], 1), out["B"]])
    Image.fromarray(out.astype(np.uint8)).save(argv[3])
