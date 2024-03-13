/*!
# Terminal User Interface

Design terminal menus for users to navigate.
*/

use std::fmt::Display;

/**
Create a string of dashes with desired length.

# Examples

```
use albion_terminal_rpg::prelude::line;

let line1 = line(3);
let line2 = line(5);

assert_eq!(line1, String::from("---"));
assert_eq!(line2, String::from("-----"));
```
 */
pub fn line(total_length: usize) -> String {
    let mut line_string: String = String::new();
    let mut current_length: usize = 1;
    const LINE_CHAR: char = '-';

    while current_length <= total_length {
        line_string.push(LINE_CHAR);
        current_length += 1;
    }

    line_string
}

/**
Print a line of dashes to STDOUT.
Default Length 80 characters.

# Examples

```
use albion_terminal_rpg::prelude::print_line;

print_line(Some(10)); // prints a line of dashes with a length of 10
print_line(None); // prints a line of dashes with the default length (80)
```
*/
pub fn print_line(total_length: Option<usize>) {
    let line_string: String = match total_length {
        None => line(80),
        Some(length) => line(length),
    };

    println!("{}", line_string);
}

/**
Prints a header box to stdout with a custom line length.
The title is centered between two lines.

# Example

```
use albion_terminal_rpg::prelude::header;

// prints a header with a title centered between two
// lines of 10 dashes
header("Albion", 10)
```
*/
pub fn header<T: Display>(title: T, line_length: usize) {
    fn add_spaces_to_string(s: &mut String, spaces: usize) {
        let mut index = 0;

        while index < spaces {
            s.push(' ');

            index += 1;
        }
    }

    let mut header = String::new();

    header.push_str(&line(line_length));
    header.push('\n');

    let spaces_on_one_side = (line_length - (title.to_string().len() + 2)) / 2 + 1;

    add_spaces_to_string(&mut header, spaces_on_one_side);

    header.push_str(&title.to_string());

    header.push('\n');
    header.push_str(&line(line_length));

    println!("{}", header);
}

/**
Clears the screen and prints a header.
The text "Albion - " is prefixed to the given
title.

# Example

```
use albion_terminal_rpg::prelude::{page_header, Instructions};

page_header("Main Menu", Instructions::TypeCode);
```
*/
pub fn page_header<T: Display>(title: T, instructions: Instructions) {
    crate::utils::terminal::clearscr();
    header(format!("Albion - {}", title), 80);
    println!("{}\n", instructions);
}

/**
Pre-generated prompts displayed under headers to give players instructions
for navigating the game. This implement the Display trait for easy printing.

# Examples

```
use albion_terminal_rpg::prelude::Instructions;

println!("{}", Instructions::TypeCode);
println!("{}", Instructions::Keyboard);
println!("{}", Instructions::Other("Some text here"));
println!("{}", Instructions::None);
```
*/
pub enum Instructions {
    TypeCode,
    Keyboard,
    Other(&'static str),
    None,
}

impl Display for Instructions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TypeCode => write!(f, "Press ↑ or ↓ to navigate, then press ENTER/RETURN."),
            Self::Keyboard => write!(f, "Enter a code (ex. 1), then press ENTER/RETURN."),
            Self::Other(text) => write!(f, "{}", text),
            Self::None => write!(f, ""),
        }
    }
}

/**
Equivalent of DOS pause command. This waits for the user to press the
ENTER/RETURN key on their keyboard before moving on to other code.

# Usage

```
use albion_terminal_rpg::prelude::pause;

pause();
```
*/
pub fn pause() {
    println!("[PRESS (RETURN/ENTER) TO CONTINUE]");
    let mut garbage = String::new();
    let _ = std::io::stdin().read_line(&mut garbage);
}

/**
Print out a table to STDOUT based on a vector of strings with comma separators.

# Example

```
use albion_terminal_rpg::prelude::csv_table;

csv_table(vec![
    "Column1,Column2,Column3".to_string(),
    "Hello,World,Yes".to_string(),
    "Goodbye,World,No".to_string(),
]);
```
*/
pub fn csv_table(strings: Vec<String>) {
    let table_string = strings.join("\n");
    let table = csv_to_table::iter::from_reader(table_string.as_bytes()).to_string();

    println!("{}\n", table);
}

/**
Prints a check mark if the boolean is true, or a space if false.

# Examples

```
use albion_terminal_rpg::prelude::checkmark;

let yes = checkmark(true);
let no = checkmark(false);

assert_eq!(yes, '✓');
assert_eq!(no, ' ');
```
*/
pub fn checkmark(flag: bool) -> char {
    match flag {
        true => '✓',
        false => ' ',
    }
}

mod tests {
    #[test]
    fn line() {
        let output = crate::utils::tui::line(10);
        let compare: String = "----------".to_string();

        assert!(output == compare);
    }
}
