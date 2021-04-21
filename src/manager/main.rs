mod rattata;

use std::thread::sleep;
use std::time::Duration;

fn main() {
    let (_socket, tor, port) = rattata::start(0);
    sleep(Duration::from_millis(2000));
    println!("Files in {}", rattata::location());
    println!("Server running at {}:{}", rattata::hostname(), port);
    let _ = tor.join();
}
