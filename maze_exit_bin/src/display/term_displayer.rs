use std::{
    collections::BinaryHeap,
    io::{stdout, Write},
    thread::sleep,
    time::Duration,
};

use crossterm::{
    cursor::MoveTo,
    queue,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};
use maze_exit_lib::{algorithm::QueueNode, generator::PathRef, maze::Maze, position::Pos};

use crate::display::display_trait::Displayer;

type CTResult = crossterm::Result<()>;

pub struct TerminalDisplayer {
    wall_char: char,
    queue_char: char,
    path_char: char,
    start_char: char,
    goal_char: char,
    sleep_ms: Option<u64>,
}

impl TerminalDisplayer {
    pub fn new(
        wall_char: char,
        queue_char: char,
        path_char: char,
        start_char: char,
        goal_char: char,
        sleep_ms: Option<u64>,
    ) -> Self {
        TerminalDisplayer {
            goal_char,
            path_char,
            queue_char,
            start_char,
            wall_char,
            sleep_ms,
        }
    }

    fn inner_display_image(
        &mut self,
        maze: &Maze,
        start_to_goal: f64,
        path: Option<PathRef>,
        queue: Option<&BinaryHeap<QueueNode<Pos>>>,
    ) -> CTResult {
        let mut stdout = stdout();

        queue!(stdout, Clear(ClearType::All), MoveTo(0, 0), ResetColor,)?;

        for w in maze.walls() {
            let Pos { x, y } = w;
            queue!(stdout, MoveTo(*x as u16, *y as u16), Print(self.wall_char))?;
        }

        if let Some(queue) = queue {
            for p in queue {
                let Pos { x, y } = p.node;
                let h: f64 = p.heuristic;
                let v = (h / start_to_goal * 255.0) as u8;
                queue!(
                    stdout,
                    MoveTo(x as u16, y as u16),
                    SetForegroundColor(Color::Rgb {
                        r: v,
                        g: 255 - v,
                        b: 0
                    }),
                    Print(self.queue_char)
                )?;
            }
        }

        if let Some(path) = path {
            queue!(stdout, SetForegroundColor(Color::Blue))?;
            for p in path {
                let Pos { x, y } = p;
                queue!(stdout, MoveTo(*x as u16, *y as u16), Print(self.path_char))?;
            }
        }

        let Pos { x, y } = maze.start;
        queue!(
            stdout,
            MoveTo(x as u16, y as u16),
            SetForegroundColor(Color::Red),
            Print(self.start_char)
        )?;
        let Pos { x, y } = maze.goal;
        queue!(
            stdout,
            MoveTo(x as u16, y as u16),
            SetForegroundColor(Color::Green),
            Print(self.goal_char)
        )?;

        queue!(stdout, MoveTo(0, maze.height() as u16 + 1), ResetColor,)?;

        stdout.flush()?;

        self.sleep_ms
            .map(Duration::from_millis)
            .into_iter()
            .for_each(sleep);

        Ok(())
    }
}

impl Displayer for TerminalDisplayer {
    fn init(&mut self) -> Result<(), String> {
        Ok(())
    }

    fn display_image(
        &mut self,
        maze: &Maze,
        start_to_goal: f64,
        path: Option<PathRef>,
        queue: Option<&BinaryHeap<QueueNode<Pos>>>,
    ) -> Result<(), String> {
        self.inner_display_image(maze, start_to_goal, path, queue)
            .map_err(|e| e.to_string())
    }

    fn wait_for_end(&self) -> Result<(), String> {
        Ok(())
    }
}

impl Default for TerminalDisplayer {
    fn default() -> Self {
        TerminalDisplayer::new('+', '*', '*', 'S', 'G', Some(100))
    }
}
