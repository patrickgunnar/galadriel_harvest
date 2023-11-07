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
    // if char matches a double quote
    match c {
      '"' => {
        // change the value of the quote control
        inside_quotes = !inside_quotes;
        // push the char into the result
        result.push(c);
      }// if char is a white space
      ' ' if !inside_quotes => continue,
      // else, push the char into the result
      _ => result.push(c),
    }
  }

  // return the result replacing the line breaks
  Ok(result.replace('\n', ""))
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
      let result = clear_white_spaces_and_break_lines_from_code(file_content)?;

      println!("{}", result);
    }
  }

  Ok(())
}
