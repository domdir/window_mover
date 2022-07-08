use std::process::Command;

use crate::config::Position;

use super::Mover;

// Window positions can be found with 'wmctrl -lG'

pub(crate) struct LinuxMover {}

impl LinuxMover {
    fn position_to_str(position: &Position) -> String {
        format!(
            "0,{},{},{},{}",
            position.left, position.top, position.width, position.height
        )
    }
}

impl Mover for LinuxMover {
    fn move_to_position(&self, position: Position) {
        // first remove maximized properties
        // otherwise it is not possible to put a window to a specific pos
        Command::new("wmctrl")
            .arg("-r")
            .arg(":ACTIVE:")
            .arg("-b")
            .arg("remove,maximized_vert,maximized_horz")
            .output()
            .expect("Failed to execute wmctrl. Is it installed?");

        Command::new("wmctrl")
            .arg("-r")
            .arg(":ACTIVE:")
            .arg("-e")
            .arg(Self::position_to_str(&position))
            .output()
            .expect("Failed to set position.");
    }

    fn get_current_position(&self) -> Position {
        let window_name = self.get_window_name();
        let result = Command::new("wmctrl")
            .arg("-lGx")
            .output()
            .expect("Failed to execute wmctrl. Is it installed?");
        let result = std::str::from_utf8(&result.stdout).expect("Output could not be converted");

        result
            .lines()
            .map(|line| line.split_whitespace())
            .map(|parts| parts.into_iter().skip(2))
            .map(|mut parts| {
                (
                    Position::new(
                        parts.next().unwrap().parse().unwrap(),
                        parts.next().unwrap().parse().unwrap(),
                        parts.next().unwrap().parse().unwrap(),
                        parts.next().unwrap().parse().unwrap(),
                    ),
                    parts
                        .next()
                        .unwrap()
                        .split_once('.')
                        .unwrap()
                        .1
                        .to_ascii_lowercase(),
                )
            })
            .find(|parts| parts.1 == window_name)
            .expect("Could not find current window position")
            .0
    }

    fn get_window_name(&self) -> String {
        std::str::from_utf8(
            &Command::new("xdotool")
                .arg("getwindowfocus")
                .arg("getwindowclassname")
                .output()
                .expect("Failed to execute xdotool. Is it installed?")
                .stdout,
        )
        .expect("Output could not be converted")
        .trim()
        .to_string()
        .to_lowercase()
    }

    fn get_screen_resolution(&self) -> String {
        std::str::from_utf8(
            &Command::new("xdpyinfo")
                .output()
                .expect("Failed to execute xdpyinfo. Is it installed?")
                .stdout,
        )
        .expect("Output could not be converted")
        .lines()
        .find_map(|line| {
            if line.contains("dimensions") {
                Some(line.split_whitespace().nth(1)?.to_string())
            } else {
                None
            }
        })
        .expect("No dimension output found")
    }
}
