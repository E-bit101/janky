// use vexide::controller::{ButtonState, JoystickState};
use vexide::prelude::*;

#[derive(Clone)]

pub struct RobotConfig {
    pub number_configs: [ConfigNumber; 2],
    pub current_row: isize
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
            ConfigNumber { option_name: "Turning Speed".to_string(), value: 0.95, min_value: 0.0, max_value: 2.0, increment: 0.1 }
        ],
        current_row: 0
    };
}

fn num_cfg(cfg: ConfigNumber, selected: bool) -> String {
    if selected {
        return format!(">{}: {}       ", cfg.option_name, cfg.value);
    }
    return format!(" {}: {}       ", cfg.option_name, cfg.value);
}

pub async fn show_controls(controller: &mut Controller) {
    _ = controller.set_text("LIFT: L1=+ L2=-       ", 1, 1).await;
    _ = controller.set_text("CLAW: A=toggle        ", 2, 1).await;
    _ = controller.set_text("TOGGLE: R1=+ R2=-     ", 3, 1).await;
}

pub async fn edit_config(controller: &mut Controller, config: &mut RobotConfig) {
    let state = controller.state().unwrap_or_default();

    if state.button_up.is_now_pressed() {
        config.current_row -= 1;
    }
    if state.button_down.is_now_pressed() {
        config.current_row += 1;
    }
    config.current_row = config.current_row.clamp(0, config.number_configs.len() as isize - 1);

    if state.button_left.is_now_pressed() {
        let option: &mut ConfigNumber = &mut config.number_configs[config.current_row as usize];
        option.value -= if state.button_x.is_pressed() { option.increment * option.increment } else { option.increment };
        option.value = option.value.clamp(option.min_value, option.max_value);
    }

    if state.button_right.is_now_pressed() {
        let option: &mut ConfigNumber = &mut config.number_configs[config.current_row as usize];
        option.value += if state.button_x.is_pressed() { option.increment * option.increment } else { option.increment };
        option.value = option.value.clamp(option.min_value, option.max_value);
    }

    draw_config(controller, config).await;
}

pub async fn draw_config(controller: &mut Controller, config: &mut RobotConfig) {
    // _ = controller.clear_screen().await;
    if config.current_row == 0 {
        _ = controller.set_text("", 1, 1).await;
    } else {
        _ = controller.set_text(num_cfg(config.number_configs[config.current_row as usize - 1].clone(), false), 1, 1).await;
    }
    
    _ = controller.set_text(num_cfg(config.number_configs[config.current_row as usize].clone(), true), 2, 1).await;
    
    if config.current_row as usize == config.number_configs.len() - 1 {
        _ = controller.set_text("", 3, 1).await;
    } else {
        _ = controller.set_text(num_cfg(config.number_configs[config.current_row as usize + 1].clone(), false), 3, 1).await;
    }
}