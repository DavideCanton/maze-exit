use std::cell::RefCell;
use std::collections::BinaryHeap;

use image::{GenericImage, Rgba, RgbaImage};
use maze_exit_lib::{
    algorithm::QueueNode, generator::PathRef, heuristics::MazeHeuristic, maze::Maze, position::Pos,
};
use show_image::{
    create_window,
    event::{VirtualKeyCode, WindowEvent},
    Image, WindowProxy,
};

use crate::display::display_trait::Displayer;

pub struct GuiDisplayer<'a> {
    maze: Option<&'a Maze>,
    window: Option<WindowProxy>,
    start_to_goal: Option<f64>,
    last: RefCell<Option<Vec<u8>>>,
}

impl<'a> GuiDisplayer<'a> {
    pub fn new() -> Self {
        GuiDisplayer {
            maze: None,
            window: None,
            start_to_goal: None,
            last: RefCell::new(None),
        }
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
            let max = self.start_to_goal.unwrap();
            for p in queue {
                let Pos { x, y } = p.node;
                let h: f64 = p.heuristic;
                let v = (h / max * 255.0) as u8;
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
}

impl<'a> Displayer<'a> for GuiDisplayer<'a> {
    fn init<'b: 'a>(
        &'a mut self,
        maze: &'b Maze,
        heuristic: &dyn MazeHeuristic,
    ) -> Result<(), String> {
        self.maze = Some(maze);
        self.window = Some(create_window("image", Default::default()).map_err(|e| e.to_string())?);
        let start_to_goal = heuristic.compute_heuristic(&maze.start);
        self.start_to_goal = Some(start_to_goal);
        Ok(())
    }

    fn display_image(
        &self,
        path: Option<PathRef>,
        queue: Option<&BinaryHeap<QueueNode<Pos>>>,
    ) -> Result<(), String> {
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
        self.window
            .as_ref()
            .unwrap()
            .set_image("image", img)
            .map_err(|e| e.to_string())
    }

    fn wait_for_end(&self) -> Result<(), String> {
        let window = self.window.as_ref().unwrap();
        for event in window.event_channel().map_err(|e| e.to_string())? {
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
}
