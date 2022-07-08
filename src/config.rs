use std::{collections::HashMap, fs};

use dirs::config_dir;
use serde::{Deserialize, Serialize};

const CONFIG_FILE_NAME: &str = "window_mover.yaml";

#[derive(Serialize, Deserialize, Default, Debug)]
pub(crate) struct Config {
    pub(crate) commands: HashMap<String, Vec<Command>>,
}

impl Config {
    pub(crate) fn get_config() -> Self {
        Self::read_config().unwrap_or_default()
    }

    fn read_config() -> Option<Config> {
        let mut file_path = config_dir()?;
        file_path.push(CONFIG_FILE_NAME);

        let config_file_content = fs::read_to_string(file_path).ok()?;
        serde_yaml::from_str(&config_file_content).ok()?
    }

    pub(crate) fn save_config(&self) {
        let config_str = serde_yaml::to_string(self).expect("Unable to serialize config");

        let mut file_path = config_dir().expect("Home dir not found");
        file_path.push(CONFIG_FILE_NAME);
        fs::write(file_path, config_str).expect("Unable to write config")
    }

    pub(crate) fn add_window_position(
        &mut self,
        command_name: &str,
        screen_resolution: String,
        window_position: WindowPosition,
    ) {
        let commands = self
            .commands
            .entry(screen_resolution)
            .or_insert(Default::default());
        let command = if let Some(c) = commands.iter_mut().find(|cmd| cmd.name == command_name) {
            c
        } else {
            commands.push(Command::new(command_name.to_string(), Position::default()));
            commands.last_mut().unwrap()
        };

        command.window_positions.push(window_position);
    }

    pub(crate) fn get_position(
        &self,
        screen_resolution: &str,
        command_name: &str,
        window_name: &str,
    ) -> Position {
        if let Some(commands) = self.commands.get(screen_resolution) {
            if let Some(cmd) = commands.iter().find(|cmd| cmd.name == command_name) {
                if let Some(pos) = cmd.window_positions.iter().find(|pos| {
                    pos.window_names
                        .iter()
                        .filter(|name| name.contains(window_name))
                        .next()
                        != None
                }) {
                    pos.position.clone()
                } else {
                    cmd.default.clone()
                }
            } else {
                Position::default()
            }
        } else {
            Position::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct WindowPosition {
    window_names: Vec<String>,
    position: Position,
}

impl WindowPosition {
    pub(crate) fn new(window_name: String, position: Position) -> Self {
        Self {
            window_names: vec![window_name],
            position,
        }
    }
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
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
