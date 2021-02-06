mod checkerboard;
mod color;
mod drawable;

use checkerboard::Checkerboard;
use drawable::Drawable;

fn main() {
    let board = Checkerboard::new();
    board.write_ppm("image.ppm");
}
