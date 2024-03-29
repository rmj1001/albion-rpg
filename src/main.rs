use albion_terminal_rpg::panic_menu;

fn main() {
    let os = std::env::consts::OS;

    let supported_operating_systems: Vec<&str> =
        vec!["linux", "macos", "freebsd", "dragonfly", "netbsd", "openbsd", "windows"];

    if !supported_operating_systems.contains(&os) {
        panic_menu!("This program does not support {}.", os);
    }

    albion_terminal_rpg::menus::accounts::main();
}
