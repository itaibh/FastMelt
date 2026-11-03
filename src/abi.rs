use std::ffi::{c_char, c_int, c_void};

pub type MltProfile = *mut c_void;
pub type MltProducer = *mut c_void;
pub type MltTractor = *mut c_void;
pub type MltConsumer = *mut c_void;
pub type MltFrame = *mut c_void;

#[no_mangle]
pub extern "C" fn mlt_factory_init(_directory: *const c_char) -> c_int {
    log::info!("OliveMltBridge: mlt_factory_init called");
    0
}

#[no_mangle]
pub extern "C" fn mlt_factory_close() {
    log::info!("OliveMltBridge: mlt_factory_close called");
}

#[no_mangle]
pub extern "C" fn mlt_profile_init(_name: *const c_char) -> MltProfile {
    // Return dummy profile pointer
    Box::into_raw(Box::new(1080u32)) as MltProfile
}

#[no_mangle]
pub extern "C" fn mlt_profile_load_file(_path: *const c_char) -> MltProfile {
    Box::into_raw(Box::new(1920u32)) as MltProfile
}

#[no_mangle]
pub extern "C" fn mlt_producer_init() -> MltProducer {
    Box::into_raw(Box::new(1u32)) as MltProducer
}

#[no_mangle]
pub extern "C" fn mlt_tractor_init() -> MltTractor {
    Box::into_raw(Box::new(2u32)) as MltTractor
}

#[no_mangle]
pub extern "C" fn mlt_consumer_init() -> MltConsumer {
    Box::into_raw(Box::new(3u32)) as MltConsumer
}

#[no_mangle]
pub extern "C" fn mlt_consumer_start(_consumer: MltConsumer) -> c_int {
    0
}

#[no_mangle]
pub extern "C" fn mlt_consumer_stop(_consumer: MltConsumer) -> c_int {
    0
}
