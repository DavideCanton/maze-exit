use std::{
    sync::{mpsc, Arc},
    thread,
};

use anyhow::Result;
use macroquad::{
    color::{self, Color},
    input::{get_keys_pressed, KeyCode},
    shapes::draw_rectangle,
    window::{clear_background, next_frame, Conf},
};
use maze_exit_bin_common::{parse_args, read_maze, App};
use maze_exit_lib::{
    algorithm::Message,
    heuristics::{DiagonalHeuristic, MazeHeuristic},
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

#[macroquad::main(window_conf)]
async fn main() -> Result<()> {
    let args = parse_args();
    let maze = Arc::new(read_maze(args.img_path)?);
    let (tx, rx) = mpsc::channel();

    let heuristic = DiagonalHeuristic::new(&maze);
    let start_to_goal = heuristic.compute_heuristic(maze.start());

    let maze2 = maze.clone();
    thread::spawn(move || {
        let mut app = App::new_channel(&maze2, heuristic, tx);
        app.main()
    });

    let mut queue = Vec::new();
    let mut path = Vec::new();
    let mut end = false;

    let zoom_factor = (W as f32 / maze.width() as f32)
        .floor()
        .min((H as f32 / maze.height() as f32).floor())
        .max(1.0);

    while !end {
        clear_background(color::WHITE);

        for k in get_keys_pressed() {
            if k == KeyCode::Escape {
                end = true;
            }
        }

        while let Ok(m) = rx.try_recv() {
            match m {
                Message::Enqueued(pos, dist) => {
                    queue.push((pos, dist));
                }
                Message::End(p) => {
                    if path.is_empty() {
                        path.extend(p);
                    }
                }
            }
        }

        draw_point(maze.start(), RED, zoom_factor);
        draw_point(maze.goal(), GREEN, zoom_factor);

        for (pos, dist) in &queue {
            let v = (dist / start_to_goal * 255.0) as u8;

            draw_point(
                *pos,
                color::Color::from_rgba(v, 255 - v, 0, 255),
                zoom_factor,
            );
        }

        for w in maze.walls() {
            draw_point(w, color::BLACK, zoom_factor);
        }

        for p in &path {
            draw_point(*p, BLUE, zoom_factor);
        }

        next_frame().await;
    }

    Ok(())
}

fn draw_point(point: Pos, color: color::Color, zoom_factor: f32) {
    let x = point.x as f32 * zoom_factor;
    let y = point.y as f32 * zoom_factor;
    draw_rectangle(x, y, zoom_factor, zoom_factor, color);
}
