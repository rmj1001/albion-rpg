/// Create a string of dashes with desired length
///
/// Example: create_line_string(3) // ---
pub fn create_line_string(total_length: usize, optional_character: Option<char>) -> String {
    let mut line_string: String = String::new();
    let mut current_length: usize = 1;
    let mut line_character = '-';

    if let Some(character) = optional_character {
        line_character = character;
    }

    while current_length <= total_length {
        line_string.push(line_character);
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
        None => line_string = create_line_string(80, None),
        Some(length) => line_string = create_line_string(length, None),
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

    header.push_str(&create_line_string(line_length, None));
    header.push('\n');

    let spaces_on_one_side = (line_length - (title.len() + 2)) / 2 + 1;

    add_spaces_to_string(&mut header, spaces_on_one_side);

    header.push_str(title);

    header.push('\n');
    header.push_str(&create_line_string(line_length, None));

    println!("{}", header);
}

pub enum HeaderInstructions {
    EnterCode,
    Keyboard,
    Other(String),
    None,
}

/// Prints a header with a title, using a line of dashes on the top
/// and bottom. The title is centered.
pub fn page_header(title: &str, instructions: HeaderInstructions) {
    crate::lib::terminal::clear();

    header(&format!("Albion - {}", title), 80);

    let mut instructions_string: String = String::new();

    match instructions {
        HeaderInstructions::Keyboard => {
            instructions_string.push_str("Press ↑ or ↓ to navigate, then press ENTER/RETURN.")
        }
        HeaderInstructions::EnterCode => {
            instructions_string.push_str("Enter a code (ex. p1), then press ENTER/RETURN.")
        }
        HeaderInstructions::Other(text) => instructions_string.push_str(&text),
        HeaderInstructions::None => {}
    }

    println!("{}\n", instructions_string);
}

pub fn small_header(title: &str, instructions: HeaderInstructions) {
    header(title, 40);

    let mut instructions_string = String::new();

    match instructions {
        HeaderInstructions::Keyboard => {
            instructions_string.push_str("Press ↑ or ↓ to navigate, then press ENTER/RETURN.")
        }
        HeaderInstructions::EnterCode => {
            instructions_string.push_str("Enter a code (ex. p1), then press ENTER/RETURN.")
        }
        HeaderInstructions::Other(text) => instructions_string.push_str(&text),
        HeaderInstructions::None => {
            return;
        }
    }

    println!("{}\n", instructions_string);
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
