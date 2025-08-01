extern crate nalgebra as na;

use na::{Vector2};
use macroquad::prelude::*;

const LENGTH_PX: f64 = 200.0;

struct Pendulum {
    length: f64,
    mass: f64,
    angle: f64,
    velocity: f64,
    acceleration: f64
}

impl Pendulum {
    fn new(length: f64, mass: f64, angle: f64) -> Pendulum {
        Pendulum {
            length: length,
            mass: mass,
            angle: angle.to_radians(),
            velocity: 0.0,
            acceleration: 0.0
        }
    }

    fn position_rel(&self, anchor: &Vector2<f64>) -> Vector2<f64> {
        anchor + self.length * Vector2::new(self.angle.sin(), -self.angle.cos())
    }
    
    fn position(&self) -> Vector2<f64> {
        self.length * Vector2::new(self.angle.sin(), -self.angle.cos())
    }

    fn update(&mut self, acceleration: &f64, velocity: &f64, delta_time: &f64) {
        self.acceleration = acceleration / self.length;
        self.velocity + (velocity / self.length).powf(0.5);
        self.angle += self.velocity * delta_time;
    }
}

fn screen_pos(offset: &Vector2<f32>, position: &Vector2<f64>) -> Vector2<f32> {
    Vector2::new(offset.x + (LENGTH_PX * position.x) as f32, offset.y - (LENGTH_PX * position.y) as f32)
}

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut pendulum1 = Pendulum::new(1.0, 1.0, 70.0);
    let mut pendulum2 = Pendulum::new(0.8, 1.0, 77.0);
    let anchor0 = Vector2::new(0.0_f64, 0.0_f64);

    loop {
        clear_background(BLACK);
        draw_fps();
        let delta_time = get_frame_time() as f64;
        let offset = Vector2::new(screen_width() / 2.0, screen_height() / 4.0);
        let angle_diff = pendulum2.angle - pendulum1.angle;
        let tension1 = -pendulum1.mass * pendulum2.length * pendulum2.acceleration / angle_diff.sin();
        let tension2 = pendulum1.mass * (pendulum1.length * pendulum1.acceleration + 9.81 * pendulum1.angle.sin()) / angle_diff.sin();
        let accel1 = tension2 * angle_diff.sin() / pendulum1.mass - 9.81 * pendulum1.angle.sin();
        let vel1 = tension1 / pendulum1.mass - tension2 * angle_diff.cos() / pendulum1.mass - 9.81 * pendulum1.angle.cos();
        let accel2 = -tension1 * angle_diff.sin() / pendulum1.mass;
        let vel2 = tension2 / pendulum2.mass + tension2 / pendulum1.mass - tension1 * angle_diff.cos() / pendulum1.mass;

        pendulum1.update(&accel1, &vel1, &delta_time);
        pendulum2.update(&accel2, &vel2, &delta_time);
        
        let screen_pos1 = screen_pos(&offset, &pendulum1.position_rel(&anchor0));
        let screen_pos2 = screen_pos(&offset, &pendulum2.position_rel(&pendulum1.position()));
        draw_circle(screen_pos1.x, screen_pos1.y, 25.0, WHITE);
        draw_circle(screen_pos2.x, screen_pos2.y, 25.0, WHITE);
        draw_line(offset.x, offset.y, screen_pos1.x, screen_pos1.y, 10.0, WHITE);
        draw_line(screen_pos1.x, screen_pos1.y, screen_pos2.x, screen_pos2.y, 10.0, WHITE);

        next_frame().await
    }
}
