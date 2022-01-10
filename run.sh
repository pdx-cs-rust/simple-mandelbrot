#!/bin/sh
cargo run >mandelbrot.pgm
pnmtopng mandelbrot.pgm > mandelbrot.png
