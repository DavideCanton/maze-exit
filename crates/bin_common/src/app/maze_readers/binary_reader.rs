use anyhow::Result;
use byteorder::{BigEndian, ReadBytesExt};
use maze_exit_lib::{
    maze::Maze,
    maze_builder::MazeBuilder,
    position::{MyFuncs, Pos},
};
use std::{
    io::{Read, Seek},
    mem,
};

use super::MazeReader;

pub const HEADER: &str = "MAZE";

pub(crate) struct MazeBinaryReader;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Cell {
    Empty = 0,
    Start = 1,
    Goal = 2,
    Wall = 3,
}

impl TryFrom<u8> for Cell {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value < 4 {
            Ok(unsafe { mem::transmute::<u8, Cell>(value) })
        } else {
            Err(format!("Invalid value provided {value}"))
        }
    }
}

impl From<Cell> for u8 {
    fn from(value: Cell) -> Self {
        value as u8
    }
}

impl MazeBinaryReader {
    pub(crate) fn check(&self, reader: &mut (impl Read + Seek)) -> Result<bool> {
        let mut buf = [0; HEADER.len()];
        reader.read_exact(&mut buf)?;
        let header = String::from_utf8_lossy(&buf);
        if header != HEADER {
            reader.rewind()?;
            Ok(false)
        } else {
            Ok(true)
        }
    }
}

impl MazeReader for MazeBinaryReader {
    fn read_maze(&self, mut reader: impl Read + Seek) -> Result<Maze> {
        let mut builder = MazeBuilder::new();

        let w = reader.read_u32::<BigEndian>()?;
        let h = reader.read_u32::<BigEndian>()?;
        let mut x = 0;
        let mut y = 0;
        let mut remaining = w * h;

        builder = builder.width(w).height(h);

        for b in reader.bytes() {
            for v in read_cell(b?) {
                if remaining == 0 {
                    break;
                }
                remaining -= 1;
                let pos = Pos::convert(x, y);
                match v {
                    Cell::Wall => builder = builder.add_wall(pos),
                    Cell::Start => builder = builder.start(pos),
                    Cell::Goal => builder = builder.goal(pos),
                    _ => (),
                }
                x += 1;
                if x == w {
                    x = 0;
                    y += 1;
                }
            }
        }

        builder.build()
    }
}

fn read_cell(b: u8) -> [Cell; 4] {
    [
        ((b >> 6) & 0b11).try_into().unwrap(),
        ((b >> 4) & 0b11).try_into().unwrap(),
        ((b >> 2) & 0b11).try_into().unwrap(),
        (b & 0b11).try_into().unwrap(),
    ]
}
