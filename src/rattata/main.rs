mod powershell;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} <ADDRESS> <PORT>", args[0]);
    } else {
        println!("TODO: connect to {}", args[1]);
    }
}
