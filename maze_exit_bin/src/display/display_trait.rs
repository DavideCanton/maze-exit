use std::collections::BinaryHeap;

use maze_exit_lib::{
    algorithm::QueueNode, generator::PathRef, heuristics::MazeHeuristic, maze::Maze, position::Pos,
};

pub trait Displayer<'a> {
    fn init<'b: 'a>(
        &'a mut self,
        maze: &'b Maze,
        heuristic: &dyn MazeHeuristic,
    ) -> Result<(), String>;
    fn display_image(
        &self,
        path: Option<PathRef>,
        queue: Option<&BinaryHeap<QueueNode<Pos>>>,
    ) -> Result<(), String>;
    fn wait_for_end(&self) -> Result<(), String>;
}
