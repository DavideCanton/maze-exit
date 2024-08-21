use anyhow::Result;
use byteorder::{BigEndian, WriteBytesExt};
use maze_exit_lib::{maze::Maze, position::Position};
use std::io::Write;
use zstd::Encoder;

use crate::{BinaryReaderCell, MAZE_BINARY_READER_HEADER};

use super::MazeWriter;
use BinaryReaderCell::*;

fn set(vec: &mut [u8], pos: Position, cell: BinaryReaderCell, maze: &Maze) {
    let index = (pos.y as usize) * (maze.width() as usize) + (pos.x as usize);

    let i = index / 4;
    let r = index % 4;

    vec[i] |= u8::from(cell) << ((3 - r) * 2);
}

pub struct BinaryMazeWriter;

impl<W: Write> MazeWriter<W> for BinaryMazeWriter {
    fn write_maze(&self, maze: &Maze, mut writer: W) -> Result<()> {
        write!(writer, "{}", MAZE_BINARY_READER_HEADER)?;

        let mut encoder = Encoder::new(writer, 0)?.auto_finish();

        encoder.write_u32::<BigEndian>(maze.width())?;
        encoder.write_u32::<BigEndian>(maze.height())?;

        let size = (maze.width() as f64 * maze.height() as f64) / 4.0;
        let mut maze_data = vec![0; size.ceil() as usize];

        for w in maze.walls() {
            set(&mut maze_data, w, Wall, maze);
        }

        set(&mut maze_data, maze.start(), Start, maze);
        set(&mut maze_data, maze.goal(), Goal, maze);

        encoder.write_all(&maze_data)?;

        Ok(())
    }
}
