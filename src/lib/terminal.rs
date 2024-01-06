pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

pub fn exit() {
    clear_screen();
    std::process::exit(0);
}
