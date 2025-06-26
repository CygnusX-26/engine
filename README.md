# Engine

A super basic graphics engine written completely from scratch that can render `.obj` and `.mtl` files. I've been working on this as both my first rust project and a way to kill time at work.

[example](imgs/sky.gif)

## Run

```sh
RUST_LOG=info cargo run --release objects/skyscraper.obj
```
You can replace `objects/skyscraper.obj` with any `.obj` file. 



Polygons:
 1. ff any degree >= 3 are suppored
 2. without a material will be rendered `SKYBLUE`
 3. with a material will use the corresponding `.mtl` definition.


> `.mtl` file paths are from the root of the project directory. For example `vp.mtl`.

## Credits

All sample meshes used in this project were sourced [here](https://people.sc.fsu.edu/~jburkardt/data/obj/obj.html)
