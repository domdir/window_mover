use crate::config::Position;

#[cfg(target_os = "linux")]
mod linux_mover;

pub(crate) trait Mover {
    fn move_to_position(&self, position: Position);
    fn get_current_position(&self) -> Position;
    fn get_window_name(&self) -> String;
}

pub(crate) fn create_mover() -> impl Mover {
    #[cfg(target_os = "linux")]
    linux_mover::LinuxMover {}
}
