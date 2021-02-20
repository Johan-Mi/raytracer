use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Args {
    /// Width of the generated image
    #[structopt(short, long)]
    pub width: usize,
    /// Height of the generated image
    #[structopt(short, long)]
    pub height: usize,

    /// Number of rays per pixel
    #[structopt(short, long, default_value = "50")]
    pub samples: usize,

    /// Gamma value used for color correction
    #[structopt(short, long, default_value = "2.2")]
    pub gamma: f32,

    /// Don't print anything
    #[structopt(short, long)]
    pub quiet: bool,

    /// The file that the image will be written to
    #[structopt(short, default_value = "image.ppm")]
    pub outfile: PathBuf,

    /// The file describing the scene to be rendered
    #[structopt(name = "FILE")]
    pub infile: PathBuf,
}
