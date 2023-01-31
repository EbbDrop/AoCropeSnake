use macroquad::prelude::*;

use std::iter::FromIterator;

const SQUARES: i16 = 32;
const START_SPEED: f64 = 0.15;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i16,
    y: i16,
}

impl Point {
    fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }

    fn follow(&mut self, other: &Point) {
        if (other.x - self.x).abs() >= 2 || (other.y - self.y).abs() >= 2 {
            self.x += (other.x - self.x).signum();
            self.y += (other.y - self.y).signum();
        }
    }
}

struct Snake {
    head: Point,
    body: Vec<Point>,
    dir: (i16, i16),
    odir: (i16, i16),
}

fn new_snake() -> Snake {
    Snake {
        head: Point::new(SQUARES / 2, SQUARES / 2),
        dir: (-1, 0),
        odir: (-1, 0),
        body: Vec::from_iter([
            Point {
                x: SQUARES / 2 + 1,
                y: SQUARES / 2,
            },
            Point {
                x: SQUARES / 2 + 2,
                y: SQUARES / 2,
            },
        ]),
    }
}

#[macroquad::main("Snake")]
async fn main() {
    rand::srand(get_time() as _);

    let mut snake = new_snake();
    let mut fruit = Point::new(rand::gen_range(0, SQUARES), rand::gen_range(0, SQUARES));
    let mut score = 0;
    let mut speed = START_SPEED;
    let mut last_update = get_time();
    let mut game_over = false;
    let mut paused = false;

    let up = (0, -1);
    let down = (0, 1);
    let right = (1, 0);
    let left = (-1, 0);

    loop {
        if !game_over && !paused {
            if (is_key_down(KeyCode::Right) || is_key_down(KeyCode::D)) && snake.odir != left {
                snake.dir = right;
            } else if (is_key_down(KeyCode::Left) || is_key_down(KeyCode::A)) && snake.odir != right
            {
                snake.dir = left;
            } else if (is_key_down(KeyCode::Up) || is_key_down(KeyCode::W)) && snake.odir != down {
                snake.dir = up;
            } else if (is_key_down(KeyCode::Down) || is_key_down(KeyCode::S)) && snake.odir != up {
                snake.dir = down;
            } else if is_key_down(KeyCode::Space) { // Pause
                paused = true;
            } else if !cfg!(target_os = "wasm") && is_key_down(KeyCode::Escape) {
                std::process::exit(0);
            }

            if get_time() - last_update > speed {
                last_update = get_time();
                snake.head.x += snake.dir.0;
                snake.head.y += snake.dir.1;

                let old_tail_pos = snake.body.last().cloned();
                snake.body[0].follow(&snake.head);
                for i in 1..(snake.body.len()) {
                    let (_, rest_body) = snake.body.split_at_mut(i - 1);
                    let (a, b) = rest_body.split_at_mut(1);
                    b[0].follow(&a[0]);
                }
                if old_tail_pos.as_ref() != snake.body.last() {
                    score += 1;
                }

                if snake.head == fruit {
                    fruit = Point::new(rand::gen_range(0, SQUARES), rand::gen_range(0, SQUARES));
                    let last_tail = snake.body.last().unwrap_or(&snake.head).clone();
                    snake.body.push(last_tail);

                    score += 50;
                    speed *= 0.95;
                    speed = speed.max(0.06);
                }
                if snake.head.x < 0
                    || snake.head.y < 0
                    || snake.head.x >= SQUARES
                    || snake.head.y >= SQUARES
                {
                    game_over = true;
                }
                if snake.body.contains(&snake.head) {
                    game_over = true;
                }
                snake.odir = snake.dir;
            }
        }

        clear_background(BLACK);

        let game_size = screen_width().min(screen_height() - 40.);
        let sq_size = (game_size - 80.) / SQUARES as f32;
        let game_size = sq_size * SQUARES as f32;
        let offset_x = (screen_width() - game_size) / 2.;
        let offset_y = (screen_height() - game_size) / 2. + 40.;

        draw_rectangle(offset_x, offset_y, game_size, game_size, DARKBLUE);

        for p in snake.body.iter() {
            draw_rectangle(
                offset_x + p.x as f32 * sq_size,
                offset_y + p.y as f32 * sq_size,
                sq_size,
                sq_size,
                LIGHTGRAY,
            );
        }

        draw_rectangle(
            offset_x + snake.head.x as f32 * sq_size,
            offset_y + snake.head.y as f32 * sq_size,
            sq_size,
            sq_size,
            WHITE,
        );

        draw_circle(
            offset_x + fruit.x as f32 * sq_size + sq_size / 2.0,
            offset_y + fruit.y as f32 * sq_size + sq_size / 2.0,
            sq_size / 2.0,
            GREEN,
        );

        let text = &format!("Score: {score}");
        let font_size = 60.;
        let text_size = measure_text(text, None, font_size as _, 1.0);

        draw_text(
            text,
            screen_width() / 2. - text_size.width / 2.,
            offset_y - text_size.height,
            font_size,
            WHITE,
        );
        if game_over {
            let text = "Game Over. Press [enter] to play again.";
            let text_size = measure_text(text, None, font_size as _, 1.0);

            draw_text(
                text,
                screen_width() / 2. - text_size.width / 2.,
                offset_y + text_size.height,
                font_size,
                WHITE,
            );

            if is_key_down(KeyCode::Enter) {
                snake = new_snake();
                fruit = Point::new(rand::gen_range(0, SQUARES), rand::gen_range(0, SQUARES));
                score = 0;
                speed = START_SPEED;
                last_update = get_time();
                game_over = false;
            } else if !cfg!(target_os = "wasm") && is_key_down(KeyCode::Escape) {
                std::process::exit(0);
            }
        }

        if paused {
            let text = "Paused. Press [enter] to continue.";
            let text_size = measure_text(text, None, font_size as _, 1.0);

            draw_text(
                text,
                screen_width() / 2. - text_size.width / 2.,
                offset_y + text_size.height,
                font_size,
                WHITE,
            );

            if is_key_down(KeyCode::Enter) {
                paused = false;
            } else if !cfg!(target_os = "wasm") && is_key_down(KeyCode::Escape) {
                std::process::exit(0);
            }
        }
        next_frame().await
    }
}
