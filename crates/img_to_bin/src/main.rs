use std::{fs::File, io::Write};

use byteorder::{BigEndian, WriteBytesExt};
use clap::Parser;
use maze_exit_bin_common::{read_maze, BinaryReaderCell, MAZE_BINARY_READER_HEADER};
use maze_exit_lib::{maze::Maze, position::Pos};

fn set(vec: &mut [u8], pos: Pos, cell: BinaryReaderCell, maze: &Maze) {
    let index = (pos.y as usize) * (maze.width() as usize) + (pos.x as usize);

    let i = index / 4;
    let r = index % 4;

    let v = &mut vec[i];
    *v |= u8::from(cell) << ((3 - r) * 2);
}

#[derive(Parser, Debug)]
#[command(author = "Davide C. <davide.canton5@gmail.com>", version = "1.0")]
struct Args {
    img_path: String,
    out_path: String,
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    let img = args.img_path;
    let out = args.out_path;
    let maze = read_maze(img)?;

    let mut file = File::create(out)?;
    write!(file, "{}", MAZE_BINARY_READER_HEADER)?;

    let mut encoder = zstd::stream::Encoder::new(file, 0)?.auto_finish();

    encoder.write_u32::<BigEndian>(maze.width())?;
    encoder.write_u32::<BigEndian>(maze.height())?;

    let size = (maze.width() as f64 * maze.height() as f64) / 4.0;
    let mut maze_data = vec![0; size.ceil() as usize];

    for w in maze.walls() {
        set(&mut maze_data, w, BinaryReaderCell::Wall, &maze);
    }

    set(&mut maze_data, maze.start(), BinaryReaderCell::Start, &maze);
    set(&mut maze_data, maze.goal(), BinaryReaderCell::Goal, &maze);

    encoder.write_all(&maze_data)?;

    Ok(())
}
