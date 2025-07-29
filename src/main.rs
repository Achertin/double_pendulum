extern crate nalgebra as na;
use na::{Vector2};
use macroquad::prelude::*;

const LENGTH_PX: f32 = 300.0;
const GRAVITY: Vector2<f32> = Vector2::new(0.0, -9.81);

struct Pendulum {
    length: f32,
    mass: f32,
    angle: f32,
    velocity: f32,
    acceleration: f32
}

impl Pendulum {
    fn position(&self, anchor: &Vector2<f32>) -> Vector2<f32> {
        self.length * Vector2::new(self.angle.sin(), -self.angle.cos()) + anchor
    }
    
    fn update(&mut self, torque: &f32, delta_time: &f32) {
        self.acceleration = torque / (self.mass * self.length * self.length);
        self.velocity += self.acceleration * delta_time;
        self.angle += self.velocity * delta_time;
    }
    
    fn tension(&self, anchor: &Vector2<f32>) -> Vector2<f32> {
        let position = self.position(&anchor);
        -position * position.dot(&GRAVITY) / position.dot(&position)
    }
}

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut pendulum1 = Pendulum {
        length: 1.0,
        mass: 1.0,
        angle: 45.0_f32.to_radians(),
        velocity: 0.0,
        acceleration: 0.0
    };

    let mut pendulum2 = Pendulum {
        length: 0.5,
        mass: 1.0,
        angle: 45.0_f32.to_radians(),
        velocity: 0.0,
        acceleration: 0.0
    };

    loop {
        clear_background(BLACK);
        draw_fps();
        let translate = Vector2::new(screen_width() / 2.0, screen_height() / 2.0);
        let pos1 = pendulum1.position(&Vector2::new(0.0, 0.0));
        let screen_pos1 = translate + LENGTH_PX * Vector2::new(pos1.x, -pos1.y);
        let force1 = GRAVITY + pendulum1.tension(&Vector2::new(0.0, 0.0));
        let torque1 = if force1.x <= 0.0 { -force1.norm() } else { force1.norm() };
        let delta_time = get_frame_time();

        pendulum1.update(&torque1, &delta_time);

        draw_circle(screen_pos1.x, screen_pos1.y, 25.0, WHITE);

        next_frame().await
   }
}
