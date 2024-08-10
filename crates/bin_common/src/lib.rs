mod app;
mod display;

use std::{sync::mpsc, time::Instant};

use anyhow::Result;
use maze_exit_lib::{
    algorithm::{a_star, Message},
    generator::{ChildrenGenerator, JpsGenerator},
    heuristics::MazeHeuristic,
    maze::Maze,
};

pub use app::args::{parse_args, Args};
pub use app::maze_readers::{read_maze, BinaryReaderCell, MAZE_BINARY_READER_HEADER};
pub use display::Displayer;

pub fn find_path(
    maze: &Maze,
    channel: Option<mpsc::Sender<Message>>,
    heuristic: Box<dyn MazeHeuristic>,
) -> Result<()> {
    let gen = JpsGenerator::new(maze);
    let start_time = Instant::now();

    let (path, info) = a_star(
        maze.start(),
        maze.goal(),
        heuristic.as_ref(),
        &gen,
        channel.clone(),
    );

    let end_time = Instant::now() - start_time;

    match path {
        Some(path) => {
            let (path, cost) = gen.reconstruct_path(&path);
            let path_len = path.len();
            if let Some(channel) = &channel {
                let _ = channel.send(Message::End(path));
            }
            println!("Path found!");
            println!("Length: {}", path_len);
            println!("Cost: {}", cost);
            println!("Time: {}s", end_time.as_secs_f64());
            println!("{:?}", info);
        }
        None => println!("Path not found"),
    }

    Ok(())
}
