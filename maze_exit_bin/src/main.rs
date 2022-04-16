use std::{
    env::{self, VarError},
    error::Error,
    fmt::Display,
    path::{Path, PathBuf},
    rc::Rc,
    time::Instant,
};

use enum_derive::ParseEnumError;
use image::ImageResult;

use maze_exit_lib::{
    algorithm::a_star,
    generator::{JpsGenerator, MazeChildrenGenerator},
    heuristics::{DiagonalHeuristic, HeuristicFn, MazeHeuristic},
    maze::Maze,
};

use crate::{
    display::{
        display_trait::Displayer, gui_displayer::GuiDisplayer, term_displayer::TerminalDisplayer,
    },
    image_reader::{MazeImageReader, MazeReader},
};

mod display;
mod image_reader;

#[macro_use]
extern crate custom_derive;
#[macro_use]
extern crate enum_derive;

custom_derive! {
    #[derive(Debug, EnumFromStr)]
    enum UIType {
        Terminal,
        Gui,
    }
}

#[derive(Debug)]
enum UITypeError {
    ParseEnumError(ParseEnumError, String),
    VarError(VarError),
}

impl Display for UITypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UITypeError::ParseEnumError(_, s) => write!(f, "Could not parse the UI type: {}", s),
            UITypeError::VarError(e) => write!(f, "Error in getting env variable: {}", e),
        }
    }
}

impl Error for UITypeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            UITypeError::ParseEnumError(e, _) => Some(e),
            UITypeError::VarError(e) => Some(e),
        }
    }
}

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

        let mut displayer: Box<dyn Displayer> = match ui_type {
            UIType::Gui => Box::new(GuiDisplayer::new()),
            UIType::Terminal => Box::new(TerminalDisplayer::default()),
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

        let mut displayer = self.create_displayer()?;

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
                displayer.display_image(&maze, start_to_goal, Some(&path), None)?;
                println!("Path found!");
                println!("Cost: {}", cost);
                println!("Time: {}s", end_time.as_secs_f64());
                println!("{:?}", res.1);
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
