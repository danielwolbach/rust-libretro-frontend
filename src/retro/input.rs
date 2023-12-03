pub unsafe extern "C" fn set_input_poll() {}

pub unsafe extern "C" fn set_input_state(
    _port: libc::c_uint,
    _device: libc::c_uint,
    _index: libc::c_uint,
    _id: libc::c_uint,
) -> i16 {
    0
}
