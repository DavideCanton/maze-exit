use std::thread;

use anyhow::{bail, Result};
use maze_exit_bin_common::{find_path, parse_args, print_info, read_maze, Args};
use maze_exit_lib::{algorithm::Message, channel::channel, heuristics::DiagonalHeuristic};

fn main() -> Result<()> {
    #[cfg(feature = "debug-so")]
    unsafe {
        backtrace_on_stack_overflow::enable()
    };

    let args: Args = parse_args();

    let maze = read_maze(&args.img_path)?;
    let heuristic = Box::new(DiagonalHeuristic::new(&maze));

    let (tx, rx) = channel();

    let jh = thread::spawn(move || loop {
        let msg = rx.recv().unwrap();
        if let Message::End(info) = msg {
            return info;
        }
    });

    find_path(&maze, heuristic, tx)?;

    match jh.join() {
        Ok(info) => {
            print_info(&info);
        }
        Err(e) => {
            let e = *e.downcast_ref::<&str>().unwrap();
            bail!(e)
        }
    }

    Ok(())
}
