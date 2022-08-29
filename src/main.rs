#![deny(warnings, unsafe_code, clippy::all)]

const VERSION: &str = env!("CARGO_PKG_VERSION");

struct SystemInformation {
    system: &'static str,
    arch: &'static str,
    family: &'static str,
}

impl SystemInformation {
    fn gather() -> Self {
        use std::env::consts::*;

        SystemInformation {
            system: OS,
            family: FAMILY,
            arch: ARCH,
        }
    }
}

fn main() {
    let info = SystemInformation::gather();

    println!(
        "osinfo {VERSION}\n\nOperating System: {}\nSystem Architecture: {}\nOS Family: {}",
        info.system, info.arch, info.family
    );
}
