use libloading::Library;
use std::ffi::{CStr, c_char, c_int, c_void};
use std::path::PathBuf;

#[test]
fn test_libmlt_shim_export_and_forwarding() {
    let target_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/release");
    let lib_path = target_dir.join("libmlt_7.so");

    if !lib_path.exists() {
        eprintln!("libmlt_7.so not built yet, skipping test");
        return;
    }

    unsafe {
        let lib = Library::new(&lib_path).expect("Failed to load libmlt_7.so");

        // 1. Test intercepted symbol: mlt_factory_init
        type MltFactoryInitFn = unsafe extern "C" fn(*const c_char) -> *mut c_void;
        let factory_init: libloading::Symbol<MltFactoryInitFn> = lib
            .get(b"mlt_factory_init\0")
            .expect("Symbol mlt_factory_init missing");
        let init_res = factory_init(std::ptr::null());
        println!("mlt_factory_init returned: {:?}", init_res);

        // 2. Test unintercepted forwarded symbol: mlt_version_get_string
        type MltVersionGetStringFn = unsafe extern "C" fn() -> *const c_char;
        let version_get_str: libloading::Symbol<MltVersionGetStringFn> = lib
            .get(b"mlt_version_get_string\0")
            .expect("Symbol mlt_version_get_string missing");
        let ver_ptr = version_get_str();
        assert!(!ver_ptr.is_null(), "mlt_version_get_string returned null");
        let ver_str = CStr::from_ptr(ver_ptr).to_string_lossy();
        println!("mlt_version_get_string returned: {}", ver_str);
        assert!(
            ver_str.starts_with("7."),
            "Expected MLT version 7.x, got: {}",
            ver_str
        );

        // 3. Test unintercepted forwarded symbol: mlt_profile_init
        type MltProfileInitFn = unsafe extern "C" fn(*const c_char) -> *mut c_void;
        type MltProfileCloseFn = unsafe extern "C" fn(*mut c_void);

        let profile_init: libloading::Symbol<MltProfileInitFn> = lib
            .get(b"mlt_profile_init\0")
            .expect("Symbol mlt_profile_init missing");
        let profile_close: libloading::Symbol<MltProfileCloseFn> = lib
            .get(b"mlt_profile_close\0")
            .expect("Symbol mlt_profile_close missing");

        let prof = profile_init(std::ptr::null());
        assert!(!prof.is_null(), "mlt_profile_init returned null profile");
        profile_close(prof);

        // 4. Test intercepted symbol: mlt_factory_close
        type MltFactoryCloseFn = unsafe extern "C" fn();
        let factory_close: libloading::Symbol<MltFactoryCloseFn> = lib
            .get(b"mlt_factory_close\0")
            .expect("Symbol mlt_factory_close missing");
        factory_close();
    }
}

