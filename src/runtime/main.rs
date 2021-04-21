// this is 61 bytes, space-padded
// it will be changed at deploy-time, look for "ONION_ADDRESS" in binary
const ONION_ADDRESS: &str = "ONION_ADDRESS                                                            ";

// this is 36 bytes
// it will be changed at deploy-time, look for "MY_UUIID" in binary
const MY_ID: &str = "MY_UUIIDxxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx";

// get the manager address (inserted at deploy-time)
fn get_manager_address() -> String {
    return format!("{}.onion", String::from(ONION_ADDRESS).split_off(13).trim());
}

// get the manager address (inserted at deploy-time)
fn get_my_id() -> String {
    return String::from(MY_ID).split_off(8);
}

fn main() {
    println!("The hard-coded manager address is {}.", get_manager_address());
    println!("The hard-coded UUID for me is {}.", get_my_id());
}