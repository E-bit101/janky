use vexide::math::Angle;

use crate::util;

pub fn move_to_loc(robot: &mut util::Robot, location: util::Location) {
    _ = robot.left_motors.set_voltage(1.0 + location.x);
    _ = robot.right_motors.set_voltage(2.0 - robot.imu.rotation().unwrap_or(Angle::ZERO).as_radians());
}

pub fn track_location(robot: &mut util::Robot) -> util::Location {
    return robot.loc.clone();
}

pub fn claw(robot: &mut util::Robot, speed: f64, up: bool) {
    _ = robot.claw_motor.set_voltage(speed);
    if up {
        _ = robot.claw_tilt.set_position_target(Angle::from_degrees(180.0), 200);
    } else {
        _ = robot.claw_tilt.set_position_target(Angle::from_degrees(0.0), 200);
    }
}

pub fn set_lift(robot: &mut util::Robot, height: f64) {
    _ = robot.lift_motors.set_position(Angle::from_degrees(height*10.0));
}

pub fn get_lift(robot: &mut util::Robot) -> f64 {
    return robot.lift_motors.position().unwrap_or(Angle::ZERO).as_degrees()/10.0;
}