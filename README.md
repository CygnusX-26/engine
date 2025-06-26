# Engine

A super basic graphics engine written completely from scratch that can render .objs i've been working on as both my first rust project (and a way to kill time at work).

## Run

```sh
RUST_LOG=info cargo run --release objects/shuttle.obj
```
You can replace `objects/shuttle.obj` with any .obj file.

Currently everything will be rendered SKYBLUE and polygons of degree >=3 are supported.

## Examples

![lamp](imgs/lamp.gif)
