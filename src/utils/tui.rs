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

    for _ in 1..=total_length {
        line_string.push('-');
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
    let length = total_length.unwrap_or(80);
    println!("{}", line(length));
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
    let mut header = format!("{}\n", line(line_length));

    for _ in 0..=((line_length - (title.to_string().len() + 2)) / 2) {
        header.push(' ');
    }

    header.push_str(&format!("{title}\n"));
    header.push_str(&line(line_length));

    println!("{header}");
}

/**
Clears the screen and prints a header.
The text "Albion - " is prefixed to the given
title.

# Example

```
use albion_terminal_rpg::prelude::{page_header, Instructions};

page_header("Main Menu", &Instructions::TypeCode);
```
*/
pub fn page_header<T: Display>(title: T, instructions: &Instructions) {
    crate::utils::terminal::clearscr();
    header(format!("Albion - {title}"), 80);
    println!("{instructions}\n");
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
        write!(
            f,
            "{}",
            match self {
                Self::TypeCode => "Press ↑ or ↓ to navigate, then press ENTER/RETURN.",
                Self::Keyboard => "Enter a code (ex. 1), then press ENTER/RETURN.",
                Self::Other(text) => text,
                Self::None => "",
            }
        )
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

csv_table(&[
    "Column1,Column2,Column3".to_string(),
    "Hello,World,Yes".to_string(),
    "Goodbye,World,No".to_string(),
]);
```
*/
pub fn csv_table(strings: &[String]) {
    let table_string = strings.join("\n");
    let table = csv_to_table::iter::from_reader(table_string.as_bytes()).to_string();

    println!("{table}\n");
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
    if flag {
        '✓'
    } else {
        ' '
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
