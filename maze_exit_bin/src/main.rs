use app::app_enums::UIType;
use app::app_struct::App;
use clap::Parser;
use context::{create_context, ContextResult};
use std::path::Path;

mod app;
mod context;
mod display;

#[derive(Parser, Debug)]
#[command(
    author="Davide C. <davide.canton5@gmail.com>", 
    version="1.0", 
    about="A* algorithm for solving mazes", 
    long_about = None
)]
struct Args {
    img_path: String,

    #[arg(short, long, value_enum, default_value_t = UIType::No)]
    ui_type: UIType,
}

fn main() -> ContextResult {
    let args = Args::parse();

    let ctx = create_context(args.ui_type)?;

    ctx.run(move || {
        let mut app = App::new(Path::new(&args.img_path).to_owned(), args.ui_type);
        app.main()?;
        Ok(())
    })
}
