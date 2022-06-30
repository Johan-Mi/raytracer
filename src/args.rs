use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct Args {
    /// Width of the generated image
    #[clap(short, long)]
    pub width: usize,
    /// Height of the generated image
    #[clap(short, long)]
    pub height: usize,

    /// Number of rays per pixel
    #[clap(short, long, default_value_t = 50)]
    pub samples: usize,

    /// Gamma value used for color correction
    #[clap(short, long, default_value_t = 2.2)]
    pub gamma: f32,

    /// Don't print anything
    #[clap(short, long)]
    pub quiet: bool,

    /// File that the image will be written to
    #[clap(short, default_value = "image.ppm")]
    pub outfile: PathBuf,

    /// The file describing the scene to render
    #[clap(name = "FILE")]
    pub infile: PathBuf,
}
