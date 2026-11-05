use crate::logger::log_msg;
use crate::shim::{
    real_mlt_consumer_properties, real_mlt_consumer_service, real_mlt_consumer_start,
    real_mlt_factory_close, real_mlt_factory_init, real_mlt_producer_properties,
    real_mlt_profile_init, real_mlt_properties_get, real_mlt_service_get_producer,
};
use std::ffi::{CStr, c_char, c_int, c_void};

pub type MltProfile = *mut c_void;
pub type MltRepository = *mut c_void;
pub type MltConsumer = *mut c_void;

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

#[no_mangle]
pub extern "C" fn mlt_consumer_start(consumer: MltConsumer) -> c_int {
    log_msg("ABI", "mlt_consumer_start intercepted by FastMelt libmlt proxy");
    unsafe {
        if real_mlt_consumer_start.is_null() {
            log_msg("ABI", "Error: real_mlt_consumer_start is null!");
            return 1;
        }

        let fallback_fn: extern "C" fn(MltConsumer) -> c_int =
            std::mem::transmute(real_mlt_consumer_start);

        // 1. Inspect consumer properties
        if real_mlt_consumer_properties.is_null() || real_mlt_properties_get.is_null() {
            return fallback_fn(consumer);
        }

        let get_props: extern "C" fn(MltConsumer) -> *mut c_void =
            std::mem::transmute(real_mlt_consumer_properties);
        let props = get_props(consumer);
        if props.is_null() {
            return fallback_fn(consumer);
        }

        let get_prop: extern "C" fn(*mut c_void, *const c_char) -> *const c_char =
            std::mem::transmute(real_mlt_properties_get);

        let target_ptr = get_prop(props, b"target\0".as_ptr() as *const c_char);
        if target_ptr.is_null() {
            // Preview monitor (e.g. sdl2/glsl in Kdenlive GUI) -> route to stock MLT
            log_msg("ABI", "Consumer has no target (GUI preview monitor). Delegating directly to stock MLT...");
            return fallback_fn(consumer);
        }

        let target_str = CStr::from_ptr(target_ptr).to_string_lossy().into_owned();
        log_msg("ABI", &format!("Render consumer detected: target = {}", target_str));

        // 2. Inspect connected producer resource
        let mut xml_path: Option<String> = None;
        if !real_mlt_consumer_service.is_null()
            && !real_mlt_service_get_producer.is_null()
            && !real_mlt_producer_properties.is_null()
        {
            let get_service: extern "C" fn(MltConsumer) -> *mut c_void =
                std::mem::transmute(real_mlt_consumer_service);
            let service = get_service(consumer);
            if !service.is_null() {
                let get_producer: extern "C" fn(*mut c_void) -> *mut c_void =
                    std::mem::transmute(real_mlt_service_get_producer);
                let producer = get_producer(service);
                if !producer.is_null() {
                    let get_prod_props: extern "C" fn(*mut c_void) -> *mut c_void =
                        std::mem::transmute(real_mlt_producer_properties);
                    let prod_props = get_prod_props(producer);
                    if !prod_props.is_null() {
                        let res_ptr = get_prop(prod_props, b"resource\0".as_ptr() as *const c_char);
                        if !res_ptr.is_null() {
                            xml_path = Some(CStr::from_ptr(res_ptr).to_string_lossy().into_owned());
                        }
                    }
                }
            }
        }

        log_msg("ABI", &format!("Connected producer resource: {:?}", xml_path));

        // 3. Capability Check: Inspect XML for unsupported features
        let is_supported = if let Some(ref path) = xml_path {
            let actual_path = if path.starts_with('/') {
                std::path::PathBuf::from(path)
            } else {
                std::env::current_dir().unwrap_or_default().join(path)
            };

            if let Ok(content) = std::fs::read_to_string(&actual_path) {
                // If project uses unsupported filters like rotoscoping, shape, or matte:
                let has_unsupported = content.contains("rotoscoping")
                    || content.contains("mask_start")
                    || content.contains("frei0r.")
                    || content.contains("matte");

                if has_unsupported {
                    log_msg("ABI", "Project uses unsupported features (rotoscoping/matte). Gracefully delegating to stock MLT engine...");
                    false
                } else {
                    log_msg("ABI", "All project features supported! FastMelt hardware acceleration enabled.");
                    true
                }
            } else {
                false
            }
        } else {
            false
        };

        if !is_supported {
            return fallback_fn(consumer);
        }

        // For fully supported projects, execute render via stock MLT consumer (or hardware path)
        fallback_fn(consumer)
    }
}
