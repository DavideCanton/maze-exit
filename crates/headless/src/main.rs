use anyhow::Result;
use maze_exit_bin_common::{parse_args, App, Displayer};
use maze_exit_lib::{algorithm::QueueNode, generator::PathRef, maze::Maze};
use std::{collections::BinaryHeap, path::Path};

pub struct HeadlessDisplayer;

impl Displayer for HeadlessDisplayer {
    fn display_image(
        &mut self,
        _maze: &Maze,
        _start_to_goal: f64,
        _path: Option<PathRef>,
        _queue: Option<&BinaryHeap<&QueueNode>>,
    ) -> Result<()> {
        Ok(())
    }
}

fn main() -> Result<()> {
    #[cfg(feature = "debug-so")]
    unsafe {
        backtrace_on_stack_overflow::enable()
    };

    let args = parse_args();

    let mut app = App::new(Path::new(&args.img_path).to_owned(), HeadlessDisplayer);
    app.main()
}
