/// Panics if the operating system is not supported by the program.
pub fn detect_os() {
    let os = std::env::consts::OS;

    match os {
        "solaris" => {
            panic!("This program does not support Solaris.");
        }
        "ios" => {
            panic!("This program does not support iOS.");
        }
        "android" => {
            panic!("This program does not support Android.");
        }
        "windows" => {
            panic!("This program does not support Windows.");
        }
        _ => {}
    }
}
