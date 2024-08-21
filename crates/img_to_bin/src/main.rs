use std::{
    fs::{create_dir, metadata, File},
    io::{ErrorKind, Write},
    path::{Path, PathBuf},
};

use byteorder::{BigEndian, WriteBytesExt};
use clap::{Parser, Subcommand};
use maze_exit_bin_common::{read_maze, BinaryReaderCell, MAZE_BINARY_READER_HEADER};
use maze_exit_lib::{maze::Maze, position::Pos};
use rayon::prelude::*;
use zstd::Encoder;

use BinaryReaderCell::*;

const BINARY_EXT: &str = "bin";

fn set(vec: &mut [u8], pos: Pos, cell: BinaryReaderCell, maze: &Maze) {
    let index = (pos.y as usize) * (maze.width() as usize) + (pos.x as usize);

    let i = index / 4;
    let r = index % 4;

    vec[i] |= u8::from(cell) << ((3 - r) * 2);
}

#[derive(Parser, Debug)]
#[command(author = "Davide C. <davide.canton5@gmail.com>", version = "1.0")]
struct Args {
    #[command(subcommand)]
    subcmd: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    File { src: PathBuf, dst: Option<PathBuf> },
    Dir { src: PathBuf, dst: Option<PathBuf> },
}

fn translate_single_file(img: &Path, out: &Path) -> Result<(), anyhow::Error> {
    let maze = read_maze(img)?;

    let mut file = File::create(out)?;
    write!(file, "{}", MAZE_BINARY_READER_HEADER)?;

    let mut encoder = Encoder::new(file, 0)?.auto_finish();

    encoder.write_u32::<BigEndian>(maze.width())?;
    encoder.write_u32::<BigEndian>(maze.height())?;

    let size = (maze.width() as f64 * maze.height() as f64) / 4.0;
    let mut maze_data = vec![0; size.ceil() as usize];

    for w in maze.walls() {
        set(&mut maze_data, w, Wall, &maze);
    }

    set(&mut maze_data, maze.start(), Start, &maze);
    set(&mut maze_data, maze.goal(), Goal, &maze);

    encoder.write_all(&maze_data)?;

    Ok(())
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    match args.subcmd {
        Commands::File { src, dst } => handle_file(src, dst),
        Commands::Dir { src, dst } => handle_dir(src, dst),
    }
}

fn handle_dir(src: PathBuf, dst: Option<PathBuf>) -> Result<(), anyhow::Error> {
    match metadata(&src) {
        Ok(metadata) if metadata.is_file() => {
            anyhow::bail!("Source is a file");
        }
        Err(e) if e.kind() == ErrorKind::NotFound => {
            anyhow::bail!("Source does not exist");
        }
        Err(e) => anyhow::bail!(e),
        _ => {}
    }

    let dst = dst.unwrap_or_else(|| src.clone());

    if let Err(e) = create_dir(&dst) {
        if e.kind() != ErrorKind::AlreadyExists {
            anyhow::bail!(e);
        }
    }

    let files: Vec<_> = src.read_dir()?.flatten().collect();

    files
        .into_par_iter()
        .filter(|f| f.file_type().map(|t| t.is_file()).unwrap_or_default())
        .map(|f| f.path())
        .filter(|f| f.extension().map(|v| v != BINARY_EXT).unwrap_or_default())
        .for_each(|fp| {
            let dst = &dst.join(fp.with_extension(BINARY_EXT).file_name().unwrap());
            println!("Translating {:?} to {:?}", fp, dst);

            if let Err(e) = translate_single_file(&fp, dst) {
                eprintln!("Error translating {:?}: {}", fp, e);
            }
        });

    Ok(())
}

fn handle_file(src: PathBuf, dst: Option<PathBuf>) -> Result<(), anyhow::Error> {
    match metadata(&src) {
        Ok(metadata) if metadata.is_dir() => {
            anyhow::bail!("Source is a directory");
        }
        Err(e) if e.kind() == ErrorKind::NotFound => {
            anyhow::bail!("Source does not exist");
        }
        Err(e) => anyhow::bail!(e),
        _ => {}
    }

    let dst = dst.unwrap_or_else(|| src.with_extension(BINARY_EXT));
    translate_single_file(&src, &dst)
}
