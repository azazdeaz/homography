[![](https://github.com/azazdeaz/homography/actions/workflows/docs.yaml/badge.svg)](https://azazdeaz.github.io/homography/homography/) 
[![](https://github.com/azazdeaz/homography/actions/workflows/lint_and_test.yaml/badge.svg)](https://github.com/azazdeaz/homography/actions/workflows/lint_and_test.yaml)

# Homography
Rust reimplementation of [OpenCV's homography estimation algorithm](https://github.com/opencv/opencv/blob/a1143c4ea02afa7c45c2a1e86be431b81a83bcd1/modules/calib3d/src/fundam.cpp#L118).

### [Usage and Documentation](https://azazdeaz.github.io/homography/homography/)

### Demos

 - #### Find homography between to images:  
`cargo run --release --example from_images -- --image1 ./test-data/image1.png --image2 ./test-data/image2.png`

 - #### Fun little demo app: 
`cargo run --release --bin demo` (add `--features opencv` enable [opencv-rust](https://github.com/twistedfall/opencv-rust) and see the resutls with OpenCV's findHomography() as well)

https://user-images.githubusercontent.com/2298371/146600035-a88b0753-84b8-49d7-932f-e876572ee02d.mp4

