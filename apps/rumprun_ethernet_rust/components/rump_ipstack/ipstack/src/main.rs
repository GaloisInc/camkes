#![feature(linkage)]

#[no_mangle]
extern "C" {
    // eth driver camkes
    #[linkage = "extern_weak"]
    static eth_ev_tx_emit: *const extern "C" fn();

    #[linkage = "extern_weak"]
    static eth_ev_rx_poll: *const extern "C" fn();

    #[linkage = "extern_weak"]
    static eth_buffer: *const *mut u8;


    // ipstack camkes
    #[linkage = "extern_weak"]
    static ipframe_ev_rx_emit: *const extern "C" fn();

    #[linkage = "extern_weak"]
    static ipframe_ev_tx_poll: *const extern "C" fn();

    #[linkage = "extern_weak"]
    static network_buffer: *const *mut u8;

}


// Notify the ethdriver about data to send
fn eth_emit_tx_data() {
    unsafe {
        (*((&eth_ev_tx_emit as *const *const extern "C" fn()) as *const extern "C" fn()))();
    }
}

// Wait for data from ethdriver
fn eth_poll_rx_data() -> bool {
    let res = unsafe {
        (*((&eth_ev_rx_poll as *const *const extern "C" fn()) as *const extern "C" fn() -> i32))()
    };
    match res {
        0 => false,
        1 => true,
        _ => panic!("Unexpected return code res={}", res),
    }
}

// copy from a buffer to eth_buffer
fn eth_copy_data_from(buf: &[u8], len: usize) {
    // TODO: check for mutex
    unsafe {
        *(*eth_buffer).offset(0) = len as u8;
        for idx in 0..len {
            *(*eth_buffer).offset(idx as isize + 1) = buf[idx];
        }
    }
}

// copy from eth_buffer to a buffer
fn eth_copy_data_to(buf: &mut Vec<u8>) -> usize {
    // TODO: check for mutex
    unsafe {
        let len = *(*eth_buffer).offset(0) as usize;
        buf[0] = len as u8;
        for idx in 1..len + 1 {
            buf[idx] = *(*eth_buffer).offset(idx as isize);
        }
        len
    }
}





// Notify the application about received data
fn ip_emit_rx_data() {
    unsafe {
        (*((&ipframe_ev_rx_emit as *const *const extern "C" fn()) as *const extern "C" fn()))();
    }
}

// Wait for data to send from the application
fn ip_poll_tx_data() -> bool {
    let res = unsafe {
        (*((&ipframe_ev_tx_poll as *const *const extern "C" fn()) as
               *const extern "C" fn() -> i32))()
    };
    match res {
        0 => false,
        1 => true,
        _ => panic!("Unexpected return code res={}", res),
    }
}

// copy from a buffer to network_buffer
fn ip_copy_data_from(buf: &[u8], len: usize) {
    // TODO: check for mutex
    unsafe {
        *(*network_buffer).offset(0) = len as u8;
        for idx in 0..len {
            *(*network_buffer).offset(idx as isize + 1) = buf[idx];
        }
    }
}

// copy from network_buffer to a buffer
fn ip_copy_data_to(buf: &mut Vec<u8>) -> usize {
    // TODO: check for mutex
    unsafe {
        let len = *(*network_buffer).offset(0) as usize;
        buf[0] = len as u8;
        for idx in 1..len + 1 {
            buf[idx] = *(*network_buffer).offset(idx as isize);
        }
        len
    }
}



use std::{thread, time};
//extern crate smoltcp;

fn main() {
    println!("Hello ipstack!");
    //thread::sleep(time::Duration::from_millis(5000));
    
    println!("Spwaning thread eth_RX: wait for rx data from ethdriver ");
    let t_eth_rx = thread::spawn(move || {
        // allocate buffer
        let mut buf: Vec<u8> = vec![0; 1500];

        println!("eth_RX entering main loop");
        loop {
            if eth_poll_rx_data() {
            	
                let msg_len = eth_copy_data_to(&mut buf);
                
                println!("got eth_poll_rx_data, len={}, buffer = {:?}",msg_len, &buf[0..msg_len+1]);

                // just pass through
                // TODO: fancy networking magic

                // copy data to network_buffer
                ip_copy_data_from(&buf[1..msg_len+1], msg_len);

                // notify the app
                ip_emit_rx_data();
            }
            thread::sleep(time::Duration::from_millis(1));
            thread::yield_now();
        }
    });

    println!("Spwaning thread ip_RX: wait for rx data from the application ");
    let t_ip_rx = thread::spawn(move || {
        // allocate buffer
        let mut buf: Vec<u8> = vec![0; 1500];

        println!("eth_RX entering main loop");
        loop {
            if ip_poll_tx_data() {
                let msg_len = ip_copy_data_to(&mut buf);
                
                println!("got eth_poll_rx_data, len={}, buffer = {:?}",msg_len, &buf[0..msg_len+1]);

                // just pass through
                // TODO: fancy networking magic

                // copy data to eth_buffer
                eth_copy_data_from(&buf[1..msg_len+1], msg_len);

                // notify the eth_driver to send data
                eth_emit_tx_data();
            }
            thread::sleep(time::Duration::from_millis(1));
            thread::yield_now();
        }
    });

    println!("Waiting for threads to end");
    match t_eth_rx.join() {
        Ok(_) => println!("t_eth_rx All well"),
        Err(e) => println!("t_eth_rx thread joined with an error {:?}", e), 
    }
    match t_ip_rx.join() {
        Ok(_) => println!("t_ip_rx All well"),
        Err(e) => println!("t_ip_rx thread joined with an error {:?}", e), 
    }
}
