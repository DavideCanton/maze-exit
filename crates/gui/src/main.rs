use std::{
    fs::File,
    sync::{mpsc, Arc},
    thread,
};

use anyhow::Result;
use clap::{command, Parser};
use macroquad::{
    camera::{set_camera, Camera2D},
    color::{Color, BLACK, WHITE},
    input::{get_keys_pressed, mouse_wheel, KeyCode},
    math::{Rect, Vec2},
    shapes::draw_rectangle,
    window::{clear_background, next_frame, Conf},
};
use maze_exit_bin_common::{
    find_path, parse_args, print_info, read_maze, Args, ImageMazeWriter, MazeWriter,
    MazeWriterWithPath,
};
use maze_exit_lib::{
    algorithm::Message,
    channel::{channel, sync_channel, ChannelSender},
    heuristics::{DiagonalHeuristic, MazeHeuristic},
    maze::Maze,
    position::Position,
};

const W: i32 = 1280;
const H: i32 = 1024;

fn window_conf() -> Conf {
    Conf {
        window_title: "Gui Maze".to_owned(),
        window_width: W,
        window_height: H,
        window_resizable: false,
        ..Default::default()
    }
}

const RED: Color = Color::new(1.0, 0.0, 0.0, 1.0);
const GREEN: Color = Color::new(0.0, 1.0, 0.0, 1.0);
const BLUE: Color = Color::new(0.0, 0.0, 1.0, 1.0);

struct App {
    maze: Arc<Maze>,
    queue: Vec<(Position, f64)>,
    path: Option<Vec<Position>>,
    end: bool,
    camera: Camera2D,
    original_zoom: Vec2,
    move_offset: f32,
    buffer_size: u8,
    solved: bool,
}

impl App {
    fn new(maze: Maze, buffer_size: u8) -> Self {
        let camera = Camera2D::from_display_rect(Rect::new(
            0.0,
            0.0,
            maze.width() as f32,
            maze.height() as f32,
        ));
        let original_zoom = camera.zoom;

        Self {
            maze: Arc::new(maze),
            queue: Vec::new(),
            path: None,
            end: false,
            camera,
            original_zoom,
            move_offset: 1.0,
            buffer_size,
            solved: false,
        }
    }

    async fn main(mut self) {
        let heuristic = Box::new(DiagonalHeuristic::new(&self.maze));
        let start_to_goal = heuristic.compute_heuristic(self.maze.start());

        let maze = self.maze.clone();

        fn start(
            maze: Arc<Maze>,
            heuristic: Box<dyn MazeHeuristic + Send>,
            tx: impl ChannelSender<Message> + Send + 'static,
        ) {
            thread::spawn(move || find_path(&maze, heuristic, tx));
        }

        let rx = match self.buffer_size {
            0 => {
                let (tx, rx) = channel();
                start(maze, heuristic, tx);
                rx
            }
            n => {
                let (tx, rx) = sync_channel(n as usize);
                start(maze, heuristic, tx);
                rx
            }
        };

        while !self.end {
            clear_background(WHITE);

            self.handle_input();

            self.handle_zoom();

            set_camera(&self.camera);

            self.draw_frame(&rx, start_to_goal);

            next_frame().await;
        }
    }

    fn handle_zoom(&mut self) {
        let (_, mouse_wheel_y) = mouse_wheel();
        self.camera.zoom += mouse_wheel_y * 0.001 * self.camera.zoom.signum();
        self.move_offset = 1.0 / (self.camera.zoom.length() * 500.0);
    }

    fn handle_input(&mut self) {
        for k in get_keys_pressed() {
            match k {
                KeyCode::Escape => {
                    self.end = true;
                }
                KeyCode::Up => {
                    self.camera.offset.y -= self.move_offset;
                }
                KeyCode::Down => {
                    self.camera.offset.y += self.move_offset;
                }
                KeyCode::Left => {
                    self.camera.offset.x += self.move_offset;
                }
                KeyCode::Right => {
                    self.camera.offset.x -= self.move_offset;
                }
                KeyCode::Key0 => {
                    self.camera.zoom = self.original_zoom;
                }
                KeyCode::C => {
                    self.camera.offset = Vec2::ZERO;
                }
                KeyCode::S => {
                    if self.solved {
                        let _ = self.save_img();
                    }
                }
                _ => (),
            }
        }
    }

    fn draw_point(&self, point: Position, color: Color) {
        let x = point.x as f32;
        let y = point.y as f32;
        draw_rectangle(x, y, 1.0, 1.0, color);
    }

    fn draw_frame(&mut self, rx: &mpsc::Receiver<Message>, start_to_goal: f64) {
        self.handle_messages(rx);

        self.draw_point(self.maze.start(), RED);
        self.draw_point(self.maze.goal(), GREEN);

        for (pos, dist) in self.queue.iter().copied() {
            let ratio = (dist / start_to_goal) as f32;
            self.draw_point(pos, Color::new(ratio, 1.0 - ratio, 0.0, 1.0));
        }

        for pos in self.maze.walls() {
            self.draw_point(pos, BLACK);
        }

        if let Some(ref path) = self.path {
            for pos in path.iter().copied() {
                self.draw_point(pos, BLUE);
            }
        }
    }

    fn handle_messages(&mut self, rx: &mpsc::Receiver<Message>) {
        while let Ok(msg) = rx.try_recv() {
            match msg {
                Message::Enqueued(pos, dist) => {
                    self.queue.push((pos, dist));
                }
                Message::End(info) => {
                    if !self.solved {
                        self.solved = true;
                        print_info(&info);

                        if let Some(path) = info.path {
                            self.path.replace(path.path);
                        }
                    }
                }
            }
        }
    }

    fn save_img(&self) -> Result<()> {
        let writer = File::create("maze_path.png").unwrap();
        match &self.path {
            None => {
                ImageMazeWriter.write_maze(&self.maze, writer)?;
            }
            Some(path) => {
                ImageMazeWriter.write_maze_with_path(&self.maze, path, writer)?;
            }
        }
        Ok(())
    }
}

#[derive(Parser, Debug)]
struct GuiArgs {
    #[command(flatten)]
    common: Args,
    #[arg(short = 's', long = "buffer_size", default_value_t = 0)]
    buffer_size: u8,
}

#[macroquad::main(window_conf)]
async fn main() -> Result<()> {
    let args: GuiArgs = parse_args();
    let maze = read_maze(args.common.img_path)?;

    let app = App::new(maze, args.buffer_size);
    app.main().await;

    Ok(())
}
