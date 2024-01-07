/// Create a string of dashes with desired length
///
/// Example: create_line_string(3) // ---
pub fn create_line_string(total_length: usize) -> String {
    let mut line_string: String = String::new();
    let mut current_length: usize = 1;

    while current_length <= total_length {
        line_string.push('-');
        current_length += 1;
    }

    line_string
}

/// Print a line of dashes to STDOUT.
///
/// Default length is 80 characters
pub fn print_line_string(total_length: Option<usize>) {
    let mut line_string: String = String::new();

    match total_length {
        None => line_string = create_line_string(80),
        Some(length) => line_string = create_line_string(length),
    }

    println!("{}", &line_string[..]);
}

pub fn header(title: &str, line_length: usize) {
    fn add_spaces_to_string(s: &mut String, spaces: usize) {
        let mut index = 0;

        while index < spaces {
            s.push(' ');

            index += 1;
        }
    }

    let mut header = String::new();

    header.push_str(&create_line_string(line_length));
    header.push('\n');

    let spaces_on_one_side = (line_length - (title.len() + 2)) / 2 + 1;

    add_spaces_to_string(&mut header, spaces_on_one_side);

    header.push_str(title);

    header.push('\n');
    header.push_str(&create_line_string(line_length));

    println!("{}", header);
}

/// Prints a header with a title, using a line of dashes on the top
/// and bottom. The title is centered.
pub fn page_header(title: &str, instructions: Option<&str>) {
    crate::lib::terminal::clear_screen();

    header(&format!("Albion - {}", title), 80);

    if let Some(instruction_text) = instructions {
        println!("{}\n", instruction_text);
    } else {
        println!("\n");
    }
}

pub fn sub_header(title: &str) {
    header(title, 40);
}

/// Equivalent to DOS "pause" command
pub fn press_enter_to_continue() {
    println!("[PRESS RETURN/ENTER TO CONTINUE.]");
    let mut garbage = String::new();
    let _ = std::io::stdin().read_line(&mut garbage);
}

pub fn invalid_input(input: Option<&str>) {
    match input {
        Some(data) => {
            println!("\nInvalid input '{}'.", data);
            press_enter_to_continue();
        }

        None => {
            println!("\nInvalid input.");
            press_enter_to_continue();
        }
    }
}

pub mod dialogue {
    /// Returns the index of the element in the vector selected.
    pub fn selector(
        options: &[&str],
        default_index: usize,
        optional_prompt: Option<&str>,
    ) -> usize {
        match optional_prompt {
            Some(prompt) => {
                if prompt.is_empty() {
                    dialoguer::Select::new()
                        .items(options)
                        .default(0)
                        .interact()
                        .unwrap_or(default_index)
                } else {
                    dialoguer::Select::new()
                        .with_prompt(prompt)
                        .items(options)
                        .default(0)
                        .interact()
                        .unwrap_or(default_index)
                }
            }
            None => dialoguer::Select::new()
                .with_prompt(
                    "Use ↑ ↓ keys to select an option below, then press ENTER/RETURN to run it",
                )
                .items(options)
                .default(0)
                .interact()
                .unwrap_or(default_index),
        }
    }

    pub fn prompt_input(prompt: &str) -> String {
        dialoguer::Input::new()
            .with_prompt(prompt)
            .interact_text()
            .unwrap()
    }

    pub fn password() -> String {
        dialoguer::Password::new()
            .with_prompt("Password")
            .interact()
            .unwrap()
    }
}
