#[no_mangle]
extern "C" {
  fn printf(val: *const i8);
  fn m_lock();
  fn m_unlock();
}

/*
void a_lock(void) {
    (void)m_lock();
}
*/
#[no_mangle]
pub extern "C" fn a_lock()  {
  unsafe{ printf(b"a_lock()\0".as_ptr() as *const i8); }
  unsafe{m_lock();}
}

/*
void a_unlock(void) {
    (void)m_unlock();
}
*/
#[no_mangle]
pub extern "C" fn a_unlock()  {
  unsafe{ printf(b"a_unlock()\0".as_ptr() as *const i8); }
  unsafe{m_unlock();}
}

/*
void b_lock(void) {
    (void)m_lock();
}
*/
#[no_mangle]
pub extern "C" fn b_lock()  {
  unsafe{ printf(b"b_lock()\0".as_ptr() as *const i8); }
  unsafe{m_lock();}
}

/*
void b_unlock(void) {
    (void)m_unlock();
}
*/
#[no_mangle]
pub extern "C" fn b_unlock()  {
  unsafe{ printf(b"b_unlock()\0".as_ptr() as *const i8); }
  unsafe{m_unlock();}
}

/*
void c_lock(void) {
    (void)m_lock();
}
*/
#[no_mangle]
pub extern "C" fn c_lock()  {
  unsafe{ printf(b"c_lock()\0".as_ptr() as *const i8); }
  unsafe{m_lock();}
}

/*
void c_unlock(void) {
    (void)m_unlock();
}
*/
#[no_mangle]
pub extern "C" fn c_unlock()  {
  unsafe{ printf(b"c_unlock()\0".as_ptr() as *const i8); }
  unsafe{m_unlock();}
}
