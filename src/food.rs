use nalgebra::{Vector2, Vector4};
use rand::*;

#[derive(Debug)]
pub struct Food {
    pub name: &'static str,
    pub position: Vector2<i16>,
    pub nutrition: u8,
}

impl Food {
    pub fn new(boundaries: &Vector4<i16>) -> Self {
        let mut rng = rand::thread_rng();
        let choice = rng.gen_bool(0.5);
        match choice {
            true => Food::berry(boundaries),
            false => Food::fruit(boundaries),
        }
    }

    pub fn _new(name: &'static str, position: Vector2<i16>, nutrition: u8) -> Self {
        Food {
            name,
            position,
            nutrition,
        }
    }

    pub fn berry(boundaries: &Vector4<i16>) -> Self {
        Food {
            name: "berry",
            position: Food::rand_position(boundaries),
            nutrition: 1,
        }
    }
    pub fn fruit(boundaries: &Vector4<i16>) -> Self {
        Food {
            name: "fruit",
            position: Food::rand_position(boundaries),
            nutrition: 2,
        }
    }
    fn rand_position(boundaries: &Vector4<i16>) -> Vector2<i16> {
        let mut rng = rand::thread_rng();
        Vector2::<i16>::new(
            rng.gen_range(boundaries.x..boundaries.z) as i16,
            rng.gen_range(boundaries.y..boundaries.w) as i16,
        )
    }
}
