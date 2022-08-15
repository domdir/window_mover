use clap::Parser;
use config::{Config, WindowPosition};
use mover::Mover;

mod config;
mod mover;

/// Tool for moving windows to predefined positions
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the command to move the currently active window to
    #[clap(short, long)]
    command_name: String,

    /// Whether the current window should be added to the config instead
    #[clap(short, long)]
    save: bool,
}

fn main() {
    let args = Args::parse();
    let mut config = Config::get_config();
    let mover = mover::create_mover();

    if args.save {
        save_window_position(&args, &mut config, &mover)
    } else {
        move_window_position(&args, &mut config, &mover)
    }
}

fn save_window_position(args: &Args, config: &mut Config, mover: &impl Mover) {
    let window_name = mover.get_window_name();
    let position = mover.get_current_position();
    let screen_resolution = mover.get_screen_resolution();
    config.add_window_position(
        &args.command_name,
        screen_resolution,
        WindowPosition::new(window_name, position),
    );
    config.save_config();
}

fn move_window_position(args: &Args, config: &mut Config, mover: &impl Mover) {
    let window_name = mover.get_window_name();
    let screen_resolution = mover.get_screen_resolution();
    let target_position = config.get_position(&screen_resolution, &args.command_name, &window_name);
    mover.move_to_position(target_position)
}

mod tests {
    #[test]
    fn my_test() {
        let mut x = 3;
        x += 2;
        assert!(x == 5);
    }
}
