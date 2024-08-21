use clap::{command, Parser};

#[derive(Parser, Debug)]
#[command(
    author="Davide C. <davide.canton5@gmail.com>", 
    version="1.0", 
    about="A* algorithm for solving mazes", 
    long_about = None
)]
pub struct Args {
    pub img_path: String,
}

pub fn parse_args<T: Parser>() -> T {
    T::parse()
}
