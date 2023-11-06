#![deny(clippy::all)]

use napi::Result;
use std::io::Read;
use std::{
  fs::File,
  path::Path
};

#[macro_use]
extern crate napi_derive;

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

    if !file_content.is_empty() {
      println!("{}", file_content);
    }
  }

  Ok(())
}
