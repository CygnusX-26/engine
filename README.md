# Engine

A super basic graphics engine written completely from scratch that can render `.obj` and `.mtl` files WITH texture mapping. I've been working on this as both my first rust project and a way to kill time at work.

![example](imgs/sky.gif)

## Run

```sh
Usage: engine [OPTIONS] --filename <FILENAME>

Options:
  -f, --filename <FILENAME>
  -n, --normals              flip all normals
  -h, --help                 Print help
  -V, --version              Print version
```

```sh
RUST_LOG=info cargo run --release -- --filename objects/cottage_obj.obj
```
You can replace `objects/cottage_obj.obj` with any `.obj` file. 



Polygons:
 1. ff any degree >= 3 are suppored
 2. without a material will be rendered `DIM`
 3. with a material will use the corresponding `.mtl` definition.

Check out my [blog post](https://b.neilhommes.xyz/2025/06/16/engine/) that talks a bit about the foundations of computer graphics.

> `.mtl` file paths are from the root of the project directory. For example `vp.mtl`.

## Credits

All sample meshes used in this project were sourced [here](https://people.sc.fsu.edu/~jburkardt/data/obj/obj.html)
