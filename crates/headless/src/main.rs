use anyhow::Result;
use maze_exit_bin_common::{parse_args, read_maze, App};
use maze_exit_lib::heuristics::DiagonalHeuristic;

fn main() -> Result<()> {
    #[cfg(feature = "debug-so")]
    unsafe {
        backtrace_on_stack_overflow::enable()
    };

    let args = parse_args();

    let maze = read_maze(&args.img_path)?;
    let mut app = App::new(&maze, DiagonalHeuristic::new(&maze));
    app.main()
}
