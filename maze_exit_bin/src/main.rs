use std::{env, error::Error, path::Path};
use app::app_struct::App;

mod app;
mod display;

#[macro_use]
extern crate custom_derive;
#[macro_use]
extern crate enum_derive;

fn main() -> Result<(), Box<dyn Error>> {
    let img_path = Path::new(&env::args().nth(1).expect("Missing path")).to_owned();
    let mut app = App::new(img_path);
    app.main()
}
