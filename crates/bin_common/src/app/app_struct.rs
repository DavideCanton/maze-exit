use std::{path::PathBuf, time::Instant};

use maze_exit_lib::{
    algorithm::a_star,
    generator::{ChildrenGenerator, JpsGenerator},
    heuristics::{DiagonalHeuristic, MazeHeuristic},
    maze::Maze,
};

use crate::{
    app::image_reader::{MazeImageReader, MazeReader},
    display::Displayer,
};
use anyhow::Result;

pub struct App<D> {
    img_path: PathBuf,
    displayer: D,
}

impl<D: Displayer> App<D> {
    pub fn new(img_path: PathBuf, displayer: D) -> Self {
        App {
            img_path,
            displayer,
        }
    }

    pub fn main(&mut self) -> Result<()> {
        let maze = self.build_maze()?;

        let heuristic = DiagonalHeuristic::new(&maze);
        let start_to_goal = heuristic.compute_heuristic(maze.start());

        self.displayer
            .display_image(&maze, start_to_goal, None, None)?;

        let gen = JpsGenerator::new(&maze);
        let start_time = Instant::now();

        let (path, info) = a_star(maze.start(), maze.goal(), &heuristic, &gen, |q| {
            // here we ignore errors on display
            let _ = self
                .displayer
                .display_image(&maze, start_to_goal, None, Some(q));
        });

        let end_time = Instant::now() - start_time;

        match path {
            Some(path) => {
                let (path, cost) = gen.reconstruct_path(&path);
                self.displayer
                    .display_image(&maze, start_to_goal, Some(&path), None)?;
                println!("Path found!");
                println!("Length: {}", path.len());
                println!("Cost: {}", cost);
                println!("Time: {}s", end_time.as_secs_f64());
                println!("{:?}", info);
                self.displayer.wait_for_end()?;
            }
            None => println!("Path not found"),
        }

        Ok(())
    }

    fn build_maze(&self) -> Result<Maze> {
        let reader = MazeImageReader;
        reader.read_maze(&self.img_path)
    }
}
