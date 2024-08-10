use std::{sync::mpsc, time::Instant};

use maze_exit_lib::{
    algorithm::{a_star, Message},
    generator::{ChildrenGenerator, JpsGenerator},
    heuristics::MazeHeuristic,
    maze::Maze,
};

use anyhow::Result;

pub struct App<'a> {
    maze: &'a Maze,
    channel: Option<mpsc::Sender<Message>>,
    heuristic: Box<dyn MazeHeuristic>,
}

impl App<'_> {
    pub fn new(maze: &Maze, heuristic: Box<dyn MazeHeuristic>) -> App<'_> {
        App {
            maze,
            channel: None,
            heuristic,
        }
    }

    pub fn new_channel(
        maze: &Maze,
        heuristic: Box<dyn MazeHeuristic>,
        channel: mpsc::Sender<Message>,
    ) -> App<'_> {
        App {
            maze,
            channel: Some(channel),
            heuristic,
        }
    }

    pub fn main(&mut self) -> Result<()> {
        let maze = self.maze;

        let gen = JpsGenerator::new(maze);
        let start_time = Instant::now();

        let (path, info) = a_star(
            maze.start(),
            maze.goal(),
            self.heuristic.as_ref(),
            &gen,
            self.channel.clone(),
        );

        let end_time = Instant::now() - start_time;

        match path {
            Some(path) => {
                let (path, cost) = gen.reconstruct_path(&path);
                let path_len = path.len();
                if let Some(channel) = &self.channel {
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
}
