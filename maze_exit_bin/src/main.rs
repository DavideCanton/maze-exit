use std::env;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::time::Instant;

use image::ImageResult;

use crate::image_reader::{MazeImageReader, MazeReader};
use maze_exit_lib::algorithm::a_star;
use maze_exit_lib::generator::{JpsGenerator, MazeChildrenGenerator};
use maze_exit_lib::heuristics::{DiagonalHeuristic, MazeHeuristic};
use maze_exit_lib::maze::Maze;

use crate::display::display_trait::Displayer;
use crate::display::gui_displayer::GuiDisplayer;

mod display;
mod image_reader;

struct App {
    maze: Option<Maze>,
    img_path: PathBuf,
}

impl App {
    fn new(img_path: PathBuf) -> Self {
        App {
            maze: None,
            img_path,
        }
    }

    fn main(&mut self) -> Result<(), Box<dyn Error>> {
        self.maze = Some(self.build_maze()?);

        let maze = self.maze.as_ref().unwrap();

        let mut heuristic = DiagonalHeuristic::default();
        heuristic.set_goal(maze.goal);

        let mut displayer = GuiDisplayer::new();
        displayer.init(&maze, &heuristic);

        displayer.display_image(None, None)?;

        let gen = JpsGenerator::new(maze);
        let start_time = Instant::now();

        let res = a_star(
            maze.start,
            |&pos| pos == maze.goal,
            &heuristic,
            &gen,
            |q| {
                displayer.display_image(None, Some(q)).unwrap();
            },
        );

        let end_time = Instant::now() - start_time;

        match res.0 {
            Some(path) => {
                let (path, cost) = gen.reconstruct_path(&path);
                println!("Path found!");
                println!("Cost: {}", cost);
                println!("Time: {}s", end_time.as_secs_f64());
                println!("{:?}", res.1);
                displayer.display_image(Some(&path), None)?;
            }
            None => println!("Path not found"),
        }

        Ok(())
    }

    fn build_maze(&self) -> ImageResult<Maze> {
        let reader = MazeImageReader;
        reader.read_maze(&self.img_path)
    }
}

#[show_image::main]
fn main() -> Result<(), Box<dyn Error>> {
    let img_path = Path::new(&env::args().nth(1).expect("Missing path")).to_owned();
    let mut app = App::new(img_path);
    app.main()
}
