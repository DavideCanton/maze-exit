use anyhow::Result;
use image::{Rgba, RgbaImage};
use maze_exit_bin_common::Displayer;
use maze_exit_lib::{algorithm::QueueNode, generator::PathRef, maze::Maze, position::Pos};
use show_image::{
    create_window,
    event::{VirtualKeyCode, WindowEvent},
    WindowProxy,
};
use std::collections::BinaryHeap;

/// Displays a maze in a window using [show_image](https://docs.rs/show_image/latest/show_image/).
pub struct GuiDisplayer {
    /// The window used to display the maze
    window: WindowProxy,
    /// The last image displayed, to double buffer
    last: Option<RgbaImage>,
}

impl GuiDisplayer {
    /// Creates a new `GuiDisplayer`.
    /// It can fail if the `create_window` function fails.
    pub fn new() -> Result<Self> {
        let window = create_window("image", Default::default())?;
        Ok(GuiDisplayer { window, last: None })
    }

    /// Builds the image to display.
    fn build_image<'a>(
        &self,
        maze: &Maze,
        start_to_goal: f64,
        path: Option<PathRef>,
        queue: Option<impl Iterator<Item = &'a QueueNode>>,
        img: &mut RgbaImage,
    ) {
        for w in maze.walls() {
            let Pos { x, y } = w;
            img.put_pixel(x as u32, y as u32, Rgba::from([0, 0, 0, 255]));
        }

        if let Some(queue) = queue {
            for p in queue {
                let Pos { x, y } = p.node;
                let h: f64 = p.heuristic;
                let v = (h / start_to_goal * 255.0) as u8;
                img.put_pixel(x as u32, y as u32, Rgba::from([v, 255 - v, 0, 255]));
            }
        }

        if let Some(path) = path {
            for p in path {
                let Pos { x, y } = p;
                img.put_pixel(*x as u32, *y as u32, Rgba::from([0, 0, 255, 255]));
            }
        }

        let Pos { x, y } = maze.start();
        img.put_pixel(x as u32, y as u32, Rgba::from([255, 0, 0, 255]));

        let Pos { x, y } = maze.goal();
        img.put_pixel(x as u32, y as u32, Rgba::from([0, 255, 0, 255]));
    }
}

impl Displayer for GuiDisplayer {
    fn display_image(
        &mut self,
        maze: &Maze,
        start_to_goal: f64,
        path: Option<PathRef>,
        queue: Option<&BinaryHeap<&QueueNode>>,
    ) -> Result<()> {
        let w = maze.width();
        let h = maze.height();

        let mut img = self
            .last
            .take()
            .unwrap_or_else(|| RgbaImage::from_pixel(w, h, Rgba::from([255, 255, 255, 255])));

        self.build_image(
            maze,
            start_to_goal,
            path,
            queue.map(|v| v.iter().copied()),
            &mut img,
        );
        self.window.set_image("image", img.clone())?;
        self.last.replace(img);
        Ok(())
    }

    /// Waits for `ESC` key to be pressed before exiting.
    fn wait_for_end(&self) -> Result<()> {
        let events = self.window.event_channel()?;
        for event in events {
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
