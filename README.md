[![Rust](https://github.com/kriogenia/ferrux_canvas/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/kriogenia/ferrux_canvas/actions/workflows/rust.yml)

# FerruX Canvas

Ferrux Canvas is an abstraction layer over the [Pixels](https://crates.io/crates/pixels) crate. It manages the pixel
buffer exposing simple operations to draw in the screen. In its current state it only works with 
[Winit](https://crates.io/crates/winit).

The list of current goals is listed in the repository's [project](https://github.com/kriogenia/ferrux_canvas/projects/1).

## About

The FerruX Canvas is a tool developed while creating the FerruXengine, an attempt of 3D graphics engine I was trying to
make. I made this canvas to ease the use of the *Pixels* buffer used by the engine. As the tool proved useful for 
other possible future projects I was thinking of, and a couple of mutuals of mine thought that they could use it too, 
it was decided to extract it as its own library.