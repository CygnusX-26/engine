# Engine

A super basic graphics engine i've been working on as both my first rust project (and a way to kill time at work).

![shapes](imgs/shapes.gif)
![demo](imgs/demo.gif)

## Run

`RUST_LOG=info cargo run`

## usage

In main.rs you can change the filename:
```rust
let filename = "objects/lamp.obj";
```
To any .obj file you want.

Currently everything will be rendered SKYBLUE and polygons of degree >=3 are supported.

