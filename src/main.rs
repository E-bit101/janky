use std::time::Duration;
use futures_util::join;

use vexide::prelude::*;
use vexide_motorgroup::MotorGroup;
use evian::{
    prelude::*,
    math::Angle,
    control::loops::{AngularPid, Pid},
    drivetrain::model::{Arcade, Differential},
    motion::Basic,
    tracking::wheeled::{TrackingWheel, WheeledTracking},
};

mod util;
mod config;

impl util::Robot {
    const LINEAR_PID: Pid = Pid::new(1.0, 0.0, 0.125, None);
    const ANGULAR_PID: AngularPid = AngularPid::new(16.0, 0.0, 1.0, None);
    const LINEAR_TOLERANCES: Tolerances = Tolerances::new()
        .error(4.0)
        .velocity(0.25)
        .duration(Duration::from_millis(15));
    const ANGULAR_TOLERANCES: Tolerances = Tolerances::new()
        .error(f64::to_radians(8.0))
        .velocity(0.09)
        .duration(Duration::from_millis(15));
}

impl Compete for util::Robot {
    async fn autonomous(&mut self) {
        let dt = &mut self.drivetrain;
        let mut basic: Basic<Pid, AngularPid> = Basic {
            linear_controller: Self::LINEAR_PID,
            angular_controller: Self::ANGULAR_PID,
            linear_tolerances: Self::LINEAR_TOLERANCES,
            angular_tolerances: Self::ANGULAR_TOLERANCES,
            timeout: Some(Duration::from_secs(10)),
        };

        util::go_to_pos(&mut basic, dt, 24.0, 24.0).await;
        join![
            util::move_lift_to(&mut self.lift_motors, 10.0),
            util::claw(&mut self.claw_motor, true),
            util::go_to_pos(&mut basic, dt, 24.0, 72.0)
        ];
        util::go_to_pos(&mut basic, dt, -24.0, 72.0).await;
        join![
            util::claw(&mut self.claw_motor, false),
            util::move_lift_to(&mut self.lift_motors, 0.0),
            util::go_to_pos(&mut basic, dt, -24.0, 24.0)
        ];
        util::go_to_pos(&mut basic, dt, 0.0, 0.0).await;

        // Having fun with modifiers.
        // basic
        //     .drive_distance_at_heading(dt, 8.0, 45.0.deg())
        //     .with_linear_kd(1.2)
        //     .with_linear_output_limit(6.0)
        //     .with_angular_tolerance_duration(Duration::from_millis(5))
        //     .with_angular_error_tolerance(f64::to_radians(10.0))
        //     .with_linear_error_tolerance(12.0)
        //     .await;
    }

    async fn driver(&mut self) {
        let mut config_open = false;
        let mut claw_open = false;

        loop {
            let state = self.controller.state().unwrap_or_default();

            // CONFIG
            if config_open {
                if state.button_x.is_pressed() && (state.button_down.is_now_pressed() || state.button_down.is_now_pressed()) { 
                    config_open = false;
                    config::show_controls(&mut self.controller).await;
                } else if state.button_down.is_pressed()
                       || state.button_left.is_now_pressed() 
                       || state.button_right.is_now_pressed() 
                       || state.button_up.is_now_pressed() {
                    config::edit_config(&mut self.controller, &mut self.config).await;
                }
            } else {
                if state.button_x.is_pressed() && (state.button_down.is_now_pressed() || state.button_down.is_now_pressed()) 
                { config_open = true }
            }

            // LIFT
            if self.lift_motors.position().unwrap_or(Angle::ZERO).as_turns() < 0.0 {
                _ = self.lift_motors.set_voltage(self.lift_motors.max_voltage());
            } else {
                let lift_speed = if state.button_l1.is_pressed() { 1.0 } 
                                 else if state.button_l2.is_pressed() { -1.0 }
                                 else { 0.0 };
                _ = self.lift_motors.set_voltage(self.lift_motors.max_voltage() * lift_speed);
                if lift_speed == 0.0 { 
                    _ = self.lift_motors.brake(vexide::smart::motor::BrakeMode::Hold);
                }
            }

            // CLAW
            if state.button_a.is_now_pressed() { claw_open = !claw_open; }
            if claw_open {
                _ = self.claw_motor.set_voltage(-self.claw_motor.max_voltage());
            } else {
                _ = self.claw_motor.set_voltage(self.claw_motor.max_voltage());

                // Prevent constant stalling
                if self.claw_motor.efficiency().unwrap_or(0.0) == 0.0 {
                    _ = self.claw_motor.set_voltage(0.0);
                    _ = self.claw_motor.brake(vexide::smart::motor::BrakeMode::Hold);
                }
            }
            
            // TOGGLE
            if state.button_r1.is_pressed() {
                _ = self.toggle_motor.set_voltage(self.toggle_motor.max_voltage());
            } else if state.button_l1.is_pressed() {
                _ = self.toggle_motor.set_voltage(self.toggle_motor.max_voltage());
            } else {
                _ = self.toggle_motor.set_voltage(0.0);
            }

            // DRIVING
            let drive_speed = self.config.number_configs[0].value;
            let turn_speed  = self.config.number_configs[1].value;

            let turn = util::steering(state.right_stick.x()) * util::turn_speed(state.right_stick.x()) * turn_speed;

            _ = self.drivetrain.model.drive_arcade(state.left_stick.y() * drive_speed, turn);
            if state.button_y.is_pressed() {
                println!("Heading: {}° Location: {}", self.drivetrain.tracking.heading().as_degrees(), self.drivetrain.tracking.position());
            }

            sleep(Motor::WRITE_INTERVAL).await;
        }
    }
}

#[vexide::main]
async fn main(mut peripherals: Peripherals) {
    let forwards_enc = RotationSensor::new(peripherals.port_5, Direction::Reverse);
    let left_motors = [
        Motor::new(peripherals.port_1, Gearset::Blue, Direction::Reverse),
        Motor::new(peripherals.port_4, Gearset::Blue, Direction::Reverse),
    ];
    let right_motors = [
        Motor::new(peripherals.port_2, Gearset::Blue, Direction::Forward),
        Motor::new(peripherals.port_3, Gearset::Blue, Direction::Forward),
    ];

    let mut imu = InertialSensor::new(peripherals.port_6);
    imu.calibrate().await.unwrap();

    let claw_motor = Motor::new(peripherals.port_10, Gearset::Green, Direction::Forward);
    let lift_motors = MotorGroup::new(vec![
        Motor::new(peripherals.port_11, Gearset::Green, Direction::Forward),
        Motor::new(peripherals.port_12, Gearset::Green, Direction::Forward)
    ]);
    let toggle_motor = Motor::new(peripherals.port_16, Gearset::Green, Direction::Forward);

    config::show_controls(&mut peripherals.primary_controller).await;

    let empty_wheel: [TrackingWheel<RotationSensor>; 0] = [];
    util::Robot {
        drivetrain: Drivetrain::new(
            Differential::new(left_motors, right_motors),
            WheeledTracking::new(
                (0.0, 0.0),
                90.0.deg(),
                [TrackingWheel::new(forwards_enc, 2.75, 0.0, None)],
                empty_wheel,
                Some(imu),
            ),
        ),
        controller: peripherals.primary_controller,
        claw_motor: claw_motor,
        lift_motors: lift_motors,
        toggle_motor: toggle_motor,
        config: config::default_config()
    }
    .compete()
    .await;
}