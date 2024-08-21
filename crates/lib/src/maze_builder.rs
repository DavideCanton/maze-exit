use crate::maze::Maze;
use crate::position::Position;
use anyhow::{bail, Result};
use std::collections::HashSet;

#[derive(Default)]
pub struct MazeBuilder {
    walls: HashSet<Position>,
    start: Option<Position>,
    goal: Option<Position>,
    errors: Vec<String>,
    width: Option<u32>,
    height: Option<u32>,
}

macro_rules! set_or_error {
    ($name: ident, $arg: ty) => {
        pub fn $name(mut self, arg: $arg) -> Self {
            match self.$name {
                Some(_) => {
                    self.errors
                        .push(format!("{} already set at {}", stringify!($name), arg))
                }
                None => self.$name = Some(arg),
            };
            self
        }
    };
}

impl MazeBuilder {
    pub fn new() -> Self {
        MazeBuilder::default()
    }

    set_or_error!(width, u32);
    set_or_error!(height, u32);
    set_or_error!(start, Position);
    set_or_error!(goal, Position);

    pub fn add_wall(mut self, pos: Position) -> Self {
        self.walls.insert(pos);
        self
    }

    pub fn build(mut self) -> Result<Maze> {
        self.check_options();

        let mut maze_opt = None;

        if self.errors.is_empty() {
            let mut maze = Maze::new(
                self.width.unwrap(),
                self.height.unwrap(),
                self.start.unwrap(),
                self.goal.unwrap(),
            );

            for wall in self.walls {
                if let Err(s) = maze.set(wall, true) {
                    self.errors.push(format!("Invalid wall {}, {}", wall, s));
                }
            }

            maze_opt = Some(maze);
        }

        if self.errors.is_empty() {
            Ok(maze_opt.unwrap())
        } else {
            bail!(self.errors.join("\n"));
        }
    }

    fn check_options(&mut self) {
        if self.width.is_none() {
            self.errors.push("width not set".to_owned());
        }
        if self.height.is_none() {
            self.errors.push("height not set".to_owned());
        }
        if self.start.is_none() {
            self.errors.push("start not set".to_owned());
        }
        if self.goal.is_none() {
            self.errors.push("goal not set".to_owned());
        }
    }
}
