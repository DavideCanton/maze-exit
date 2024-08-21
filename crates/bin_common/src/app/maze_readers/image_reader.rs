use anyhow::{Context, Result};
use image::io::Reader;
use image::Rgb;
use maze_exit_lib::position::{PosFunctions, Position};
use std::io::{BufReader, Read, Seek};

use maze_exit_lib::maze::Maze;
use maze_exit_lib::maze_builder::MazeBuilder;

use super::MazeReader;

const THRESHOLD: f64 = 250.0;

pub(crate) struct MazeImageReader;

impl MazeReader for MazeImageReader {
    fn read_maze(&self, reader: impl Read + Seek) -> Result<Maze> {
        let mut reader = Reader::new(BufReader::new(reader)).with_guessed_format()?;
        reader.no_limits();
        let image = reader.decode().context("Failed image load")?.to_rgb8();
        let mut builder = MazeBuilder::new();
        builder = builder.width(image.width()).height(image.height());

        for x in 0..image.width() {
            for y in 0..image.height() {
                let p = *image.get_pixel(x, y);
                let pos = Position::try_convert(x, y)?;

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

fn is_goal(pixel: Rgb<u8>) -> bool {
    color_difference(pixel.0, [0, 255, 0]) < THRESHOLD
}

fn is_start(pixel: Rgb<u8>) -> bool {
    color_difference(pixel.0, [255, 0, 0]) < THRESHOLD
}

fn is_wall(pixel: Rgb<u8>) -> bool {
    color_difference(pixel.0, [0, 0, 0]) < THRESHOLD
}

fn color_difference(p1: [u8; 3], p2: [u8; 3]) -> f64 {
    let [r1, g1, b1] = p1;
    let [r2, g2, b2] = p2;
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
        let pixel = Rgb::from([237u8, 28, 36]);
        assert!(is_start(pixel));

        let pixel = Rgb::from([10u8, 28, 36]);
        assert!(!is_start(pixel));
    }

    #[test]
    fn detect_green() {
        let pixel = Rgb::from([34u8, 177, 76]);
        assert!(is_goal(pixel));
    }
}
