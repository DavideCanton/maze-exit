use crate::{maze::Maze, position::Pos};

#[test]
pub fn test_creation() {
    let maze = Maze::new(3, 4, Pos::new(0, 0), Pos::new(2, 3));
    assert_eq!(maze.width(), 3);
    assert_eq!(maze.height(), 4);
    assert_eq!(maze.start(), Pos::new(0, 0));
    assert_eq!(maze.goal(), Pos::new(2, 3));
    assert_eq!(maze.walls().count(), 0);
}
