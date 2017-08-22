#![feature(linkage)]

use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn main() {
    println!("TCP listener starting!");
    let listener = TcpListener::bind("192.168.179.50:6666").unwrap();

  match listener.accept() {
    Ok((socket, addr)) => {
      println!("new client: {:?}", addr);
      handle_client(socket);
    },
    Err(e) => println!("couldn't get client: {:?}", e), 
  }

    println!("Rust done");
}

fn handle_client(mut stream: TcpStream) {
  let mut buf: Vec<u8> = vec![0; 255];
  let len = stream.read(buf.as_mut_slice()).unwrap();

  unsafe {
    for idx in 0..len {
      *(*camkes_buffer).offset(idx as isize) = buf[idx] as u8;
    }

    println!("camkes_ev_emit!");
    (*((&camkes_ev_emit as *const *const extern "C" fn()) as *const extern "C" fn()))();
    println!("camkes ev wait!");
        //std::mem::transmute::<*const extern "C" fn(), extern "C" fn()>(camkes_ev1_wait)();
    (*((&camkes_ev1_wait as *const *const extern "C" fn()) as *const extern "C" fn()))();
        
        for idx in 0..len as isize {
        	println!("RUST: Buffer: {}", *(*camkes_buffer).offset(idx) as char);
        }
  }

}

#[no_mangle]
extern "C" {
    #[linkage = "extern_weak"]
    static camkes_ev_emit: *const extern "C" fn();
    #[linkage = "extern_weak"]
    static camkes_ev1_wait: *const extern "C" fn(); 
    #[linkage = "extern_weak"]
    static camkes_buffer: *const *mut u8;
}
