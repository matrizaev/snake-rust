use nalgebra::{ComplexField, Vector2, Vector4};
use snake_rust::Snake;

#[test]
fn test_snake() {
    let mut snake = Snake::new();

    assert!(snake.body.len() == 1);

    assert!(snake.speed == 1.0);
    assert!(snake.body[0].x == 1 && snake.body[0].y == 1);
    assert!(snake.direction.x == 0 && snake.direction.y == 0);

    let mut direction = Vector2::<i8>::new(1, 0);
    snake.set_direction(&direction);
    snake.step();
    assert!(snake.body[0].x == 3 && snake.body[0].y == 1);
    assert!(snake.direction.x == 1 && snake.direction.y == 0);

    let food = Vector2::<i8>::new(3, 1);
    assert!(snake.try_eat_food(&food) == true);
    assert!(snake.body.len() == 2);
    assert!(snake.body[1].x == 3 && snake.body[1].y == 1);

    direction = Vector2::<i8>::new(-1, 0);
    snake.set_direction(&direction);
    assert!(snake.direction.x == 1 && snake.direction.y == 0);

    direction = Vector2::<i8>::new(0, 0);
    snake.set_direction(&direction);
    assert!(snake.direction.x == 1 && snake.direction.y == 0);

    snake.speed_up();
    assert!((snake.speed - 1.2).abs() < 0.1);

    let boundaries = Vector4::<i8>::new(0, 0, 1, 1);

    assert!(snake.try_hit_walls(&boundaries) == true)
}
