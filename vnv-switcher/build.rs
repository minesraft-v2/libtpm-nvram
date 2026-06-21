fn main() {
    // Compile the native C hardware implementation layer
    cc::Build::new()
        .file("native/hardware.c")
        .include("native/include")
        .compile("native_hardware");

    // Enforce recompilation hooks if native source files are altered
    println!("cargo:rerun-if-changed=native/hardware.c");
    println!("cargo:rerun-if-changed=native/include/hardware.h");
}
