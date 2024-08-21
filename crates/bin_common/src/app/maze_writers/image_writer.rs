use anyhow::Result;
use image::{ImageBuffer, ImageFormat, Rgb};
use maze_exit_lib::{generator::MazePath, maze::Maze};
use std::io::{Seek, Write};

use super::{MazeWriter, MazeWriterWithPath};

pub struct ImageMazeWriter;

impl ImageMazeWriter {
    fn fill_image(&self, maze: &Maze) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let mut image = ImageBuffer::from_pixel(maze.width(), maze.height(), Rgb([255, 255, 255]));

        let black = Rgb([0, 0, 0]);
        for w in maze.walls() {
            image.put_pixel(w.x as u32, w.y as u32, black);
        }

        image.put_pixel(
            maze.start().x as u32,
            maze.start().y as u32,
            Rgb([255, 0, 0]),
        );

        image.put_pixel(maze.goal().x as u32, maze.goal().y as u32, Rgb([0, 255, 0]));

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

        for p in path.iter() {
            image.put_pixel(p.x as u32, p.y as u32, Rgb([0, 0, 255]));
        }

        image.write_to(&mut writer, ImageFormat::Png)?;
        Ok(())
    }
}
