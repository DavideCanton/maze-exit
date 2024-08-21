use anyhow::Result;
use std::{
    fs::{create_dir, metadata, File},
    io::ErrorKind,
    path::{Path, PathBuf},
};

use clap::{Parser, Subcommand};
use rayon::prelude::*;

use maze_exit_bin_common::{read_maze, BinaryMazeWriter, MazeWriter};

const BINARY_EXT: &str = "bin";

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

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    match args.subcmd {
        Commands::File { src, dst } => handle_file(src, dst),
        Commands::Dir { src, dst } => handle_dir(src, dst),
    }
}

fn translate_single_file(fp: &Path, dst: &Path) -> Result<()> {
    let maze = read_maze(fp)?;
    let writer = File::open(dst)?;
    BinaryMazeWriter.write_maze(&maze, writer)
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
