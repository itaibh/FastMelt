use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();

    let dispatch_c = PathBuf::from(&manifest_dir).join("src/shim/mlt_dispatch.c");
    let trampolines_s = PathBuf::from(&manifest_dir).join("src/shim/mlt_trampolines.s");
    let version_script = PathBuf::from(&manifest_dir).join("mlt.version");

    println!("cargo:rerun-if-changed={}", dispatch_c.display());
    println!("cargo:rerun-if-changed={}", trampolines_s.display());
    println!("cargo:rerun-if-changed={}", version_script.display());

    let dispatch_o = PathBuf::from(&out_dir).join("mlt_dispatch.o");
    let trampolines_o = PathBuf::from(&out_dir).join("mlt_trampolines.o");
    let shim_a = PathBuf::from(&out_dir).join("libmlt_shim.a");

    // 1. Compile mlt_dispatch.c
    let gcc_status = Command::new("gcc")
        .args(&[
            "-c",
            "-fPIC",
            "-O2",
            dispatch_c.to_str().unwrap(),
            "-o",
            dispatch_o.to_str().unwrap(),
        ])
        .status()
        .expect("Failed to execute gcc to compile mlt_dispatch.c");
    if !gcc_status.success() {
        panic!("gcc compilation of mlt_dispatch.c failed");
    }

    // 2. Assemble mlt_trampolines.s
    let as_status = Command::new("as")
        .args(&[
            trampolines_s.to_str().unwrap(),
            "-o",
            trampolines_o.to_str().unwrap(),
        ])
        .status()
        .expect("Failed to execute as to assemble mlt_trampolines.s");
    if !as_status.success() {
        panic!("as assembly of mlt_trampolines.s failed");
    }

    // 3. Create static library libmlt_shim.a
    let ar_status = Command::new("ar")
        .args(&[
            "rcs",
            shim_a.to_str().unwrap(),
            dispatch_o.to_str().unwrap(),
            trampolines_o.to_str().unwrap(),
        ])
        .status()
        .expect("Failed to execute ar to create libmlt_shim.a");
    if !ar_status.success() {
        panic!("ar creation of libmlt_shim.a failed");
    }

    // 4. Link directives
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-arg={}", dispatch_o.display());
    println!("cargo:rustc-link-arg={}", trampolines_o.display());
    println!("cargo:rustc-link-lib=dl");

    // Pass version script and soname to cdylib linker
    println!(
        "cargo:rustc-cdylib-link-arg=-Wl,--version-script={}",
        version_script.display()
    );
    println!("cargo:rustc-cdylib-link-arg=-Wl,-soname,libmlt-7.so.7");
}
