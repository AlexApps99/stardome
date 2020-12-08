from sys import argv
if __name__ == "__main__" and len(argv) == 3:
    import numpy as np
    from PIL import Image
    import py360convert
    img = np.array(Image.open(argv[1]))
    if len(img.shape) == 2:
        img = img[..., None]
    out = np.concatenate(py360convert.e2c(img, face_w=256, cube_format="list"))
    Image.fromarray(out.astype(np.uint8)).save(argv[2])
