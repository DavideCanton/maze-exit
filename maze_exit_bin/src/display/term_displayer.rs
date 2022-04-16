use std::{collections::BinaryHeap, io::Write};

use maze_exit_lib::{algorithm::QueueNode, generator::PathRef, maze::Maze, position::Pos};

use crate::display::display_trait::Displayer;

pub struct TerminalDisplayer<W: Write> {
    writer: W,
}

impl<W: Write> TerminalDisplayer<W> {
    pub fn new(writer: W) -> Self {
        TerminalDisplayer { writer }
    }

    fn build_maze(
        &self,
        maze: &Maze,
        path: Option<PathRef>,
        queue: Option<&BinaryHeap<QueueNode<Pos>>>,
    ) -> String {
        let mut rows = Vec::with_capacity(maze.height() as usize);

        for _ in 0..maze.height() {
            let row = vec![" "; maze.width() as usize];
            rows.push(row);
        }

        for w in maze.walls() {
            let Pos { x, y } = w;
            rows[*y as usize][*x as usize] = "#";
        }

        if let Some(queue) = queue {
            for p in queue {
                let Pos { x, y } = p.node;
                rows[y as usize][x as usize] = ":";
            }
        }

        if let Some(path) = path {
            for p in path {
                let Pos { x, y } = p;
                rows[*y as usize][*x as usize] = ".";
            }
        }

        let Pos { x, y } = maze.start;
        rows[y as usize][x as usize] = "S";

        let Pos { x, y } = maze.goal;
        rows[y as usize][x as usize] = "G";

        rows.into_iter()
            .map(|r| r.join(""))
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn write(&mut self, s: String) -> Result<(), String> {
        self.writer
            .write_all(s.as_bytes())
            .map(|_| ())
            .map_err(|e| e.to_string())
    }
}

impl<W: Write> Displayer for TerminalDisplayer<W> {
    fn init(&mut self) -> Result<(), String> {
        Ok(())
    }

    fn display_image(
        &mut self,
        maze: &Maze,
        _start_to_goal: f64,
        path: Option<PathRef>,
        queue: Option<&BinaryHeap<QueueNode<Pos>>>,
    ) -> Result<(), String> {
        let s = self.build_maze(maze, path, queue);
        self.write(s)?;
        self.write("-".repeat(100))
    }

    fn wait_for_end(&self) -> Result<(), String> {
        Ok(())
    }
}
