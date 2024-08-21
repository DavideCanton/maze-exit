use anyhow::Result;
use image::{ImageBuffer, ImageFormat, Rgb};
use maze_exit_lib::{generator::MazePath, maze::Maze, position::Position};
use std::io::{Seek, Write};

use super::{MazeWriter, MazeWriterWithPath};

pub struct ImageMazeWriter;

impl ImageMazeWriter {
    fn set_pixel(&self, image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, pos: Position, color: Rgb<u8>) {
        image.put_pixel(pos.x as u32, pos.y as u32, color);
    }

    fn fill_image(&self, maze: &Maze) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let mut image = ImageBuffer::from_pixel(maze.width(), maze.height(), Rgb([255, 255, 255]));

        let black = Rgb([0, 0, 0]);
        for w in maze.walls() {
            self.set_pixel(&mut image, w, black);
        }

        self.set_pixel(&mut image, maze.start(), Rgb([255, 0, 0]));
        self.set_pixel(&mut image, maze.goal(), Rgb([0, 255, 0]));

        image
    }
}

impl<W: Write + Seek> MazeWriter<W> for ImageMazeWriter {
    fn write_maze(&self, maze: &Maze, mut writer: W) -> Result<()> {
        let image = self.fill_image(maze);
        image.write_to(&mut writer, ImageFormat::Png)?;
        Ok(())
    }
}

impl<W: Write + Seek> MazeWriterWithPath<W> for ImageMazeWriter {
    fn write_maze_with_path(&self, maze: &Maze, path: &MazePath, mut writer: W) -> Result<()> {
        let mut image = self.fill_image(maze);

        let blue = Rgb([0, 0, 255]);
        for &p in path.iter() {
            self.set_pixel(&mut image, p, blue);
        }

        image.write_to(&mut writer, ImageFormat::Png)?;
        Ok(())
    }
}
