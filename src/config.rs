// use vexide::controller::{ButtonState, JoystickState};
use vexide::prelude::*;

#[derive(Clone)]

pub struct RobotConfig {
    pub number_configs: [ConfigNumber; 4],
    pub current_row: usize
}

#[derive(Clone)]
pub struct ConfigNumber {
    pub option_name: String,
    pub value: f64,
    pub min_value: f64,
    pub max_value: f64,
    pub increment: f64
}

// #[derive(Clone)]
// pub struct ConfigButton {
//     pub option_name: String,
//     pub button: ButtonState
// }

// #[derive(Clone)]
// pub struct ConfigAxis {
//     pub option_name: String,
//     pub button: JoystickState
// }

// #[derive(Clone)]
// pub struct ConfigToggle {
//     pub option_name: String,
//     pub value: bool
// }

pub fn default_config() -> RobotConfig {
    return RobotConfig { 
        number_configs: [
            ConfigNumber { option_name: "Driving Speed".to_string(), value: 1.0, min_value: 0.0, max_value: 1.0, increment: 0.1 }, 
            ConfigNumber { option_name: "Turning Speed".to_string(), value: 0.8, min_value: 0.0, max_value: 2.0, increment: 0.1 }, 
            ConfigNumber { option_name: "Claw Low Angle".to_string(), value: 0.0, min_value: -90.0, max_value: 270.0, increment: 5.0 }, 
            ConfigNumber { option_name: "Claw Low Angle".to_string(), value: 200.0, min_value: -90.0, max_value: 270.0, increment: 5.0 },
        ],
        current_row: 0
    };
}

pub fn draw_config(controller: &mut Controller, config: RobotConfig) {
    if config.current_row == 0 {
        _ = controller.set_text("", 1, 1);
    } else {
        _ = controller.set_text(config.number_configs[config.current_row - 1].clone().option_name, 1, 1);
    }
    
    _ = controller.set_text(config.number_configs[config.current_row].clone().option_name, 2, 1);
    
    if config.current_row == config.number_configs.len() {
        _ = controller.set_text("", 3, 1);
    } else {
        _ = controller.set_text(config.number_configs[config.current_row + 1].clone().option_name, 3, 1);
    }
}