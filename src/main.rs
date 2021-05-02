#[macro_use]
extern crate impl_ops;

use std::cell::RefCell;
use std::collections::BinaryHeap;
use std::env;
use std::error::Error;
use std::path::{Path, PathBuf};

use image::{GenericImage, Rgba, RgbaImage};
use show_image::error::SetImageError;
use show_image::event::{VirtualKeyCode, WindowEvent};
use show_image::{create_window, Image, WindowProxy};

use crate::algorithm::{a_star, QueueNode};
use crate::generator::{ChildrenGenerator, JpsGenerator, PathRef};
use crate::heuristics::heur_diag;
use crate::image_reader::read_from_image;
use crate::maze::Maze;
use crate::position::Pos;
use std::time::Instant;

#[allow(dead_code)]
mod algorithm;
#[allow(dead_code)]
mod generator;
mod heuristics;
mod image_reader;
#[allow(dead_code)]
mod maze;
mod maze_builder;
mod position;
#[allow(dead_code)]
mod utils;

struct App {
    maze: Option<Maze>,
    window: Option<WindowProxy>,
    heuristic: Option<Box<dyn Fn(&Pos) -> f64>>,
    max_heur: f64,
    last: RefCell<Option<Vec<u8>>>,
    img_path: PathBuf,
}

impl App {
    fn new(img_path: PathBuf) -> Self {
        App {
            maze: None,
            window: None,
            heuristic: None,
            max_heur: 0.0,
            last: RefCell::new(None),
            img_path,
        }
    }

    fn main(&mut self) -> Result<(), Box<dyn Error>> {
        self.maze = Some(self.build_maze());
        self.window = Some(create_window("image", Default::default())?);
        self.heuristic = Some(Box::new(heur_diag(self.maze.as_ref().unwrap().goal)));
        self.max_heur = self.heuristic.as_ref().unwrap()(&self.maze.as_ref().unwrap().start);

        self.display_image(None, None)?;

        let maze = self.maze.as_ref();
        let gen = JpsGenerator::new(maze.unwrap());
        let start_time = Instant::now();

        let res = a_star(
            maze.unwrap().start,
            |&pos| pos == maze.unwrap().goal,
            self.heuristic.as_ref().unwrap(),
            |c, p| gen.generate_children(c, p),
            |q| {
                self.display_image(None, Some(q)).unwrap();
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
                self.display_image(Some(&path), None)?;
            }
            None => println!("Path not found"),
        }

        for event in self.window.as_ref().unwrap().event_channel()? {
            if let WindowEvent::KeyboardInput(event) = event {
                let is_escape = event.input.key_code == Some(VirtualKeyCode::Escape)
                    && event.input.state.is_pressed();

                if is_escape {
                    break;
                }
            }
        }

        Ok(())
    }

    fn build_image(
        &self,
        path: Option<PathRef>,
        queue: Option<&BinaryHeap<QueueNode<Pos>>>,
        img: &mut impl GenericImage<Pixel = Rgba<u8>>,
    ) {
        let maze = self.maze.as_ref().unwrap();

        for w in maze.walls() {
            let Pos { x, y } = w;
            img.put_pixel(*x as u32, *y as u32, Rgba::from([0, 0, 0, 255]));
        }

        if let Some(queue) = queue {
            for p in queue {
                let Pos { x, y } = p.node;
                let h: f64 = self.heuristic.as_ref().unwrap()(&p.node);
                let v = (h / self.max_heur * 255.0) as u8;
                img.put_pixel(x as u32, y as u32, Rgba::from([v, 255 - v, 0, 255]));
            }
        }

        if let Some(path) = path {
            for p in path {
                let Pos { x, y } = p;
                img.put_pixel(*x as u32, *y as u32, Rgba::from([0, 0, 255, 255]));
            }
        }

        let Pos { x, y } = maze.start;
        img.put_pixel(x as u32, y as u32, Rgba::from([255, 0, 0, 255]));

        let Pos { x, y } = maze.goal;
        img.put_pixel(x as u32, y as u32, Rgba::from([0, 255, 0, 255]));
    }

    fn display_image(
        &self,
        path: Option<PathRef>,
        queue: Option<&BinaryHeap<QueueNode<Pos>>>,
    ) -> Result<(), SetImageError> {
        let maze = self.maze.as_ref();
        let w = maze.unwrap().width();
        let h = maze.unwrap().height();

        let mut img = match self.last.borrow().as_ref() {
            Some(v) => RgbaImage::from_raw(w, h, v.clone()).expect("Failed to create image"),
            None => RgbaImage::from_fn(w, h, |_, _| Rgba::from([255, 255, 255, 255])),
        };

        self.build_image(path, queue, &mut img);
        {
            let v = match self.last.borrow_mut().take() {
                Some(mut v) => {
                    v.clear();
                    v.extend_from_slice(img.as_raw());
                    v
                }
                None => Vec::from(img.as_raw().as_slice()),
            };
            *self.last.borrow_mut() = Some(v);
        }
        let img = Image::BoxDyn(Box::new(img));
        self.window.as_ref().unwrap().set_image("image", img)
    }

    fn build_maze(&self) -> Maze {
        let maze = read_from_image(&self.img_path);
        maze.unwrap()
    }
}

#[show_image::main]
fn main() -> Result<(), Box<dyn Error>> {
    let img_path = Path::new(&env::args().nth(1).expect("Missing path")).to_owned();
    let mut app = App::new(img_path);
    app.main()
}
