/// Panics if the operating system is not supported by the program.
pub fn deter_unsupported_os() {
    let os = std::env::consts::OS;

    let supported_operating_systems: Vec<&str> =
        vec!["linux", "macos", "freebsd", "dragonfly", "netbsd", "openbsd", "windows"];

    if !supported_operating_systems.contains(&os) {
        panic!("This program does not support {}.", os);
    }
}
