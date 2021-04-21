use libtor::{ Tor, TorFlag, HiddenServiceVersion, TorAddress, log, Error };
use dirs;
use std::fs::{ File };
use std::io::{ Read, Write };
use std::ffi::CString;
use std::net::{ TcpListener, TcpStream, SocketAddr };
use std::thread::{ spawn, JoinHandle };
use std::fmt::{ format };
use portpicker::pick_unused_port;

// this makes write work on socket
#[allow(unused)] use std::io::prelude::*;

// FFI/C interface

#[cfg(target_pointer_width = "32")]
#[no_mangle]
pub extern "C" fn ffi_location() -> *const u8 {
    return CString::new(location()).unwrap().into_raw();
}

#[cfg(target_pointer_width = "32")]
#[no_mangle]
pub extern "C" fn ffi_hostname() -> *const u8 {
    return  CString::new(hostname()).unwrap().into_raw();
}

#[cfg(target_pointer_width = "64")]
#[no_mangle]
pub extern "C" fn ffi_location() -> *const i8 {
    return CString::new(location()).unwrap().into_raw();
}

#[cfg(target_pointer_width = "64")]
#[no_mangle]
pub extern "C" fn ffi_hostname() -> *const i8 {
    return  CString::new(hostname()).unwrap().into_raw();
}

#[no_mangle]
pub extern "C" fn ffi_start(port: u16) -> u16 {
    let (_socket, tor, setport) = start(port);
    let _ = tor.join();
    return setport;
}

#[no_mangle]
pub extern "C" fn ffi_choose_port() -> u16 {
    return pick_unused_port().unwrap();
}

/// get the current setings dir (which has tor stuff in it)
pub fn location() -> String {
    return format!("{}/rattata", dirs::config_dir().unwrap().into_os_string().into_string().unwrap());
}

/// handle an incoming request
fn handle_client(mut stream: TcpStream) {
    // TODO: actual server here
    println!("connect");
    let content = "<h1>Howdy, Hacker!</h1>";
    let _ = stream.write(format(format_args!("HTTP/1.1 200 OK\nContent-Type: text/html\nContent-Length: {}\nConnection: close\n\n{}", content.len(), content)).as_bytes());
}

/// start a tor server & the local service connected to it
pub fn start(mut port: u16) -> (JoinHandle<()>, JoinHandle<Result<u8, Error>>, u16) {
    if port == 0 {
        port = pick_unused_port().unwrap();
    }
    let tor = Tor::new()
        .flag(TorFlag::SocksPort(0))
        .flag(TorFlag::HiddenServiceDir(location()))
        .flag(TorFlag::HiddenServiceVersion(HiddenServiceVersion::V3))
        .flag(TorFlag::HiddenServicePort(TorAddress::Port(port), None.into()))
        .flag(TorFlag::Log(log::LogLevel::Err))
        .start_background();

    let socket = spawn(move || {
        let listener = TcpListener::bind(SocketAddr::from(([0, 0, 0, 0], port))).unwrap();
        for stream in listener.incoming() {
            handle_client(stream.unwrap());
        }
    });

    return (socket, tor, port);
}

/// get the current onion-hostname from running tor-server
pub fn hostname() -> String {
    let mut content = String::new();
    let fname = format!("{}/hostname", location());
    let f = File::open(&fname);
    let _ = match f {
        Ok(mut file) => file.read_to_string(&mut content),
        Err(error) => panic!("Problem opening the file {}: {:?}", &fname, error),
    };
    return String::from(content.trim_end());
}