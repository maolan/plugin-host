fn main() {
    match std::env::var("CARGO_CFG_TARGET_OS").as_deref() {
        Ok("freebsd") => println!("cargo::rustc-link-search=native=/usr/local/lib"),
        Ok("openbsd") => println!("cargo::rustc-link-search=native=/usr/X11R6/lib"),
        _ => {}
    }
}
