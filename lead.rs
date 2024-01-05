

// Tutorial: Lead
// Destroy the enemy ship. Its position is given by the "target" function and velocity by the
// "target_velocity" function. Your ship is not able to accelerate in this scenario.
//
// This is where the game becomes challenging! You'll need to lead the target
// by firing towards where the target will be by the time the bullet gets there.
//
// Hint: target() + target_velocity() * t gives the position of the target after t seconds.
//
// You can scale a vector by a number: vec2(a, b) * c == vec2(a * c, b * c)
//
// p.s. You can change your username by clicking on it at the top of the page.
use oort_api::prelude::*;

const BULLET_SPEED: f64 = 1000.0; // m/s
const RED: u32 = 0xff0000;
const GREEN: u32 = 0x00ff00;
const BLUE: u32 = 0x0000ff;
const YELLOW: u32 = 0xffff00;

pub struct Ship {}

impl Ship {
    pub fn new() -> Ship {
        Ship {}
    }

    pub fn tick(&mut self) {
        // Find the vector AB
        let distance_per_tick = target_velocity() * 0.2;
        let ab = target() + distance_per_tick - position();
        draw_line(position(), position() + ab, RED);

        let dir_to_target = ab.normalize();

        let velocity_dot: f64 = dir_to_target.dot(target_velocity());
        let uj = dir_to_target * velocity_dot;
        draw_line(target(), target() + uj, BLUE);
        let ui = target_velocity() - uj;
        draw_line(target(), target() + ui, BLUE);
        draw_line(target() + ui, target() + ui + uj, BLUE);
        draw_line(target() + uj, target() + ui + uj, BLUE);
        let vi = ui;
        draw_line(position(), position() + ui, BLUE);
        let vj_abs = ((BULLET_SPEED * BULLET_SPEED) - vi.length()).sqrt();
        let vj = dir_to_target * vj_abs;
        draw_line(position(), position() + vj, GREEN);
        draw_line(position() + vj, position() + vj + vi, GREEN);
        draw_line(position() + vi, position() + vj + vi, BLUE);
        let v = vj + vi;
        draw_line(position(), position() + v, RED);

        draw_line(target(), target() + target_velocity() * 1000.0, YELLOW);
        draw_line(position(), position() + v * 1000.0, YELLOW);

        let angle = v.angle();
        Self::turn_to(angle);
        fire(0);
    }

    pub fn shoot_at_target(tgt_pos: Vec2) {
        let angle_diff = angle_diff(heading(), (tgt_pos - position()).angle());

        if angle_diff > 0.0 {
            if (angle_diff < 0.05) {
                turn(0.6);
                fire(0);
            } else {
                turn(1.0);
            }
        } else {
            if (angle_diff > -0.05) {
                turn(-0.6);
                fire(0);
            } else {
                turn(-1.0);
            }
        }
    }

    pub fn turn_to(target_heading: f64) {
        let heading_error = angle_diff(heading(), target_heading);
        turn(50.0 * heading_error);
    }
}
