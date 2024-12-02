use std::env;

fn main() {
    println!("cargo::rerun-if-changed=src/main.c");

    let target = env::var("CARGO_TARGET_DIR").unwrap_or(env::var("OUT_DIR").unwrap());
    cc::Build::new()
        .file("src/main.c")
        .flag(format!("-L{target}"))
        .flag("-lrustaveli")
        .compile("main");
}
