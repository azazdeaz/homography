[![](https://github.com/azazdeaz/homography/workflows/Docs/badge.svg)](https://azazdeaz.github.io/homography/homography/)

## WIP

Rust reimplementation of the [OpenCV's homography estimation algorithm](https://github.com/opencv/opencv/modules/calib3d/src/fundam.cpp).

Run example with: `cargo run --release --example from_images -- --image1 ./test-data/image1.png --image2 ./test-data/image2.png`

Run the demo app with: `cargo run --release --bin demo` (add `--features opencv` to see the resutls of OpenCV's findHomography() as well)

https://user-images.githubusercontent.com/2298371/145678553-12e383df-9457-429a-87a5-631218978638.mp4



