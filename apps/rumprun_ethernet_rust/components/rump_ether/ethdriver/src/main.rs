#![feature(linkage)]

#[no_mangle]
extern "C" {
    #[linkage = "extern_weak"]
    static eth_ev_rx_emit: *const extern "C" fn();
    #[linkage = "extern_weak"]
    static eth_ev_tx_wait: *const extern "C" fn(); 
    #[linkage = "extern_weak"]
    static eth_buffer: *const *mut u8;
}

/*
// fails because it needs libc definitions such as 
error[E0432]: unresolved import `self::libc::SOCK_NONBLOCK`
  --> /home/podhrad/.cargo/registry/src/github.com-1ecc6299db9ec823/lazy-socket-0.2.1/src/raw/unix.rs:79:9
   |
79 |         SOCK_NONBLOCK,
   |         ^^^^^^^^^^^^^ no `SOCK_NONBLOCK` in the root. Did you mean to use `O_NONBLOCK`?

error[E0432]: unresolved import `self::libc::SOCK_CLOEXEC`
  --> /home/podhrad/.cargo/registry/src/github.com-1ecc6299db9ec823/lazy-socket-0.2.1/src/raw/unix.rs:80:9
   |
80 |         SOCK_CLOEXEC
   |         ^^^^^^^^^^^^ no `SOCK_CLOEXEC` in the root. Did you mean to use `O_CLOEXEC`?

error: aborting due to 2 previous errors

-> maybe implement netbsd version? 
https://github.com/DoumanAsh/lazy-socket.rs/blob/master/src/raw/mod.rs
*/
//extern crate lazy_socket;
//
//use std::net;
//use std::str::FromStr;
//
//use lazy_socket::raw::{
//    Socket,
//    Family,
//    Protocol,
//    Type,
//    select
//};
//
//fn main() {
//    let timeout = 1000;
//    let socket = match Socket::new(Family::IPv4, Type::STREAM, Protocol::TCP) {
//        Ok(socket) => socket,
//        Err(error) => {
//            println!("Couldn't open socket. Erro: {}", error);
//            return;
//        }
//    };
//
//    let dest = net::SocketAddr::from_str("192.168.0.1:80").unwrap();
//
//    let _ = socket.set_blocking(false);
//    let _ = socket.connect(&dest);
//    match select(&[], &[&socket], &[&socket], Some(timeout)) {
//          Ok(_) => println!("Connected!"),
//          Err(error) => println!("Failed to connect. Error:{}", error)
//    }
//}


