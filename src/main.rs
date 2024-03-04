/*
An ncurses-based snake game in Rust.
*/

extern crate nalgebra;
extern crate ncurses;
extern crate rand;

use nalgebra::{Vector2, Vector4};
use ncurses::*;
use rand::*;
use snake_rust::Snake;

const SNAKE_COLOR_PAIR: i16 = 1;
const BERRY_COLOR_PAIR: i16 = 2;

fn new_berry() -> Vector2<i16> {
    let mut rng = rand::thread_rng();
    Vector2::<i16>::new(
        rng.gen_range(1..COLS() - 1) as i16,
        rng.gen_range(1..LINES() - 1) as i16,
    )
}

fn draw_snake(snake: &Snake) {
    assert_eq!(attron(COLOR_PAIR(SNAKE_COLOR_PAIR)), OK, "attron failed");
    assert_eq!(
        mvaddch(snake.body[0].y as i32, snake.body[0].x as i32, 'O' as u32),
        OK,
        "mvaddch failed"
    );
    for part in snake.body.iter().skip(1) {
        assert_eq!(
            mvaddch(part.y as i32, part.x as i32, 'o' as u32),
            OK,
            "mvaddch failed"
        );
    }
    assert_eq!(attroff(COLOR_PAIR(SNAKE_COLOR_PAIR)), OK, "attroff failed");
}

fn draw_berry(berry: &Vector2<i16>) {
    assert_eq!(attron(COLOR_PAIR(BERRY_COLOR_PAIR)), OK, "attron failed");
    assert_eq!(
        mvaddch(berry.y as i32, berry.x as i32, '@' as u32),
        OK,
        "mvaddch failed"
    );
    assert_eq!(attroff(COLOR_PAIR(BERRY_COLOR_PAIR)), OK, "attroff failed");
}

fn draw_stats(win: WINDOW, score: usize) {
    assert_eq!(
        wborder(
            win, '|' as u32, '|' as u32, '-' as u32, '-' as u32, '+' as u32, '+' as u32,
            '+' as u32, '+' as u32,
        ),
        OK,
        "wborder failed"
    );
    let str = format!("Score: {score}");
    assert_eq!(mvprintw(0, 2, &str), OK, "mvprintw failed");
}

fn init_ncurses() -> WINDOW {
    let win = initscr();
    assert_eq!(
        wborder(
            win, '|' as u32, '|' as u32, '-' as u32, '-' as u32, '+' as u32, '+' as u32,
            '+' as u32, '+' as u32,
        ),
        OK,
        "wborder failed"
    );
    assert_eq!(keypad(win, true), OK, "keypad failed");
    assert_eq!(nodelay(win, true), OK, "nodelay failed");
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    assert_eq!(start_color(), OK, "start_solor failed");
    assert_eq!(
        init_pair(SNAKE_COLOR_PAIR, COLOR_GREEN, COLOR_BLACK),
        OK,
        "init_pair failed"
    );
    assert_eq!(
        init_pair(BERRY_COLOR_PAIR, COLOR_RED, COLOR_BLACK),
        OK,
        "init_pair failed"
    );
    assert_eq!(wrefresh(win), OK, "wrefresh failed");
    win
}

fn main() {
    let win = init_ncurses();
    let mut snake = Snake::new();
    let mut berry = new_berry();
    let mut running = true;

    while running {
        let pressed = getch();
        let mut direction: Vector2<i16> = snake.direction;
        let boundaries = Vector4::<i16>::new(0, 0, COLS() as i16, LINES() as i16);

        match pressed {
            KEY_UP => direction = Vector2::<i16>::new(0, -1),
            KEY_DOWN => direction = Vector2::<i16>::new(0, 1),
            KEY_LEFT => direction = Vector2::<i16>::new(-1, 0),
            KEY_RIGHT => direction = Vector2::<i16>::new(1, 0),
            0x1b => {
                running = false;
                continue;
            }
            _ => {}
        }

        snake.set_direction(&direction);
        snake.step();

        if snake.try_hit_walls(&boundaries) || snake.try_eat_self() {
            break;
        }

        if snake.try_eat_food(&berry) {
            berry = new_berry();
        }

        erase();
        draw_snake(&snake);
        draw_berry(&berry);
        draw_stats(win, snake.body.len());

        //sleep for a bit
        std::thread::sleep(std::time::Duration::from_millis(
            (125.0 / snake.speed) as u64,
        ));
    }
    clear();
    refresh();
    endwin();
}
