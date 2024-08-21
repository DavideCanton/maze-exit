mod app;
mod display;

use std::time::Instant;

use anyhow::Result;
use maze_exit_lib::{
    algorithm::{a_star, Message},
    channel::ChannelSender,
    generator::{ChildrenGenerator, JpsGenerator},
    heuristics::MazeHeuristic,
    maze::Maze,
};

pub use app::args::{parse_args, Args};
pub use app::maze_readers::{read_maze, BinaryReaderCell, MAZE_BINARY_READER_HEADER};
pub use display::Displayer;

pub fn find_path(
    maze: &Maze,
    heuristic: Box<dyn MazeHeuristic>,
    channel: impl ChannelSender<Message>,
) -> Result<()> {
    let gen = JpsGenerator::new(maze);
    let start_time = Instant::now();

    let info = a_star(
        maze.start(),
        maze.goal(),
        heuristic.as_ref(),
        &gen,
        channel.clone(),
    );

    let end_time = Instant::now() - start_time;

    match info.path {
        Some(ref path) => {
            let (path, cost) = gen.reconstruct_path(path);
            let path_len = path.len();
            println!("Path found!");
            println!("Length: {}", path_len);
            println!("Cost: {}", cost);
        }
        None => {
            println!("Path not found")
        }
    }

    let _ = channel.send(Message::End(info.path));
    println!("Time: {}s", end_time.as_secs_f64());
    println!("Max queue length: {}", info.max_length);
    println!("Nodes visited: {}", info.nodes);

    Ok(())
}
