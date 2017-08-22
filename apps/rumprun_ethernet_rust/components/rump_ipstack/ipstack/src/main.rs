#![feature(linkage)]

#[no_mangle]
extern "C" {
	// eth driver camkes
    #[linkage = "extern_weak"]
    static eth_ev_tx_emit: *const extern "C" fn();
    #[linkage = "extern_weak"]
    static eth_ev_rx_wait: *const extern "C" fn(); 
    #[linkage = "extern_weak"]
    static eth_buffer: *const *mut u8;
    
    // ipstack camkes
    #[linkage = "extern_weak"]
    static ipframe_ev_rx_emit: *const extern "C" fn();
    #[linkage = "extern_weak"]
    static ipframe_ev_tx_wait: *const extern "C" fn(); 
    #[linkage = "extern_weak"]
    static network_buffer: *const *mut u8;
    
}

fn main() {
    println!("Hello ipstack!");
    println!("Waiting for ipframe_ev_tx!");
    unsafe {
    (*((&eth_ev_rx_wait as *const *const extern "C" fn()) as *const extern "C" fn()))();	
    }
    
    
}

