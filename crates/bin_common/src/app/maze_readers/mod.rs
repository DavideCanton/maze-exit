use std::{
    fs::File,
    io::{Read, Seek},
    path::Path,
};

use anyhow::Result;
use binary_reader::MazeBinaryReader;
use image_reader::MazeImageReader;
use maze_exit_lib::maze::Maze;

mod binary_reader;
mod image_reader;

pub use binary_reader::Cell as BinaryReaderCell;
pub use binary_reader::HEADER as MAZE_BINARY_READER_HEADER;

trait MazeReader {
    fn read_maze(&self, reader: impl Read + Seek) -> Result<Maze>;
}

pub fn read_maze<P: AsRef<Path>>(path: P) -> Result<Maze> {
    let mut reader = File::open(path)?;
    if MazeBinaryReader.check(&mut reader)? {
        MazeBinaryReader.read_maze(reader)
    } else {
        MazeImageReader.read_maze(reader)
    }
}
