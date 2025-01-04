fn main() {
    // This tells Cargo not to use the static SDL2 library
    println!("cargo:rustc-link-lib=dylib=sdl2");
}