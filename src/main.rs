use clap::{arg, Command};
use std::time::Duration;
use enigo::*;

fn main() {
    let matches = Command::new("awake")
        .arg(
            arg!([INTERVAL] ).default_value("15")
                .help("Interval time to check in seconds"),
        )
        .get_matches();
    let interval :u64 = matches.value_of_t("INTERVAL").unwrap();
    println!("interval:{}", interval);
    let period = Duration::from_secs(interval);
    let mut enigo = Enigo::new();
    loop {
        let (pos_x, pos_y) = Enigo::mouse_location();
        println!("X: {}, Y: {}", pos_x, pos_y);
        std::thread::sleep(period);
        let (pos1_x, pos1_y) = Enigo::mouse_location();
        if pos_x == pos1_x && pos_y == pos1_y {
            let pos_dx = match pos_x {
                dx if dx < 1000 => 10,
                _ => -16
            };
            let pos_dy = match pos_y {
                dy if dy < 600 => 4,
                _ => -16
            };
            println!("dx: {}, dy: {}", pos_dx, pos_dy);
            enigo.mouse_move_relative(pos_dx, pos_dy);
        }
    }
}
