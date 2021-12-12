use akaze::Akaze;
use bitarray::{BitArray, Hamming};
use cv_core::FeatureMatch;
use space::Knn;
use std::path::Path;
extern crate clap;
use clap::{App, Arg};

const LOWES_RATIO: f32 = 0.5;

type Descriptor = BitArray<64>;

fn image_to_kps(path: impl AsRef<Path>) -> (Vec<akaze::KeyPoint>, Vec<Descriptor>) {
    Akaze::sparse().extract_path(path).unwrap()
}

fn main() {
    let matches = App::new("Image example")
        .about("Tests homography estimation on an image pair")
        .arg(
            Arg::with_name("image1")
                .long("image1")
                .help("Path to image1")
                .default_value("./test-data/image1.png")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("image2")
                .long("image2")
                .help("Path to image2")
                .default_value("./test-data/image2.png")
                .takes_value(true),
        )
        .get_matches();

    pretty_env_logger::init_timed();

    // Extract features with AKAZE.
    println!("Extracting features");
    let (kps1, ds1) = image_to_kps(matches.value_of("image1").unwrap());
    let (kps2, ds2) = image_to_kps(matches.value_of("image2").unwrap());

    println!(
        "Foundn {} keyponits on image1 and {} on image2",
        ds1.len(),
        ds2.len()
    );

    // Perform matching.
    println!(
        "Running matching on {} and {} descriptors",
        ds1.len(),
        ds2.len()
    );
    let matches: Vec<_> = match_descriptors(&ds1, &ds2)
        .into_iter()
        .map(|(ix1, ix2)| {
            let a = nalgebra::Point2::new(kps1[ix1].point.0 as f64, kps1[ix1].point.1 as f64);
            let b = nalgebra::Point2::new(kps2[ix2].point.0 as f64, kps2[ix2].point.1 as f64);
            FeatureMatch(a, b)
        })
        .collect();
    println!("Finished matching with {} matches", matches.len());

    // Estimate homography
    let h = homography::find_homography_with_arrsac(&matches)
        .expect("Failed to find homography transform");
    println!("Result of find_homography_with_arrsac: {}", h.0);

    let h = homography::find_homography(matches).expect("Failed to find homography transform");
    println!("Result of find_homography {}", h);
}

fn match_descriptors(ds1: &[Descriptor], ds2: &[Descriptor]) -> Vec<(usize, usize)> {
    let two_neighbors = ds1
        .iter()
        .map(|d1| {
            let neighbors = space::LinearKnn {
                metric: Hamming,
                iter: ds2.iter(),
            }
            .knn(d1, 2);
            assert_eq!(neighbors.len(), 2, "there should be at least two matches");
            neighbors
        })
        .enumerate();
    let satisfies_lowes_ratio = two_neighbors.filter(|(_, neighbors)| {
        (neighbors[0].distance as f32) < neighbors[1].distance as f32 * LOWES_RATIO
    });
    satisfies_lowes_ratio
        .map(|(ix1, neighbors)| (ix1, neighbors[0].index))
        .collect()
}
