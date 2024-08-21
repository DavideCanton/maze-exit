mod app;
mod display;

use std::time::Instant;

use anyhow::Result;
use maze_exit_lib::{
    algorithm::{a_star, Info, Message},
    channel::ChannelSender,
    generator::JpsGenerator,
    heuristics::MazeHeuristic,
    maze::Maze,
};

pub use app::args::{parse_args, Args};
pub use app::maze_readers::{read_maze, BinaryReaderCell, MAZE_BINARY_READER_HEADER};
pub use app::maze_writers::{
    binary_writer::BinaryMazeWriter, image_writer::ImageMazeWriter, MazeWriter, MazeWriterWithPath,
};
pub use display::Displayer;

pub fn find_path(
    maze: &Maze,
    heuristic: Box<dyn MazeHeuristic>,
    channel: impl ChannelSender<Message>,
) -> Result<()> {
    let gen = JpsGenerator::new(maze);
    let start_time = Instant::now();

    let mut info = a_star(
        maze.start(),
        maze.goal(),
        heuristic.as_ref(),
        &gen,
        channel.clone(),
    );

    let end_time = Instant::now() - start_time;
    info.time = end_time;

    let _ = channel.send(Message::End(info));

    Ok(())
}

pub fn print_info(info: &Info) {
    match info.path {
        Some(ref path) => {
            let path_len = path.path_len();
            println!("Path found!");
            println!("Length: {}", path_len);
            println!("Cost: {}", path.cost);
        }
        None => {
            println!("Path not found");
        }
    };

    println!("Time: {}s", info.time.as_secs_f64());
    println!("Max queue length: {}", info.max_length);
    println!("Nodes visited: {}", info.nodes);
}
