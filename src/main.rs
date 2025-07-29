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
        let position = self.position(&anchor) - anchor;
        -position * position.dot(&(self.mass * GRAVITY)) / position.dot(&position)
    }
}

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut pendulum1 = Pendulum {
        length: 1.0,
        mass: 2.0,
        angle: 45.0_f32.to_radians(),
        velocity: 0.0,
        acceleration: 0.0
    };

    let mut pendulum2 = Pendulum {
        length: 0.8,
        mass: 2.0,
        angle: 85.0_f32.to_radians(),
        velocity: 0.0,
        acceleration: 0.0
    };

    loop {
        clear_background(BLACK);
        draw_fps();
        let translate = Vector2::new(screen_width() / 2.0, 100.0);
        let pos1 = pendulum1.position(&Vector2::new(0.0, 0.0));
        let pos2 = pendulum2.position(&pos1);
        let screen_pos1 = translate + LENGTH_PX * Vector2::new(pos1.x, -pos1.y);
        let screen_pos2 = translate + LENGTH_PX * Vector2::new(pos2.x, -pos2.y);
        let tension2 = pendulum2.tension(&pos1);
        let force1 = pendulum1.mass * GRAVITY + pendulum1.tension(&Vector2::new(0.0, 0.0)) - tension2;
        let force2 = pendulum2.mass * GRAVITY + tension2;
        let torque1 = pos1.x * force1.y - pos1.y * force1.x;
        let torque2 = pendulum2.length * pendulum2.angle.sin() * force2.y - pendulum2.length * -pendulum2.angle.cos() * force2.x;
        let delta_time = get_frame_time();

        pendulum1.update(&torque1, &delta_time);
        pendulum2.update(&torque2, &delta_time);

        draw_circle(screen_pos1.x, screen_pos1.y, 25.0, WHITE);
        draw_circle(screen_pos2.x, screen_pos2.y, 25.0, WHITE);
        draw_line(translate.x, translate.y, screen_pos1.x, screen_pos1.y, 5.0, WHITE);
        draw_line(screen_pos1.x, screen_pos1.y, screen_pos2.x, screen_pos2.y, 5.0, WHITE);

        next_frame().await
   }
}
