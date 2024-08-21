use crate::{
    algorithm::a_star, channel::noop_sender, generator::JpsGenerator,
    heuristics::DiagonalHeuristic, maze_builder::MazeBuilder, position::Pos,
};

#[test]
fn test_find_path() {
    let start = Pos::new(0, 0);
    let goal = Pos::new(9, 9);
    let maze = MazeBuilder::new()
        .start(start)
        .goal(goal)
        .width(10)
        .height(10)
        .build()
        .unwrap();

    let heuristic = DiagonalHeuristic::new(&maze);
    let gen = JpsGenerator::new(&maze);

    let (path, info) = a_star(start, goal, &heuristic, &gen, noop_sender());

    assert!(path.is_some());
    assert_eq!(
        path.unwrap(),
        &[Pos::new(0, 0), Pos::new(1, 1), Pos::new(9, 9)]
    );

    assert!(info.max_length > 0);
    assert!(info.nodes > 0);
}
