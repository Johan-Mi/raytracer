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

    /// Don't print anything
    #[structopt(short, long)]
    pub quiet: bool,
}
