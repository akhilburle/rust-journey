use rand::Rng;
use std::{thread, time};

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    pub fn generate() -> Point {
        let x: f64 = rand::thread_rng().gen_range(-1.0..1.0);
        let y: f64 = rand::thread_rng().gen_range(-1.0..1.0);
        Point { x: x, y: y }
    }
    pub fn is_within_circle_with_radius(&self, radius: f64) -> bool {
        self.x * self.x + self.y * self.y < radius * radius
    }
}

fn main() {
    let (mut num_points, mut num_points_within_circle) = (0f64, 0f64);
    let mut i = 0;
    loop {
        let point = Point::generate();
        num_points += 1f64;
        if point.is_within_circle_with_radius(1.0) {
            num_points_within_circle += 1f64;
        }
        if i % 100000 == 0 {
            println!(
                "Value of Pi = {pi}",
                pi = 4f64 * (num_points_within_circle / num_points)
            );
            thread::sleep(time::Duration::from_millis(500));
        }
        i += 1;
    }
}
