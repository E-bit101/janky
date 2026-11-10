use vexide::controller::{ButtonState, JoystickState};

pub struct robot_config {
    pub speed_mul: config_number,
    pub turn_mul: config_number,
    pub claw_low_ang: config_number,
    pub claw_high_ang: config_number,
    pub lift_up: config_button,
    pub lift_down: config_button,
    pub 
}

pub struct config_number {
    pub option_name: String,
    pub initial_value: f64,
    pub min_value: f64,
    pub max_value: f64,
    pub increment: f64
}

pub struct config_button {
    pub option_name: String,
    pub button: ButtonState
}

pub struct config_axis {
    pub option_name: String,
    pub button: ButtonState
}

pub struct config_toggle {
    pub option_name: String,
    pub initial_value: bool
}