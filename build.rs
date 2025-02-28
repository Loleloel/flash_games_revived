fn main() {
    // This build script doesn't need to do anything yet
    // In the future, you could add custom build steps here,
    // such as processing assets or generating code
    println!("cargo:rerun-if-changed=static/");
}