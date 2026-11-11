use std::f64::consts::PI;
use vexide::{math::Angle, prelude::*};

use crate::util::{self, angle_to, distance, shift_forward};

pub async fn move_to_loc(robot: &mut util::Robot, location: util::Location, tolerance: f64, timeout: f64) {
    let mut ticks = 0.0;
    loop {
        let ang = angle_to(robot.loc, location);
        let l = robot.left_motors.max_voltage() + ang / 4.0;
        let r = robot.left_motors.max_voltage() - ang / 4.0;
        _ = robot.left_motors.set_voltage(l);
        _ = robot.right_motors.set_voltage(r);

        // Exit conditions
        if ticks >= timeout                                     { break; }
        if distance(location, robot.loc) < tolerance { break; }

        ticks += Controller::UPDATE_INTERVAL.as_secs_f64();
        sleep(Controller::UPDATE_INTERVAL).await;
    }
}

pub async fn point_in_dir(robot: &mut util::Robot, direction: f64, tolerance: f64, timeout: f64) {
    let mut ticks = 0.0;
    loop {
        let ang = direction - robot.loc.heading;
        let l = ang / 4.0;
        let r = ang / 4.0;
        _ = robot.left_motors.set_voltage(l);
        _ = robot.right_motors.set_voltage(r);

        // Exit conditions
        if ticks >= timeout                                  { break; }
        if (direction - robot.loc.heading).abs() < tolerance { break; }

        ticks += Controller::UPDATE_INTERVAL.as_secs_f64();
        sleep(Controller::UPDATE_INTERVAL).await;
    }
}

pub fn track_location(robot: &mut util::Robot) -> util::Location {
    let wheel_travel = (PI * 0.0635) / 180.0;
    let l = robot.left_motors.position().unwrap_or(Angle::ZERO).as_degrees() * wheel_travel;
    let r = robot.right_motors.position().unwrap_or(Angle::ZERO).as_degrees() * wheel_travel;

    let mut loc = robot.loc.clone();
    loc = shift_forward(loc, (l + r) / 2.0);
    loc.heading = robot.imu.rotation().unwrap_or(Angle::ZERO).as_degrees();

    _ = robot.left_motors.reset_position();
    _ = robot.right_motors.reset_position();
    return loc;
}

pub fn claw(robot: &mut util::Robot, speed: f64, up: bool) {
    _ = robot.claw_motor.set_voltage(speed);
    if up {
        _ = robot.claw_tilt.set_position_target(Angle::from_degrees(robot.config.number_configs[3].value), 200);
    } else {
        _ = robot.claw_tilt.set_position_target(Angle::from_degrees(robot.config.number_configs[2].value), 200);
    }
}

pub fn set_lift(robot: &mut util::Robot, height: f64) {
    _ = robot.lift_motors.set_position(Angle::from_degrees(height*10.0));
}

pub fn get_lift(robot: &mut util::Robot) -> f64 {
    return robot.lift_motors.position().unwrap_or(Angle::ZERO).as_degrees()/10.0;
}