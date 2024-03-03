/*
Logic for snake
*/
use nalgebra::{Vector2, Vector4};

const MAX_SNAKE_LENGTH: i8 = 100;
const MAX_SNAKE_SPEED: f32 = 10.0;

//A snake struct
pub struct Snake {
    pub body: Vec<Vector2<i8>>,
    pub direction: Vector2<i8>,
    pub speed: f32,
}

impl Default for Snake {
    fn default() -> Self {
        Self::new()
    }
}

fn is_collision(a: &Vector2<i8>, b: &Vector2<i8>, direction: &Vector2<i8>) -> bool {
    !(a.y != b.y || a.x != b.x && (a.x - direction.x) != b.x)
}

impl Snake {
    pub fn new() -> Self {
        Snake {
            body: Vec::from([Vector2::new(1, 1)]),
            direction: Vector2::new(0, 0),
            speed: 1.0,
        }
    }
    pub fn step(&mut self) {
        for i in (1..self.body.len()).rev() {
            self.body[i] = self.body[i - 1];
        }
        self.body[0].x += self.direction.x * 2;
        self.body[0].y += self.direction.y;
    }

    pub fn grow(&mut self) {
        if self.body.len() >= MAX_SNAKE_LENGTH as usize {
            return;
        }
        self.body.push(self.body[self.body.len() - 1]);
    }

    pub fn set_direction(&mut self, direction: &Vector2<i8>) {
        let dir_len = direction.dot(direction);
        let dp = direction.dot(&self.direction);
        if dp == 0 && dir_len != 0 {
            self.direction = direction / dir_len;
        }
    }

    pub fn speed_up(&mut self) {
        if self.speed < MAX_SNAKE_SPEED {
            self.speed += 0.1;
        }
    }

    pub fn test_collision(&self, point: &Vector2<i8>) -> bool {
        for &part in &self.body {
            if is_collision(&part, point, &self.direction) {
                return true;
            }
        }

        false
    }

    pub fn try_eat_food(&mut self, food: &Vector2<i8>) -> bool {
        if self.test_collision(food) {
            self.grow();
            self.speed_up();
            return true;
        }
        false
    }
    pub fn try_eat_self(&self) -> bool {
        if self.body.len() > 4 {
            for i in 1..self.body.len() {
                if is_collision(&self.body[0], &self.body[i], &self.direction) {
                    return true;
                }
            }
        }
        false
    }
    pub fn try_hit_walls(&self, boundaries: &Vector4<i8>) -> bool {
        self.body[0].x < boundaries.x
            || self.body[0].x > boundaries.z
            || self.body[0].y < boundaries.y
            || self.body[0].y > boundaries.w
    }
}