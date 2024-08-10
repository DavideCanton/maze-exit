use std::{
    sync::{mpsc, Arc},
    thread,
};

use anyhow::Result;
use macroquad::{
    camera::{set_camera, Camera2D},
    color::{Color, BLACK, WHITE},
    input::{get_keys_pressed, mouse_wheel, KeyCode},
    math::{Rect, Vec2},
    shapes::draw_rectangle,
    window::{clear_background, next_frame, Conf},
};
use maze_exit_bin_common::{find_path, parse_args, read_maze};
use maze_exit_lib::{
    algorithm::Message,
    heuristics::{DiagonalHeuristic, MazeHeuristic},
    maze::Maze,
    position::Pos,
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
    queue: Vec<(Pos, f64)>,
    path: Vec<Pos>,
    end: bool,
    camera: Camera2D,
    original_zoom: Vec2,
    move_offset: f32,
}

impl App {
    fn new(maze: Maze) -> Self {
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
            path: Vec::new(),
            end: false,
            camera,
            original_zoom,
            move_offset: 1.0,
        }
    }

    async fn main(mut self) {
        let heuristic = Box::new(DiagonalHeuristic::new(&self.maze));
        let start_to_goal = heuristic.compute_heuristic(self.maze.start());

        let (tx, rx) = mpsc::channel();

        let maze = self.maze.clone();
        thread::spawn(move || find_path(&maze, Some(tx), heuristic));

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
                _ => (),
            }
        }
    }

    fn draw_point(&self, point: Pos, color: Color) {
        let x = point.x as f32;
        let y = point.y as f32;
        draw_rectangle(x, y, 1.0, 1.0, color);
    }

    fn draw_frame(&mut self, rx: &mpsc::Receiver<Message>, start_to_goal: f64) {
        while let Ok(m) = rx.try_recv() {
            match m {
                Message::Enqueued(pos, dist) => {
                    self.queue.push((pos, dist));
                }
                Message::End(p) => {
                    if self.path.is_empty() {
                        self.path.extend(p);
                    }
                }
            }
        }

        self.draw_point(self.maze.start(), RED);
        self.draw_point(self.maze.goal(), GREEN);

        for (pos, dist) in &self.queue {
            let v = (*dist / start_to_goal * 255.0) as u8;

            self.draw_point(*pos, Color::from_rgba(v, 255 - v, 0, 255));
        }

        for w in self.maze.walls() {
            self.draw_point(w, BLACK);
        }

        for p in &self.path {
            self.draw_point(*p, BLUE);
        }
    }
}

#[macroquad::main(window_conf)]
async fn main() -> Result<()> {
    let args = parse_args();
    let maze = read_maze(args.img_path)?;

    let app = App::new(maze);
    app.main().await;

    Ok(())
}
