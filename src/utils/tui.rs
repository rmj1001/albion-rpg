/// Create a string of dashes with desired length
///
/// Example: create_line_string(3) -> ---
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

/// Print a line of dashes to STDOUT. Default Length 80 characters.
pub fn print_line_string(total_length: Option<usize>) {
    let mut line_string: String = String::new();

    match total_length {
        None => line_string = create_line_string(80, None),
        Some(length) => line_string = create_line_string(length, None),
    }

    println!("{}", &line_string[..]);
}

/// Prints a header box to stdout with a custom line length. The title is centered between two lines.
pub fn header<T>(title: T, line_length: usize)
where
    T: Into<String>,
{
    let title = title.into();

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

    header.push_str(&title);

    header.push('\n');
    header.push_str(&create_line_string(line_length, None));

    println!("{}", header);
}

/// Clears terminal screen and prints big_header().
pub fn page_header<T>(title: T, instructions: HeaderSubtext)
where
    T: Into<String>,
{
    crate::utils::terminal::clearscr();
    big_header(title.into(), instructions)
}

/// Prints a header with 80 character width
pub fn big_header<T>(title: T, instructions: HeaderSubtext)
where
    T: Into<String>,
{
    header(format!("Albion - {}", title.into()), 80);
    header_subtext(instructions);
}

/// Prints a header with 40 character width
pub fn small_header<T>(title: T, instructions: HeaderSubtext)
where
    T: Into<String>,
{
    header(title.into(), 40);
    header_subtext(instructions);
}

pub enum HeaderSubtext {
    EnterCode,
    Keyboard,
    Other(&'static str),
    None,
}

/// Prints text under a header based on enum variant used
pub fn header_subtext(instructions: HeaderSubtext) {
    let mut instructions_string = String::new();

    match instructions {
        HeaderSubtext::Keyboard => instructions_string.push_str("Press ↑ or ↓ to navigate, then press ENTER/RETURN.\n"),
        HeaderSubtext::EnterCode => instructions_string.push_str("Enter a code (ex. p1), then press ENTER/RETURN.\n"),
        HeaderSubtext::Other(text) => instructions_string.push_str(&format!("{}\n", text)),
        HeaderSubtext::None => {}
    }

    println!("{}", instructions_string);
}

/// Equivalent to DOS "pause" command
pub fn press_enter_to_continue() {
    println!("[PRESS (RETURN/ENTER) TO CONTINUE]");
    let mut garbage = String::new();
    let _ = std::io::stdin().read_line(&mut garbage);
}

pub fn table_from_csv(strings: Vec<String>) {
    let table_string = strings.join("\n");
    let table = csv_to_table::iter::from_reader(table_string.as_bytes()).to_string();

    println!("{}\n", table);
}

pub fn pretty_bool(flag: bool) -> &'static str {
    match flag {
        true => "Yes",
        false => "No",
    }
}

pub fn checkmark(flag: bool) -> &'static str {
    match flag {
        true => "✓",
        false => "",
    }
}

pub fn price(number: usize) -> String {
    format!("{} gold", number)
}

pub fn sleep(seconds: u64) {
    std::thread::sleep(std::time::Duration::from_secs(seconds))
}

pub fn paginate_string<T>(string: T, lines_per_page: usize) -> Vec<String>
where
    T: Into<String>,
{
    let string: String = string.into();
    let lines: Vec<String> = string.split('\n').map(|s| format!("{}\n", s)).collect();

    let mut pages: Vec<String> = Vec::new();

    for chunk in lines.chunks(lines_per_page) {
        let mut page: String = String::new();
        for line in chunk {
            page.push_str(line);
        }

        pages.push(page);
    }

    pages
}

mod tests {
    #[test]
    fn line() {
        let output = crate::utils::tui::create_line_string(10, None);
        let compare: String = "----------".to_string();

        assert!(output == compare);
    }
}
