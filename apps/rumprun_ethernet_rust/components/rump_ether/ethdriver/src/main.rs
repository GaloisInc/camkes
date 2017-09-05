#![feature(linkage)]

#[no_mangle]
extern "C" {
    #[linkage = "extern_weak"]
    static eth_ev_rx_emit: *const extern "C" fn();
    
    #[linkage = "extern_weak"]
    static eth_ev_tx_poll: *const extern "C" fn();
    
    #[linkage = "extern_weak"]
    static eth_buffer: *const *mut u8; // length is stored as the first byte of the buffer
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
fn poll_tx_data() -> bool {
    let res = unsafe {
        (*((&eth_ev_tx_poll as *const *const extern "C" fn()) as *const extern "C" fn() -> i32))()
    };
    match res {
        0 => false,
        1 => true,
        _ => panic!("Unexpected return code res={}",res),
    }
}

// copy the buffer
fn copy_data_from(buf: &[u8], len: usize) {
    // TODO: check for mutex
    unsafe {
        *(*eth_buffer).offset(0) = len as u8;
        for idx in 0..len {
            *(*eth_buffer).offset(idx as isize + 1) = buf[idx];
        }
    }
}

fn copy_data_to(buf: &mut Vec<u8>) -> usize {
    // TODO: check for mutex
    unsafe {
        let len = *(*eth_buffer).offset(0) as usize;
        for idx in 1..len + 1 {
            buf[idx] = *(*eth_buffer).offset(idx as isize);
        }
        len + 1
    }
}


// UDP Socket
use std::net::UdpSocket;
use std::thread;

// Note: sleep suspends whole Rumpkernel component, not cool...
fn main() {
    println!("Spwaning thread RX");
    // reads data from the socket
    let trx = thread::spawn(move || {
        println!("RX Waiting for a socket to bind");
        // bind to an IP address assigned to an existing interface & specific port
        let socket = UdpSocket::bind("192.168.179.50:6666").expect("couldn't bind to address");
        socket
            .set_nonblocking(true)
            .expect("set_nonblocking call failed");

        // allocate buffer
        let mut buf: Vec<u8> = vec![0; 1500];

        println!("RX entering main loop");
        loop {
            if let Ok(res) = socket.recv_from(&mut buf) {
                let (len, src) = res;
                // println!("Received {} data from {}: {:?}", len, src, &buf[0..len]);

                copy_data_from(&buf, len);

                emit_rx_data();
            }
            thread::yield_now();
        }
    });

    println!("Spwaning thread TX");
    // waits for data to be sned
    let ttx = thread::spawn(move || {
        println!("TX Waiting for a socket to bind");
        // bind to an IP address assigned to an existing interface & specific port
        let socket = UdpSocket::bind("192.168.179.50:6665").expect("couldn't bind to address");
        socket
            .set_nonblocking(true)
            .expect("set_nonblocking call failed");

        // allocate buffer
        let mut buf: Vec<u8> = vec![0; 1500];


        println!("TX entering main loop");
        loop {
            if poll_tx_data() {
                let msg_len = copy_data_to(&mut buf);

                //println!("TX sending data: {:?}", &buf[1..msg_len]);
                
                // send data
                let len = socket
                    .send_to(&buf[1..msg_len], "192.168.179.1:6665")
                    .expect("Couldn't send data");
                println!("Sent {} bytes", len);
            }
            thread::yield_now();
        }
    });

    println!("Waiting for threads to end");
    match trx.join() {
        Ok(_) => println!("Trx All well"),
        Err(e) => println!("Trx thread joined with an error {:?}", e), 
    }
    match ttx.join() {
        Ok(_) => println!("Ttx All well"),
        Err(e) => println!("Ttx thread joined with an error {:?}", e), 
    }
}
