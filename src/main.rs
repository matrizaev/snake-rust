/*
An ncurses-based snake game in Rust.
*/

use nalgebra::{Vector2, Vector4};
use ncurses::*;

use snake_rust::{food::Food, snake::Snake};

const SNAKE_COLOR_PAIR: i16 = 1;
const BERRY_COLOR_PAIR: i16 = 2;
const FRUIT_COLOR_PAIR: i16 = 3;

fn get_food_color_code(food: &Food) -> i16 {
    match food.name {
        "berry" => BERRY_COLOR_PAIR,
        "fruit" => FRUIT_COLOR_PAIR,
        _ => panic!("Unknown food type"),
    }
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

fn draw_food(food: &Food) {
    let color = get_food_color_code(food);
    assert_eq!(attron(COLOR_PAIR(color)), OK, "attron failed");
    assert_eq!(
        mvaddch(food.position.y as i32, food.position.x as i32, '@' as u32),
        OK,
        "mvaddch failed"
    );
    assert_eq!(attroff(COLOR_PAIR(color)), OK, "attroff failed");
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
    assert_eq!(
        init_pair(FRUIT_COLOR_PAIR, COLOR_BLUE, COLOR_BLACK),
        OK,
        "init_pair failed"
    );
    assert_eq!(wrefresh(win), OK, "wrefresh failed");
    win
}

fn main() {
    let win = init_ncurses();
    let mut boundaries = Vector4::<i16>::new(0, 0, COLS() as i16, LINES() as i16);
    let mut snake = Snake::new();
    let mut food = Food::new(&boundaries);
    let mut running = true;

    while running {
        let pressed = getch();
        let mut direction: Vector2<i16> = snake.direction;
        boundaries = Vector4::<i16>::new(0, 0, COLS() as i16, LINES() as i16);

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

        if snake.try_eat_food(&food) {
            food = Food::new(&boundaries);
        }

        erase();
        draw_snake(&snake);
        draw_food(&food);
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
