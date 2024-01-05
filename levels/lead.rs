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
        let intercept_dir = Self::calculate_intercept_dir(false);
        Self::turn_to(intercept_dir);
        Self::fire_when_aimed(intercept_dir);
    }

    pub fn turn_to(target: Vec2) {
        let target_heading = target.angle();
        let heading_error = angle_diff(heading(), target_heading);
        turn(50.0 * heading_error);
    }

    pub fn fire_when_aimed(target: Vec2) {
        let heading_error = angle_diff(heading(), target.angle());
        if heading_error.abs() < 0.004 {
            fire(0);
        }
    }

    // The idea is to transform the shooter and target to a new coordinate system where the line between them is one of the axes, match up the parallell speed, and then calculate the appropriate tangential speed for the projectile.
    // In order for the projectile to hit the target, the I composants of the velocities, vi and ui must be equal. If they are not, the projectile will be too fast or slow in that direction and cannot hit the target.
    // To calculate vi, first find the vector between A and B, and normalize it. Then project u onto AB using a dot product, giving uj. Subtracting uj from u, we get ui which is equal to vi.
    // Once vi has been determined, |vj| can be found by using Pythagoras theorem, since we know the magnitude |v|.
    // Then we can multiply the unit vector AB by |vj| to get vj, and lastly, add together vj and vi to get v
    // A = position of shooter
    // B = positon of target
    // u = velocity of target
    // v = velocity of projectile
    pub fn calculate_intercept_dir(debug: bool) -> Vec2 {
        let vec_to_target = target() + position();
        let dir_to_target = vec_to_target.normalize();
        let velocity_dot = dir_to_target.dot(target_velocity());
        let uj = dir_to_target * velocity_dot;
        let ui = target_velocity() - uj;
        let vi = ui;
        let vj_abs = ((BULLET_SPEED * BULLET_SPEED) - (vi.length() * vi.length())).sqrt();
        let vj = dir_to_target * vj_abs;
        let v = vj + vi;

        if debug {
            draw_line(position(), position() + dir_to_target, RED);
            draw_line(target(), target() + uj, BLUE);
            draw_line(target(), target() + ui, BLUE);
            draw_line(target() + ui, target() + ui + uj, BLUE);
            draw_line(target() + uj, target() + ui + uj, BLUE);
            draw_line(position(), position() + ui, BLUE);
            draw_line(position(), position() + vj, BLUE);
            draw_line(position() + vj, position() + vj + vi, BLUE);
            draw_line(position() + vi, position() + vj + vi, BLUE);
            draw_line(position(), position() + v, RED);
            draw_line(target(), target() + target_velocity() * 1000.0, YELLOW);
            draw_line(position(), position() + v * 1000.0, YELLOW);
        }

        return v.normalize()
    }
}
