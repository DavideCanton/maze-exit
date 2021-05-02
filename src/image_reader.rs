use std::path::Path;

use image::error::ImageResult;
use image::io::Reader as ImageReader;
use image::GenericImageView;

use crate::maze::Maze;
use crate::maze_builder::MazeBuilder;
use crate::position::Pos;

pub fn read_from_image(path: &Path) -> ImageResult<Maze> {
    let image = ImageReader::open(path)?.decode()?;
    let mut builder = MazeBuilder::new();
    builder = builder.width(image.width()).height(image.height());

    for x in 0..image.width() {
        for y in 0..image.height() {
            let p = image.get_pixel(x, y);
            let pos = Pos::new(x as i32, y as i32);

            match p.0 {
                [0, 0, 0, _] => builder = builder.add_wall(pos),
                [255, 0, 0, _] => builder = builder.start(pos),
                [0, 255, 0, _] => builder = builder.goal(pos),
                _ => {}
            }
        }
    }

    Ok(builder.build().unwrap())
}
