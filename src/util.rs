use vexide::prelude::*;
use vexide_motorgroup::MotorGroup;

use crate::config::RobotConfig;

pub fn steering(x: f64) -> f64 {
    return x*x*x;
}

pub fn turn_speed(x: f64) -> f64{
    return 0.75*x*x + 0.25;
}

pub struct Robot {
    pub controller: Controller,
    pub left_motors: MotorGroup,
    pub right_motors: MotorGroup,
    pub imu: InertialSensor,
    pub claw_motor: Motor,
    pub claw_tilt: Motor,
    pub lift_motors: MotorGroup,
    pub ui: crate::slint_generatedApp::App,
    pub intake_motor: Motor,

    pub loc: Location,
    pub config: RobotConfig
}

#[derive(Copy, Clone)]
pub struct Location {
    pub x: f64,
    pub y: f64,
    pub heading: f64
}

pub fn loc(x: f64, y: f64, heading: f64) -> Location {
    Location { x, y, heading }
}

pub fn stringify_location(loc: Location) -> String {
    return format!("[{}m, {}m, {}°]", loc.x, loc.y, loc.heading);
}

pub fn remap_num(num: f64, min: f64, max: f64) -> f64 {
    return num * (max - min) - min;
}