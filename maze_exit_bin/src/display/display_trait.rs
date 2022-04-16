use std::collections::BinaryHeap;

use maze_exit_lib::{algorithm::QueueNode, generator::PathRef, maze::Maze, position::Pos};

pub trait Displayer {
    fn init(&mut self) -> Result<(), String>;

    fn display_image(
        &mut self,
        maze: &Maze,
        start_to_goal: f64,
        path: Option<PathRef>,
        queue: Option<&BinaryHeap<QueueNode<Pos>>>,
    ) -> Result<(), String>;

    fn wait_for_end(&self) -> Result<(), String>;
}
