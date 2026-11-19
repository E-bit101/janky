use vexide::prelude::*;
use vexide_motorgroup::MotorGroup;
use evian::{
    prelude::*,
    math::Angle,
    control::loops::{AngularPid, Pid},
    drivetrain::model::Differential,
    motion::Basic,
    tracking::wheeled::WheeledTracking,
};

use crate::config::RobotConfig;

pub fn steering(x: f64) -> f64 {
    return x*x*x;
}

pub fn turn_speed(x: f64) -> f64{
    return 0.75*x*x + 0.25;
}

pub struct Robot {
    pub drivetrain: Drivetrain<Differential, WheeledTracking>,
    pub controller: Controller,
    pub claw_motor: Motor,
    pub lift_motors: MotorGroup,
    pub toggle_motor: Motor,
    
    // pub ui: crate::slint_generatedApp::App,
    pub config: RobotConfig
}

// pub fn remap_num(num: f64, min: f64, max: f64) -> f64 {
//     return num * (max - min) - min;
// }

pub async fn move_lift_to(lift_motors: &mut MotorGroup, height: f64) {
    let mut timeout: f64 = 0.0;
    loop {
        let diff = height - lift_motors
                                    .position()
                                    .unwrap_or(Angle::from_turns(height)).as_turns();
        _ = lift_motors.set_voltage(diff * 3.0);

        timeout += Controller::UPDATE_INTERVAL.as_millis() as f64 / 1000.0;
        if timeout > 0.1 { break; }
        if diff <= 0.1 { break; }
        sleep(Controller::UPDATE_INTERVAL).await;
    }
}

pub async fn claw(claw_motor: &mut Motor, claw_open: bool) {
    let mut timeout: f64 = 0.0;
    loop {
        if claw_open {
            _ = claw_motor.set_voltage(-claw_motor.max_voltage());
        } else {
            _ = claw_motor.set_voltage(claw_motor.max_voltage());

            // Prevent constant stalling
            if claw_motor.efficiency().unwrap_or(0.0) == 0.0 {
                _ = claw_motor.set_voltage(0.0);
                _ = claw_motor.brake(vexide::smart::motor::BrakeMode::Hold);
            }
        }
        timeout += Controller::UPDATE_INTERVAL.as_millis() as f64 / 1000.0;
        if timeout > 0.1 { break; }
        sleep(Controller::UPDATE_INTERVAL).await;
    }
}

pub async fn go_to_pos(basic: &mut Basic<Pid, AngularPid>, drivetrain: &mut Drivetrain<Differential, WheeledTracking>, x: f64, y: f64) {
    let diff_x = x - drivetrain.tracking.position().x;
    let diff_y = y - drivetrain.tracking.position().y;
    basic.turn_to_point(drivetrain, (x, y)).await;
    basic.drive_distance_at_heading(drivetrain, (diff_x*diff_x+diff_y*diff_y).sqrt(), drivetrain.tracking.heading()).await;
}