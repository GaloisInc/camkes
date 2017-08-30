#![feature(linkage)]

#[no_mangle]
extern "C" {
    #[linkage = "extern_weak"]
    static eth_ev_rx_emit: *const extern "C" fn();
    #[linkage = "extern_weak"]
    static eth_ev_tx_wait: *const extern "C" fn();
    #[linkage = "extern_weak"]
    static eth_buffer: *const *mut u8; // TODO: use a better struct, maybe something with length?
}

// TODO: Should have two scokets and some mutex over the data buffer
// Better EthDriver interface - rx,tx and mac buf, plus the mutex?


// Notify the ipstack about the received data
fn emit_rx_data() {
    unsafe {
        (*((&eth_ev_rx_emit as *const *const extern "C" fn()) as *const extern "C" fn()))();
    }
}

// Wait for data to send
// TODO: how do I know how much data?
fn wait_tx_data() {
    unsafe {
        (*((&eth_ev_tx_wait as *const *const extern "C" fn()) as *const extern "C" fn()))();
    }
}

// copy the buffer
fn copy_data_from(buf: &[u8], len: usize) {
    // TODO: check for mutex
    unsafe {
        for idx in 0..len {
            *(*eth_buffer).offset(idx as isize) = buf[idx];
        }
    }
}

fn copy_data_to(buf: &mut [u8], len: usize) {
    // TODO: check for mutex
    unsafe {
        for idx in 0..len {
            buf[idx] = *(*eth_buffer).offset(idx as isize);
        }
    }
}


// UDP Socket
use std::net::UdpSocket;
use std::{thread,time};


fn main() {



    println!("Spwaning thread RX");
    // reads data from the socket
    let trx = thread::spawn(move || {
        println!("Waiting for a socket to bind");
        // bind to an IP address assigned to an existing interface & specific port
        let socket = UdpSocket::bind("192.168.179.50:6666").expect("couldn't bind to address");
        // read from the socket
        let mut buf: Vec<u8> = vec![0; 1500];

        loop {
            let (len, src) = socket.recv_from(&mut buf).expect("Didn't receive data");
            println!("Received {} data from {}", len, src);

            copy_data_from(&buf, len);

            emit_rx_data();
        }
    });

    println!("Spwaning thread TX");
    // waits for data to be sned
    let ttx = thread::spawn(move || {
        // bind to an IP address assigned to an existing interface & specific port
        let socket = UdpSocket::bind("192.168.179.50:6665").expect("couldn't bind to address");
        let mut buf: Vec<u8> = vec![0; 1500];

        loop {
            println!("Waiting for data to send");
            wait_tx_data(); // this freezes execution of whole rumpkernel :-(
            println!("TX data received");

            // TODO: figure lenght
            copy_data_to(buf.as_mut_slice(), 255);

            // send data
            let len = socket
                .send_to(&buf, "192.168.179.1:6665")
                .expect("Couldn't send data");
            println!("Sent {} bytes", len);
        }
    });
    
    println!("Launching Thread counter");
    let tcnt = thread::spawn(move || {
    		let mut counter = 0;
    		loop {
    			println!("Time is {} [s]",counter);
    			counter += 1;
    			thread::sleep(time::Duration::from_millis(1000));
    		}
    	}
    );

    println!("Waiting for threads to end");
    match trx.join() {
        Ok(_) => println!("Trx All well"),
        Err(e) => println!("Trx thread joined with an error {:?}", e), 
    }
    match ttx.join() {
        Ok(_) => println!("Ttx All well"),
        Err(e) => println!("Ttx thread joined with an error {:?}", e), 
    }
    match tcnt.join() {
        Ok(_) => println!("Tcnt All well"),
        Err(e) => println!("Tcnt thread joined with an error {:?}", e), 
    }
}
