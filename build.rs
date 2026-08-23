use std::env;

fn main() {
    let target = env::var("TARGET").unwrap_or_default();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.toml");

    // Android NDK Linker configuration
    if target.contains("android") {
        println!("cargo:rustc-link-lib=static=c++_shared");
        println!("cargo:rustc-link-search=native=/opt/android-ndk/toolchains/llvm/prebuilt/linux-x86_64/sysroot/usr/lib");
    }

    // Directives for maximum binary compression
    if !target.contains("msvc") {
        println!("cargo:rustc-link-arg=-s"); // Strip symbols
    }
}
