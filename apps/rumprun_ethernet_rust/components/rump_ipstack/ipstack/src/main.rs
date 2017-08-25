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
    
    let mut len = 0;
    // copy to a buffer
    unsafe {
    	let mut idx: isize = 0;
    	let mut val = *(*eth_buffer).offset(0) as u8;
    	
    	while val!=0 {
    		*(*network_buffer).offset(idx) = val;
    		
    		idx +=1;
    		val = *(*eth_buffer).offset(idx) as u8;
    	}
    	
    	len = idx;
    }

	// display
    let mut msg = String::new();
    for idx in 0..len {
    	unsafe {
    		msg.push(*(*network_buffer).offset(idx) as char);
    	}
    }    
    println!("Got data! {}",msg);
    
    // notify receiver
    println!("Notifying string_reverse");
    unsafe {
	    (*((&ipframe_ev_rx_emit as *const *const extern "C" fn()) as *const extern "C" fn()))();	
    }
    
    println!("waiting for response");
    unsafe {
	    (*((&ipframe_ev_tx_wait as *const *const extern "C" fn()) as *const extern "C" fn()))();	
    }
    
    let mut reversed_msg = String::new();
    for idx in 0..len {
    	unsafe {
    		reversed_msg.push(*(*network_buffer).offset(idx) as char);
    	}
    }
    println!("Received {}", reversed_msg);
    
}

