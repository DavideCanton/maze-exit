use std::collections::BinaryHeap;

use maze_exit_lib::{algorithm::QueueNode, generator::PathRef, maze::Maze};

use crate::display::display_trait::Displayer;

pub(super) struct NoopDisplayer;

impl Displayer for NoopDisplayer {
    fn display_image(
        &mut self,
        _maze: &Maze,
        _start_to_goal: f64,
        _path: Option<PathRef>,
        _queue: Option<&BinaryHeap<QueueNode>>,
    ) -> Result<(), String> {
        Ok(())
    }
}