/*
fails on libc defines too:

error[E0425]: cannot find value `AF_PACKET` in module `libc`
  --> /home/podhrad/.cargo/registry/src/github.com-1ecc6299db9ec823/smoltcp-0.3.0/src/phy/sys/raw_socket.rs:14:44
   |
14 |             let lower = libc::socket(libc::AF_PACKET, libc::SOCK_RAW,
   |                                            ^^^^^^^^^ not found in `libc`

error[E0433]: failed to resolve. Use of undeclared type or module `imp`
  --> /home/podhrad/.cargo/registry/src/github.com-1ecc6299db9ec823/smoltcp-0.3.0/src/phy/sys/raw_socket.rs:15:38
   |
15 |                                      imp::ETH_P_ALL.to_be() as i32);
   |                                      ^^^ Use of undeclared type or module `imp`

error[E0433]: failed to resolve. Use of undeclared type or module `imp`
  --> /home/podhrad/.cargo/registry/src/github.com-1ecc6299db9ec823/smoltcp-0.3.0/src/phy/sys/raw_socket.rs:27:50
   |
27 |         ifreq_ioctl(self.lower, &mut self.ifreq, imp::SIOCGIFMTU).map(|mtu| mtu as usize)
   |                                                  ^^^ Use of undeclared type or module `imp`

error[E0422]: cannot find struct, variant or union type `sockaddr_ll` in module `libc`
  --> /home/podhrad/.cargo/registry/src/github.com-1ecc6299db9ec823/smoltcp-0.3.0/src/phy/sys/raw_socket.rs:31:30
   |
31 |         let sockaddr = libc::sockaddr_ll {
   |                              ^^^^^^^^^^^ did you mean `sockaddr_in`?

error[E0425]: cannot find value `AF_PACKET` in module `libc`
  --> /home/podhrad/.cargo/registry/src/github.com-1ecc6299db9ec823/smoltcp-0.3.0/src/phy/sys/raw_socket.rs:32:33
   |
32 |             sll_family:   libc::AF_PACKET as u16,
   |                                 ^^^^^^^^^ not found in `libc`

error[E0433]: failed to resolve. Use of undeclared type or module `imp`
  --> /home/podhrad/.cargo/registry/src/github.com-1ecc6299db9ec823/smoltcp-0.3.0/src/phy/sys/raw_socket.rs:33:27
   |
33 |             sll_protocol: imp::ETH_P_ALL.to_be() as u16,
   |                           ^^^ Use of undeclared type or module `imp`

error[E0433]: failed to resolve. Use of undeclared type or module `imp`
  --> /home/podhrad/.cargo/registry/src/github.com-1ecc6299db9ec823/smoltcp-0.3.0/src/phy/sys/raw_socket.rs:34:73
   |
34 |             sll_ifindex:  try!(ifreq_ioctl(self.lower, &mut self.ifreq, imp::SIOCGIFINDEX)),
   |                                                                         ^^^ Use of undeclared type or module `imp`

error[E0412]: cannot find type `sockaddr_ll` in module `libc`
  --> /home/podhrad/.cargo/registry/src/github.com-1ecc6299db9ec823/smoltcp-0.3.0/src/phy/sys/raw_socket.rs:43:60
   |
43 |                                  &sockaddr as *const libc::sockaddr_ll as *const libc::sockaddr,
   |                                                            ^^^^^^^^^^^ did you mean `sockaddr_in`?

error[E0412]: cannot find type `sockaddr_ll` in module `libc`
  --> /home/podhrad/.cargo/registry/src/github.com-1ecc6299db9ec823/smoltcp-0.3.0/src/phy/sys/raw_socket.rs:44:55
   |
44 |                                  mem::size_of::<libc::sockaddr_ll>() as u32);
   |                                                       ^^^^^^^^^^^ did you mean `sockaddr_in`?

error: aborting due to 9 previous errors

error: Could not compile `smoltcp`.

*/
//extern crate smoltcp;
//
//use std::env;
//use smoltcp::phy::{Device, RawSocket};
//use smoltcp::wire::{PrettyPrinter, EthernetFrame};
//
//fn main() {
//    let ifname = env::args().nth(1).unwrap();
//    let mut socket = RawSocket::new(ifname.as_ref()).unwrap();
//    loop {
//        let buffer = socket.receive(/*timestamp=*/0).unwrap();
//        print!("{}", PrettyPrinter::<EthernetFrame<&[u8]>>::new("", &buffer))
//    }
//}


// try with nix -> raw sockets dont work, we use regular UDP sockets for start (and that works)
extern crate nix;
extern crate rand;

use nix::sys::socket::*;

fn main() {
	let fd = socket(AddressFamily::Inet, SockType::Datagram, SockFlag::empty(), IPPROTO_UDP).expect("Unable to create socket");
	println!("Socket created, fd={}",fd);
	
	let mut v: Vec<u8> = vec![0;255];
	println!("Just created vector {} bytes long",v.len());
	
	// bind socket
	match bind(fd, &SockAddr::new_inet(InetAddr::new(IpAddr::new_v4(192,168,179,50), 6666))) {
			Ok(_) => println!("Bind OK!"),
			Err(e) => println!("error binding socket: {}",e),
		}
	
	// no raw sockets in netbd?
	let len = recv(fd, &mut v, MsgFlags::empty()).expect("Error receiving data");
	
	println!("Either way, here is the buffer {:?}",v);
	
	// copy the buffer
	unsafe {
		for idx in 0..len{
			 *(*eth_buffer).offset(idx as isize) = v[idx];
		}
	}
	
	// notify the ipstack
	unsafe {
	    (*((&eth_ev_rx_emit as *const *const extern "C" fn()) as *const extern "C" fn()))();	
    }
	
	
}


// an innocent main
//fn main() {
//	println!("Hello rust");
//}


// try with nix -> use bfp
// We get "no such file or directory problem
// see http://infosecwriters.com/text_resources/pdf/raw_tcp.pdf
//extern crate nix;
//
//use nix::sys::ioctl::*;
//use nix::fcntl::*;
//use nix::sys::stat::Mode;
//
//
//fn main() {
//	// "/dev/bpfX"
//	// fd = open((char *) &buf, O_RDWR);
//	let path = "/dev/bpf0";
//	let fd = open(path, O_RDWR, Mode::empty());
//	match fd {
//		Ok(fd) => println!("File opened, fd={}",fd),
//		Err(e) => println!("error opening file: {}",e),
//	}
//}