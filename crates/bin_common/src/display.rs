use anyhow::Result;
use std::collections::BinaryHeap;

use maze_exit_lib::{algorithm::QueueNode, generator::PathRef, maze::Maze};

/// Trait to display a maze.
pub trait Displayer {
    /// Invoked for displaying the maze.
    /// # Arguments
    /// * `maze` - The maze to display
    /// * `start_to_goal` - The distance from start to goal, for scaling the color of the queue items.
    /// * `path` - The path to display, if already found
    /// * `queue` - The queue items to display
    ///
    /// # Returns
    /// `Ok(())` if the maze was displayed successfully, `Err` otherwise.
    fn display_image(
        &mut self,
        maze: &Maze,
        start_to_goal: f64,
        path: Option<PathRef>,
        queue: Option<&BinaryHeap<&QueueNode>>,
    ) -> Result<()>;

    /// Invoked when the path is found, to eventually wait for a user input before exiting. Not mandatory to implement.
    fn wait_for_end(&self) -> Result<()> {
        Ok(())
    }
}
