use std::f64::consts::SQRT_2;

use crate::{
    algorithm::Child,
    generator::{ChildrenGenerator, JpsGenerator},
    maze_builder::MazeBuilder,
    position::Pos,
};

#[test]
fn test_generate_no_parent_walls() {
    let maze = maze_builder().build().unwrap();
    let gen = JpsGenerator::new(&maze);

    let children = gen.generate_children(maze.start(), None);
    assert_eq!(children.len(), 3);

    contains_child(&children, Pos::new(1, 0), 1.0);
    contains_child(&children, Pos::new(0, 1), 1.0);
    contains_child(&children, Pos::new(1, 1), SQRT_2);
}

#[test]
fn test_generate_no_parent() {
    let maze = maze_builder().build().unwrap();
    let gen = JpsGenerator::new(&maze);

    let children = gen.generate_children((1, 1).into(), None);
    assert_eq!(children.len(), 8);

    contains_child(&children, Pos::new(1, 0), 1.0);
    contains_child(&children, Pos::new(0, 1), 1.0);
    contains_child(&children, Pos::new(1, 2), 1.0);
    contains_child(&children, Pos::new(2, 1), 1.0);
    contains_child(&children, Pos::new(0, 0), SQRT_2);
    contains_child(&children, Pos::new(2, 0), SQRT_2);
    contains_child(&children, Pos::new(2, 2), SQRT_2);
    contains_child(&children, Pos::new(0, 2), SQRT_2);
}

fn maze_builder() -> MazeBuilder {
    MazeBuilder::new()
        .width(10)
        .height(10)
        .start((0, 0).into())
        .goal((9, 9).into())
}

fn contains_child(children: &[Child], pos: Pos, weight: f64) -> bool {
    children.iter().any(|c| c.node == pos && c.weight == weight)
}
