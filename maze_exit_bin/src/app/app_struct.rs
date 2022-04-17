use std::{
    env::{self, VarError},
    error::Error,
    path::PathBuf,
    rc::Rc,
    time::Instant,
};

use image::ImageResult;
use maze_exit_lib::{
    algorithm::a_star,
    generator::{JpsGenerator, MazeChildrenGenerator},
    heuristics::{DiagonalHeuristic, HeuristicFn, MazeHeuristic},
    maze::Maze,
};

use crate::{
    app::{
        app_enums::{UIType, UITypeError},
        image_reader::{MazeImageReader, MazeReader},
    },
    display::{
        display_trait::Displayer, gui_displayer::GuiDisplayer, term_displayer::TerminalDisplayer,
    },
};

pub struct App {
    maze: Option<Rc<Maze>>,
    img_path: PathBuf,
}

impl App {
    pub fn new(img_path: PathBuf) -> Self {
        App {
            maze: None,
            img_path,
        }
    }

    fn get_ui_type(&self) -> Result<UIType, UITypeError> {
        match env::var("UI_TYPE") {
            Ok(v) => v.parse().map_err(|e| UITypeError::ParseEnumError(e, v)),

            Err(VarError::NotPresent) => Ok(UIType::Terminal),

            e => e
                .map(|_| UIType::Terminal) // needed to typecheck, but here we are always in error
                .map_err(UITypeError::VarError),
        }
    }

    fn create_displayer(&self) -> Result<Box<dyn Displayer>, Box<dyn Error>> {
        let ui_type = self.get_ui_type()?;

        let displayer: Box<dyn Displayer> = match ui_type {
            UIType::Gui => Box::new(GuiDisplayer::new()?),
            UIType::Terminal => Box::new(TerminalDisplayer::default()),
        };

        Ok(displayer)
    }

    pub fn main(&mut self) -> Result<(), Box<dyn Error>> {
        let maze = Rc::new(self.build_maze()?);
        self.maze = Some(maze.clone());

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
