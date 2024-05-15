use anyhow::Result;
use image::io::Reader;
use image::{GenericImageView, Rgba};
use std::path::Path;

use maze_exit_lib::maze::Maze;
use maze_exit_lib::maze_builder::MazeBuilder;
use maze_exit_lib::position::Pos;

const THRESHOLD: f64 = 250.0;

pub trait MazeReader {
    fn read_maze(&self, path: &Path) -> Result<Maze>;
}

pub struct MazeImageReader;

impl MazeReader for MazeImageReader {
    fn read_maze(&self, path: &Path) -> Result<Maze> {
        let mut reader = Reader::open(path)?;
        reader.no_limits();
        let image = reader.decode()?;
        let mut builder = MazeBuilder::new();
        builder = builder.width(image.width()).height(image.height());

        for x in 0..image.width() {
            for y in 0..image.height() {
                let p = image.get_pixel(x, y);
                let pos = Pos::from(x, y).unwrap();

                if is_wall(p) {
                    builder = builder.add_wall(pos);
                } else if is_start(p) {
                    builder = builder.start(pos);
                } else if is_goal(p) {
                    builder = builder.goal(pos);
                }
            }
        }

        builder.build()
    }
}

fn is_goal(pixel: Rgba<u8>) -> bool {
    color_difference(pixel.0, [0, 255, 0, 0]) < THRESHOLD
}

fn is_start(pixel: Rgba<u8>) -> bool {
    color_difference(pixel.0, [255, 0, 0, 0]) < THRESHOLD
}

fn is_wall(pixel: Rgba<u8>) -> bool {
    color_difference(pixel.0, [0, 0, 0, 0]) < THRESHOLD
}

fn color_difference(p1: [u8; 4], p2: [u8; 4]) -> f64 {
    let [r1, g1, b1, _] = p1;
    let [r2, g2, b2, _] = p2;
    let r_mean = (r1 as f64 + r2 as f64) / 2.0;

    let r = r1 as f64 - r2 as f64;
    let g = g1 as f64 - g2 as f64;
    let b = b1 as f64 - b2 as f64;

    let r_sq = r * r;
    let g_sq = g * g;
    let b_sq = b * b;

    let r_coeff = 2.0 + r_mean / 256.0;
    let g_coeff = 4.0;
    let b_coeff = 2.0 + 255.0 / 256.0 - r_mean / 256.0;

    (r_coeff * r_sq + g_coeff * g_sq + b_coeff * b_sq).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_red() {
        let pixel = Rgba::from([237u8, 28, 36, 0]);
        assert!(is_start(pixel));

        let pixel = Rgba::from([10u8, 28, 36, 0]);
        assert!(!is_start(pixel));
    }

    #[test]
    fn detect_green() {
        let pixel = Rgba::from([34u8, 177, 76, 0]);
        assert!(is_goal(pixel));
    }
}
