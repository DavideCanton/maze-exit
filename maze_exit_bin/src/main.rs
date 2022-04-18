use app::app_enums::UIType;
use app::app_struct::App;
use clap::Parser;
use std::error::Error;
use std::path::Path;
mod app;
mod display;

#[macro_use]
extern crate custom_derive;
#[macro_use]
extern crate enum_derive;

#[derive(Parser, Debug)]
#[clap(
    author="Davide C. <davide.canton5@gmail.com>", 
    version="1.0", 
    about="A* algorithm for solving mazes", 
    long_about = None
)]
struct Args {
    img_path: String,

    #[clap(short, long)]
    ui_type: UIType,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let mut app = App::new(Path::new(&args.img_path).to_owned(), args.ui_type);
    app.main()
}
