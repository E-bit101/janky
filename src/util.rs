use std::f64::consts::PI;

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

pub fn shift_forward(loc: Location, dist: f64) -> Location {
    return Location { 
        x: loc.x + loc.heading.cos() * dist, 
        y: loc.y + loc.heading.sin() * dist,
        heading: loc.heading
    }
}

pub fn distance(loc1: Location, loc2: Location) -> f64 {
    return ((loc1.x-loc2.x).powi(2)+(loc1.y-loc2.y).powi(2)).powf(0.5);
}

pub fn angle_to(loc1: Location, loc2: Location) -> f64 {
    (loc2.y - loc1.y).atan2(loc2.x - loc1.x) * 180.0 / PI
}

pub fn angle_diff(start: f64, target: f64) -> f64{
    return (target - start + 180.0) % 360.0 - 180.0;
}

pub fn stringify_location(loc: Location) -> String {
    return format!("[{}m, {}m, {}°]", loc.x, loc.y, loc.heading);
}

pub fn remap_num(num: f64, min: f64, max: f64) -> f64 {
    return num * (max - min) - min;
}