use crate::{maze::Maze, position::Position};

#[test]
pub fn test_creation() {
    let maze = Maze::new(3, 4, Position::new(0, 0), Position::new(2, 3));
    assert_eq!(maze.width(), 3);
    assert_eq!(maze.height(), 4);
    assert_eq!(maze.start(), Position::new(0, 0));
    assert_eq!(maze.goal(), Position::new(2, 3));
    assert_eq!(maze.walls().count(), 0);
}
