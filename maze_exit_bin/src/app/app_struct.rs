use std::{error::Error, path::PathBuf, rc::Rc, time::Instant};

use image::ImageResult;
use maze_exit_lib::{
    algorithm::a_star,
    generator::{JpsGenerator, MazeChildrenGenerator},
    heuristics::{DiagonalHeuristic, HeuristicFn, MazeHeuristic},
    maze::Maze,
};

use crate::{
    app::{
        app_enums::UIType,
        image_reader::{MazeImageReader, MazeReader},
    },
    display::{
        display_trait::Displayer, gui_displayer::GuiDisplayer, noop_displayer::NoopDisplayer,
        term_displayer::TerminalDisplayer,
    },
};

pub struct App {
    img_path: PathBuf,
    ui_type: UIType,
}

impl App {
    pub fn new(img_path: PathBuf, ui_type: UIType) -> Self {
        App { img_path, ui_type }
    }

    fn create_displayer(&self) -> Result<Box<dyn Displayer>, Box<dyn Error>> {
        let displayer: Box<dyn Displayer> = match self.ui_type {
            UIType::Gui => Box::new(GuiDisplayer::new()?),
            UIType::Terminal => Box::new(TerminalDisplayer::default()),
            UIType::Noop => Box::new(NoopDisplayer),
        };

        Ok(displayer)
    }

    pub fn main(&mut self) -> Result<(), Box<dyn Error>> {
        let maze = Rc::new(self.build_maze()?);

        let mut heuristic = DiagonalHeuristic::default();
        heuristic.set_goal(maze.goal);
        let start_to_goal = heuristic.compute_heuristic(&maze.start);

        let mut displayer = self.create_displayer()?;
        displayer.display_image(&maze, start_to_goal, None, None)?;

        let gen = JpsGenerator::new(maze.as_ref());
        let start_time = Instant::now();

        let (path, info) = a_star(maze.start, maze.goal, &heuristic, &gen, |q| {
            // here we ignore errors on display
            let _ = displayer.display_image(&maze, start_to_goal, None, Some(q));
        });

        let end_time = Instant::now() - start_time;

        match path {
            Some(path) => {
                let (path, cost) = gen.reconstruct_path(&path);
                displayer.display_image(&maze, start_to_goal, Some(&path), None)?;
                println!("Path found!");
                println!("Length: {}", path.len());
                println!("Cost: {}", cost);
                println!("Time: {}s", end_time.as_secs_f64());
                println!("{:?}", info);
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
