use std::env;
use std::error::Error;
use std::io::stdout;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::time::Instant;

use image::ImageResult;

use crate::image_reader::{MazeImageReader, MazeReader};
use maze_exit_lib::algorithm::a_star;
use maze_exit_lib::generator::{JpsGenerator, MazeChildrenGenerator};
use maze_exit_lib::heuristics::{DiagonalHeuristic, HeuristicFn, MazeHeuristic};
use maze_exit_lib::maze::Maze;

use crate::display::{
    display_trait::Displayer, gui_displayer::GuiDisplayer, term_displayer::TerminalDisplayer,
};

mod display;
mod image_reader;

struct App {
    maze: Option<Rc<Maze>>,
    img_path: PathBuf,
}

impl App {
    fn new(img_path: PathBuf) -> Self {
        App {
            maze: None,
            img_path,
        }
    }

    fn create_displayer(&self, gui: bool) -> Result<Box<dyn Displayer>, Box<dyn Error>> {
        let mut displayer: Box<dyn Displayer> = if gui {
            Box::new(GuiDisplayer::new())
        } else {
            Box::new(TerminalDisplayer::new(stdout()))
        };
        displayer.init()?;
        Ok(displayer)
    }

    fn main(&mut self) -> Result<(), Box<dyn Error>> {
        let maze = Rc::new(self.build_maze()?);
        self.maze = Some(maze.clone());

        let mut heuristic = DiagonalHeuristic::default();
        heuristic.set_goal(maze.goal);

        let start_to_goal = heuristic.compute_heuristic(&maze.start);

        let mut displayer = self.create_displayer(false)?;

        displayer.display_image(&maze, start_to_goal, None, None)?;

        let gen = JpsGenerator::new(maze.as_ref());
        let start_time = Instant::now();

        let res = a_star(
            maze.start,
            |&pos| pos == maze.goal,
            &heuristic,
            &gen,
            |q| {
                displayer
                    .display_image(&maze, start_to_goal, None, Some(q))
                    .unwrap();
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
                displayer.display_image(&maze, start_to_goal, Some(&path), None)?;
                displayer.wait_for_end()?;
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
