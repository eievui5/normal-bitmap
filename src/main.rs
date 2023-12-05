//! A Rust program to convert greyscale images into a normal map of u8s.

use clap::Parser;
use std::f64::consts::TAU;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Path to the input image.
    #[clap(short, long, value_parser, value_name = "PATH")]
    input: PathBuf,

    /// Path to map output.
    #[clap(short, long, value_parser, value_name = "PATH")]
    map: PathBuf,

    /// Path to vector lookup table output.
    #[clap(short, long, value_parser, value_name = "PATH")]
    vectors: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();

    let map = image::open(&cli.input).unwrap().into_luma8();
    let width = map.width();
    let height = map.height();

    let mut vectors = vec![0; (width * height) as usize];

    for y in 0..height {
        for x in 0..width {
            // Since the map is greyscale, pixel is just a u8!
            let pixel = map.get_pixel(x, y);
            // Invert colors so that white is 0.
            vectors[(x + y * width) as usize] = u8::MAX - pixel.0[0];
        }
    }

    if let Err(msg) = fs::write(&cli.map, &vectors) {
        eprintln!("failed to write to {}: {msg}", cli.map.display());
    }

    if let Some(vectors_path) = cli.vectors {
        // Also output a lookup table to convert u8s to i8 vectors.
        let mut vector_lookup = [0; 256 * 2];

        for i in 0..256 {
            let x = i as f64 / 256.0;
            let convert = |x: f64| (x * 128.0).floor() as i8 as u8;

            vector_lookup[i * 2] = convert((x * TAU).sin());
            vector_lookup[i * 2 + 1] = convert((x * TAU).cos());
        }

        if let Err(msg) = fs::write(&vectors_path, &vectors) {
            eprintln!("failed to write to {}: {msg}", vectors_path.display());
        }
    }
}
