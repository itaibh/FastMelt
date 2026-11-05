use crate::logger::log_msg;
use crate::shim::{real_mlt_factory_close, real_mlt_factory_init, real_mlt_profile_init};
use std::ffi::{CStr, c_char, c_void};

pub type MltProfile = *mut c_void;
pub type MltRepository = *mut c_void;

#[no_mangle]
pub extern "C" fn mlt_factory_init(directory: *const c_char) -> MltRepository {
    log_msg("ABI", "mlt_factory_init intercepted by FastMelt proxy");
    unsafe {
        if !real_mlt_factory_init.is_null() {
            let func: extern "C" fn(*const c_char) -> MltRepository =
                std::mem::transmute(real_mlt_factory_init);
            let repo = func(directory);
            log_msg("ABI", &format!("real_mlt_factory_init returned repository pointer: {:?}", repo));
            repo
        } else {
            log_msg("ABI", "Error: real_mlt_factory_init is null!");
            std::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub extern "C" fn mlt_factory_close() {
    log_msg("ABI", "mlt_factory_close intercepted by FastMelt proxy");
    unsafe {
        if !real_mlt_factory_close.is_null() {
            let func: extern "C" fn() = std::mem::transmute(real_mlt_factory_close);
            func();
        }
    }
}

#[no_mangle]
pub extern "C" fn mlt_profile_init(name: *const c_char) -> MltProfile {
    let name_str = if !name.is_null() {
        unsafe { CStr::from_ptr(name).to_string_lossy().into_owned() }
    } else {
        "default".to_string()
    };
    log_msg("ABI", &format!("mlt_profile_init intercepted (profile: {})", name_str));
    unsafe {
        if !real_mlt_profile_init.is_null() {
            let func: extern "C" fn(*const c_char) -> MltProfile =
                std::mem::transmute(real_mlt_profile_init);
            let prof = func(name);
            prof
        } else {
            std::ptr::null_mut()
        }
    }
}
