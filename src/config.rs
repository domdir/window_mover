use std::fs;

use dirs::config_dir;
use json_pretty::PrettyFormatter;
use serde::{Deserialize, Serialize};

const CONFIG_FILE_NAME: &str = "window_mover.json";

#[derive(Serialize, Deserialize, Default)]
pub(crate) struct Config {
    pub(crate) commands: Vec<Command>,
}

impl Config {
    pub(crate) fn get_config() -> Self {
        Self::read_config().unwrap_or_default()
    }

    fn read_config() -> Option<Config> {
        let mut file_path = config_dir()?;
        file_path.push(CONFIG_FILE_NAME);

        let config_file_content = fs::read_to_string(file_path).ok()?;

        serde_json::from_str(&config_file_content).ok()?
    }

    pub(crate) fn save_config(&self) {
        let config_str = serde_json::to_string(self).expect("Unable to serialize config");
        let formatter = PrettyFormatter::from_str(&config_str);
        let pretty_config_str = formatter.pretty();

        let mut file_path = config_dir().expect("Home dir not found");
        file_path.push(CONFIG_FILE_NAME);
        fs::write(file_path, pretty_config_str).expect("Unable to write config")
    }

    pub(crate) fn add_window_position(
        &mut self,
        command_name: &str,
        window_position: WindowPosition,
    ) {
        let command = if let Some(c) = self
            .commands
            .iter_mut()
            .find(|cmd| cmd.name == command_name)
        {
            c
        } else {
            self.commands
                .push(Command::new(command_name.to_string(), Position::default()));
            self.commands.last_mut().unwrap()
        };

        command.window_positions.push(window_position);
    }

    pub(crate) fn get_position(&self, command_name: &str, window_name: &str) -> Position {
        if let Some(cmd) = self.commands.iter().find(|cmd| cmd.name == command_name) {
            if let Some(pos) = cmd
                .window_positions
                .iter()
                .find(|pos| pos.window_name.contains(window_name))
            {
                pos.position.clone()
            } else {
                cmd.default.clone()
            }
        } else {
            Position::default()
        }
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Command {
    name: String,
    window_positions: Vec<WindowPosition>,
    default: Position,
}

impl Command {
    pub(crate) fn new(name: String, default: Position) -> Self {
        Self {
            name,
            default,
            window_positions: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct WindowPosition {
    window_name: String,
    position: Position,
}

impl WindowPosition {
    pub(crate) fn new(window_name: String, position: Position) -> Self {
        Self {
            window_name,
            position,
        }
    }
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub(crate) struct Position {
    pub(crate) left: isize,
    pub(crate) top: isize,
    pub(crate) width: isize,
    pub(crate) height: isize,
}

impl Position {
    pub(crate) fn new(left: isize, top: isize, width: isize, height: isize) -> Position {
        Position {
            left,
            top,
            width,
            height,
        }
    }
}
