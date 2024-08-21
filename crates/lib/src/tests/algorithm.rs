use crate::{
    algorithm::a_star, channel::noop_sender, generator::JpsGenerator,
    heuristics::DiagonalHeuristic, maze_builder::MazeBuilder, position::Position,
};

#[test]
fn test_find_path() {
    let start = Position::new(0, 0);
    let goal = Position::new(9, 9);
    let maze = MazeBuilder::new()
        .start(start)
        .goal(goal)
        .width(10)
        .height(10)
        .build()
        .unwrap();

    let heuristic = DiagonalHeuristic::new(&maze);
    let gen = JpsGenerator::new(&maze);

    let info = a_star(start, goal, &heuristic, &gen, noop_sender());
    let path = info.path;

    assert!(path.is_some());
    let path = path.unwrap();

    assert_eq!(
        path.path,
        &[
            Position::new(0, 0),
            Position::new(1, 1),
            Position::new(9, 9)
        ]
    );
    assert_eq!(path.cost, 1.0);

    assert!(info.max_length > 0);
    assert!(info.nodes > 0);
}
