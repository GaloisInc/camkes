#![feature(linkage)]

fn main() {
    println!("Hello rust!");
    unsafe {
    *(*camkes_buffer).offset(0) = 'a' as u8;
    *(*camkes_buffer).offset(1) = 'b' as u8;
    *(*camkes_buffer).offset(2) = 'c' as u8;
    *(*camkes_buffer).offset(3) = '\0' as u8;
			println!("RUST: Buffer: {}", *(*camkes_buffer).offset(0) as char);
			println!("RUST: Buffer: {}", *(*camkes_buffer).offset(1) as char);
			println!("RUST: Buffer: {}", *(*camkes_buffer).offset(2) as char);
        (*((&camkes_ev_emit as *const *const extern "C" fn()) as *const extern "C" fn()))();
    }
    println!("Waiting for event!");
    unsafe {
        std::mem::transmute::<*const extern "C" fn(), extern "C" fn()>(camkes_ev1_wait)();
    }
    println!("Got event!");
    unsafe {
			println!("RUST: Buffer: {}", *(*camkes_buffer).offset(0) as char);
			println!("RUST: Buffer: {}", *(*camkes_buffer).offset(1) as char);
			println!("RUST: Buffer: {}", *(*camkes_buffer).offset(2) as char);
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
