#[no_mangle]
extern "C" {
  fn printf(val: *const i8, ...);
  fn l_lock();
  fn l_unlock();
  fn get_instance_name() -> *const i8;
}

/*

int run(void) {
    printf("%s: starting...\n", get_instance_name());
    l_lock();
    printf("%s: got lock!\n", get_instance_name());
    l_unlock();
    printf("%s: released\n", get_instance_name());
    return 0;
}
*/

#[no_mangle]
pub extern "C" fn run() -> isize {
    // println!("Hello, RUST!!");
    unsafe{
      printf(b"RUST %s: starting...\n\0".as_ptr() as *const i8, get_instance_name());
      l_lock();
      printf(b"RUST %s: got lock...\n\0".as_ptr() as *const i8, get_instance_name());
      l_unlock();
      printf(b"RUST %s: released...\n\0".as_ptr() as *const i8, get_instance_name());
    }
    0
}

