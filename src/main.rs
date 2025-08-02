extern crate nalgebra as na;

use na::{Vector2, Vector4};
use macroquad::prelude::*;

const LENGTH_PX: f64 = 200.0;

struct Pendulum {
    length: f64,
    mass: f64,
    angle: f64,
    velocity: f64,
}

impl Pendulum {
    fn new(length: f64, mass: f64, angle: f64) -> Pendulum {
        Pendulum {
            length: length,
            mass: mass,
            angle: angle.to_radians(),
            velocity: 0.0,
        }
    }

    fn position_rel(&self, anchor: &Vector2<f64>) -> Vector2<f64> {
        anchor + self.length * Vector2::new(self.angle.sin(), -self.angle.cos())
    }
    
    fn position(&self) -> Vector2<f64> {
        self.length * Vector2::new(self.angle.sin(), -self.angle.cos())
    }

    fn update(&mut self, angle: f64, velocity: f64) {
        self.angle += angle;
        self.velocity += velocity;
    }
}

fn screen_pos(offset: &Vector2<f32>, position: &Vector2<f64>) -> Vector2<f32> {
    Vector2::new(offset.x + (LENGTH_PX * position.x) as f32, offset.y - (LENGTH_PX * position.y) as f32)
}

fn runge_kutta(pendulum1: &Pendulum, pendulum2: &Pendulum, delta_time: f64) -> Vector4<f64> {
    let m1 = pendulum1.mass;
    let m2 = pendulum2.mass;
    let l1 = pendulum1.length;
    let l2 = pendulum2.length;
    let param = Vector4::new(pendulum1.angle, pendulum2.angle, pendulum1.velocity, pendulum2.velocity);

    let k1 = derivative(m1, m2, l1, l2, &param);
    let k2 = derivative(m1, m2, l1, l2, &(param + delta_time * k1 / 2.0));
    let k3 = derivative(m1, m2, l1, l2, &(param + delta_time * k2 / 2.0));
    let k4 = derivative(m1, m2, l1, l2, &(param + delta_time * k3));
    
    delta_time * (k1 + 2.0 * k2 + 2.0 * k3 + k4) / 6.0
}

fn derivative(m1: f64, m2: f64, l1: f64, l2: f64, params: &Vector4<f64>) -> Vector4<f64> {
    let angle1 = params.x;
    let angle2 = params.y;
    let vel1 = params.z;
    let vel2 = params.w;
    let angle_diff = angle1 - angle2;
    let denominator1 = l1 * (2.0 * m1 + m2 - m2 * (2.0 * angle_diff).cos());
    let denominator2 = l2 * (2.0 * m1 + m2 - m2 * (2.0 * angle_diff).cos());
    let g = 9.81;

    let accel1 = (
        -g * (2.0 * m1 + m2) * angle1.sin()
        -m2 * g * (angle1 - 2.0 * angle2).sin()
        -2.0 * angle_diff.sin() * m2 * (vel2.powi(2) * l2 + vel1.powi(2) * l1 * angle_diff.cos())
    ) / denominator1;

    let accel2 = (
        2.0 * angle_diff.sin() * (vel1.powi(2) * l1 * (m1 + m2)
        + g * (m1 + m2) * angle1.cos()
        + vel2.powi(2) * l2 * m2 * angle_diff.cos())
    ) / denominator2;

    Vector4::new(vel1, vel2, accel1, accel2)
}

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut pendulum1 = Pendulum::new(1.0, 2.0, 90.0);
    let mut pendulum2 = Pendulum::new(1.0, 2.0, 91.0);
    let anchor0 = Vector2::new(0.0_f64, 0.0_f64);

    loop {
        clear_background(BLACK);
        draw_fps();
        let delta_time = get_frame_time() as f64;
        let offset = Vector2::new(screen_width() / 2.0, screen_height() / 4.0);
        let step = runge_kutta(&pendulum1, &pendulum2, delta_time);

        pendulum1.update(step.x, step.z);
        pendulum2.update(step.y, step.w);
        
        let screen_pos1 = screen_pos(&offset, &pendulum1.position_rel(&anchor0));
        let screen_pos2 = screen_pos(&offset, &pendulum2.position_rel(&pendulum1.position()));
        draw_circle(screen_pos1.x, screen_pos1.y, 25.0, WHITE);
        draw_circle(screen_pos2.x, screen_pos2.y, 25.0, WHITE);
        draw_line(offset.x, offset.y, screen_pos1.x, screen_pos1.y, 10.0, WHITE);
        draw_line(screen_pos1.x, screen_pos1.y, screen_pos2.x, screen_pos2.y, 10.0, WHITE);

        next_frame().await
    }
}
