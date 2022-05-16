use clap::{arg, Command};
use tokio::time::{sleep, Duration};
use rand;
use rand::Rng;
use mouse_rs::Mouse;
use rdev::{simulate, EventType, SimulateError, display_size};

#[tokio::main]
async fn main() {
    let matches = Command::new("awake")
        .arg(
            arg!([INTERVAL] ).default_value("30")
                .help("Interval time to check in seconds"),
        )
        .get_matches();
    let interval :u64 = matches.value_of_t("INTERVAL").unwrap();
    println!("interval:{}", interval);
    let period = Duration::from_secs(interval);
    let mut rng = rand::thread_rng();
    let (w, h) = display_size().unwrap();
    let x = w as i32;
    let y = h as i32;
    println!("Screen size:{} x {}", x, y);
    let mouse = Mouse::new();
    loop {
        let pos = mouse.get_position().unwrap();
        sleep(period).await;
        let pos1 = mouse.get_position().unwrap();
        if pos.x == pos1.x && pos.y == pos1.y {
            let pos_x:i32 = rng.gen_range(0..x);
            let pos_y:i32 = rng.gen_range(0..y);
            match simulate(&EventType::MouseMove {x: f64::from(pos_x), y: f64::from(pos_y) }){
                Ok(()) => (),
                Err(SimulateError) => {
                    println!("We could not send MouseMove");
                }
            }
            let pos1 = mouse.get_position().unwrap();
            println!("X: {}, Y: {}", pos1.x, pos1.y);
        }
    }
}
