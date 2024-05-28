use std::path::Path;

use anyhow::Result;
use display::GuiDisplayer;
use maze_exit_bin_common::{parse_args, App};

mod display;

#[show_image::main]
fn main() -> Result<()> {
    let args = parse_args();

    let displayer = GuiDisplayer::new()?;
    let mut app = App::new(Path::new(&args.img_path).to_owned(), displayer);
    app.main()
}
