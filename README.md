[![](https://github.com/azazdeaz/homography/workflows/Docs/badge.svg)](https://azazdeaz.github.io/homography/homography/)

## WIP

Rust reimplementation of the [OpenCV's homography estimation algorithm](https://github.com/opencv/opencv/modules/calib3d/src/fundam.cpp).

Run example with: `cargo run --release --example from_images -- --image1 ./test-data/image1.png --image2 ./test-data/image2.png`

Run the demo app with: `cargo run --release --bin demo` (add `--features opencv` to see the resutls of OpenCV's findHomography() as well)

https://user-images.githubusercontent.com/2298371/146600035-a88b0753-84b8-49d7-932f-e876572ee02d.mp4

