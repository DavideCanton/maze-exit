use anyhow::Result;
use maze_exit_lib::{generator::MazePath, maze::Maze};

pub mod binary_writer;
pub mod image_writer;

pub trait MazeWriter<W> {
    fn write_maze(&self, maze: &Maze, writer: W) -> Result<()>;
}

pub trait MazeWriterWithPath<W>: MazeWriter<W> {
    fn write_maze_with_path(&self, maze: &Maze, path: &MazePath, writer: W) -> Result<()>;
}
