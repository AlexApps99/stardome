from http.server import test, SimpleHTTPRequestHandler
from functools import partial
import os
import shutil
from sys import argv
#import webbrowser

if __name__ == "__main__" and len(argv) == 2:
    root = argv[1].rsplit("/", 1)[0]
    profile = root.rsplit("/", 1)[-1]
    name = root + "/deps/"
    shutil.copy(name + "stardome.js", "./web/")
    shutil.copy(name + "stardome.wasm", "./web/")
    shutil.copy(name + "stardome.data", "./web/")
    if profile == "debug":
        shutil.copy(name + "stardome.wasm.map", "./web/")
    elif profile == "release":
        if os.path.exists("./web/stardome.wasm.map"):
            os.remove("./web/stardome.wasm.map")
    else:
        print(profile, "is weird")

    #webbrowser.open("http://0.0.0.0:8000/")
    test(partial(SimpleHTTPRequestHandler, directory="./web/"))
