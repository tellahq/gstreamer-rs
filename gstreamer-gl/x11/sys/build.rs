// Generated by gir (https://github.com/gtk-rs/gir @ 6fbc68a47551)
// from gir-files (https://github.com/gtk-rs/gir-files @ 7ebd4478b4a5)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git @ 8800300f1307)
// DO NOT EDIT

#[cfg(not(feature = "dox"))]
use std::process;

#[cfg(feature = "dox")]
fn main() {} // prevent linking libraries to avoid documentation failure

#[cfg(not(feature = "dox"))]
fn main() {
    if let Err(s) = system_deps::Config::new().probe() {
        println!("cargo:warning={}", s);
        process::exit(1);
    }
}
