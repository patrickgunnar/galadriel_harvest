#![deny(clippy::all)]

use napi::Result;
use std::io::Read;
use std::{fs::File, path::Path};

#[macro_use]
extern crate napi_derive;

fn clear_white_spaces_and_break_lines_from_code(code: String) -> Result<String> {
  // quotes control
  let mut inside_quotes = false;
  // result string
  let mut result = String::new();

  // loops through the code
  for c in code.chars() {
    // if char matches a double or single quotes
    match c {
      '\'' | '"' => {
        // change the value of the quote control
        inside_quotes = !inside_quotes;
        // push the char into the result
        result.push(c);
      }
      // if the current char is a comma outside quotes
      // push {BREAK} into the result
      ',' if !inside_quotes => {
        result.push('&');
        result.push('B');
        result.push('9');
        result.push('4');
        result.push('#');
        result.push('K');
        result.push(';');
      }
      // if char is a white space
      ' ' if !inside_quotes => continue,
      // else, push the char into the result
      _ => result.push(c),
    }
  }

  // return the result replacing the line breaks
  Ok(result.replace('\n', ""))
}

fn collects_crafting_styles_from_code(code: String) -> Vec<String> {
  // craftingStyles to be returned
  let mut collected_crafting_styles: Vec<String> = Vec::new();
  // control to be used when the loop reach the craftingStyles
  let mut inside_crafting_styles = false;
  // the current collected craftingStyles
  let mut current_crafting_styles_data = String::new();
  // control of the number of parenthesis
  let mut paren_count = 0;

  // loops through the code
  // get the char and char's index
  for (i, c) in code.chars().enumerate() {
    // if inside the craftingStyles is set to true
    if inside_crafting_styles {
      // push the current char into the current callback store
      current_crafting_styles_data.push(c);

      // if the current char is an opened parenthesis
      // if it is a block statement, like: functions, conditions, loops, etc.
      if c == '(' {
        // sum 1 to the parenthesis' control
        paren_count += 1;
        // if the current char is a closing parenthesis
        // if it is the closing of a block statement, like: functions, conditions, loops, etc.
      } else if c == ')' {
        // takes 1 from the parenthesis' control
        paren_count -= 1;

        // if the parenthesis control is 0
        // it reached to the end of the craftingStyles structure
        // like: craftingStyles(COLLECTED CODE)
        if paren_count == 0 {
          // set the inside craftingStyles control to false
          inside_crafting_styles = false;
          // push the collected craftingStyles into a its store array
          collected_crafting_styles.push(current_crafting_styles_data.clone());
          // clears the current data store
          current_crafting_styles_data.clear();
        }
      }
      // if the current char is "C"
    } else if c == 'c' {
      // gets from the code string, the current char to the end
      if let Some(remaining) = code.get(i..i + 15) {
        // if the remaining content starts with "craftingStyles("
        // if it is the craftingStyles statement block
        if remaining.starts_with("craftingStyles(") {
          // set the inside craftingStyles control to true
          inside_crafting_styles = true;
          // push the current char into the current data store
          current_crafting_styles_data.push(c);
        }
      }
    }
  }

  // return the collected functions
  collected_crafting_styles
}

#[napi]
pub fn process_content(path: String) -> Result<()> {
  // checks if the file exists
  if Path::new(&path).exists() {
    // attempt to open the file
    let mut file = File::open(&path)?;
    // Create a mutable string to store the file content
    let mut file_content = String::new();

    // Read the file content into the string
    file.read_to_string(&mut file_content)?;

    // if the file content is not empty
    if !file_content.is_empty() {
      // removes all the white spaces outside quotes and break lines
      let clean_code = clear_white_spaces_and_break_lines_from_code(file_content)?;

      // if the clean_code is not empty
      if !clean_code.is_empty() {
        // collects the craftingStyles data from the code
        let collected_handlers = collects_crafting_styles_from_code(clean_code);

        // if the collected_handlers is not empty
        if !collected_handlers.is_empty() {
          for (_i, crafting_styles) in collected_handlers.iter().enumerate() {
            let crafting_styles_parts: Vec<&str> = crafting_styles.split("&B94#K;").collect();

            for part in crafting_styles_parts {
              println!("{}", part);
            }
            println!("\n");
          }
        }
      }
    }
  }

  Ok(())
}
