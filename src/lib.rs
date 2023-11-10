#![deny(clippy::all)]

mod ast {
  pub mod core_ast;
}

use ast::core_ast::CORE_AST;

use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::{Mutex, Arc};
use napi::Result;
use serde_json::Value;
use std::io::Read;
use std::{
  fs::{self, File},
  path::Path,
};
use sha2::{Digest, Sha256};

#[macro_use]
extern crate napi_derive;

// lazy static controls
lazy_static! {
  static ref GENERATED_CSS_STYLES: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

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
        result.push('"');
      }
      // if the current char is a comma outside quotes
      // push &B94#K; (break) tag into the result
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

fn collects_objects_from_crafting_styles(crafting_styles: String) -> Vec<Value> {
  // split the string into the the &B94#K; tags
  let crafting_styles_parts: Vec<&str> = crafting_styles.split("&B94#K;").collect();
  // accumulator to store the collected properties
  let mut accumulator: Vec<Value> = Vec::new();
  // temporary store the nested properties
  let mut nested_content: Vec<String> = Vec::new();
  // nested control
  let mut is_nested = false;

  // if the crafting_styles_parts is not empty
  if !crafting_styles_parts.is_empty() {
    // loops through the parts
    for part in crafting_styles_parts {
      // if part includes an opened curly bracket
      if part.contains("{") {
        // loop through splitted str
        for item in part.split("{") {
          // if item includes a colon
          if item.contains(":") {
            // if item last sign is a colon
            if item.ends_with(":") {
              is_nested = true;
              nested_content.push(item.to_string().clone());

              // if item contains an closed curly bracket
            } else if item.contains("}") {
              // loop through the splitted item
              for el in item.split("}") {
                // if el includes a colon
                if el.contains(":") {
                  // if is a nested object operation
                  if is_nested {
                    // if el ends with an 2 closed curly bracket
                    if el.ends_with("}}") {
                      nested_content.push(el.to_string().clone());
                      accumulator.push(Value::Array(
                        nested_content
                          .clone()
                          .into_iter()
                          .map(Value::String)
                          .collect(),
                      ));
                      nested_content.clear();
                      is_nested = false;
                    } else {
                      nested_content.push(el.to_string().clone());
                    }
                  } else {
                    accumulator.push(Value::String(el.to_string().clone()));
                  }
                }
              }
              // if item does not include an equals sign
            } else if !item.contains("=") {
              // if is a nested object operation
              if is_nested {
                nested_content.push(item.to_string().clone());
              } else {
                accumulator.push(Value::String(item.to_string().clone()));
              }
            }
          }
        }
        // if part includes a closed  curly bracket
      } else if part.contains("}") {
        // loop through the splitted part
        for item in part.split("}") {
          // if item includes a colon
          if item.contains(":") {
            // if is a nested object operation
            if is_nested {
              nested_content.push(item.to_string().clone());
            } else {
              accumulator.push(Value::String(item.to_string().clone()))
            }
          }
        }

        // if is a nested object operation
        if is_nested {
          accumulator.push(Value::Array(
            nested_content
              .clone()
              .into_iter()
              .map(Value::String)
              .collect(),
          ));
          nested_content.clear();
          is_nested = false;
        }
        // if part includes colon and not include an equals sign
      } else if part.contains(":") && !part.contains("=") {
        // if is a nested object operation
        if is_nested {
          nested_content.push(part.to_string().clone());
        } else {
          accumulator.push(Value::String(part.to_string().clone()));
        }
      }
    }
  }

  accumulator
}

fn collects_galadriel_config() -> Option<Value> {
  // read the dir folder
  if let Ok(entries) = fs::read_dir(".") {
    // loops through all entries in the directory
    for entry in entries {
      if let Ok(entry) = entry {
        // get the name of the entry
        let file_name = entry.file_name();

        // file name, convert it to a string
        if let Some(file_str) = file_name.to_str() {
          // if file name is equal to "galadriel.json"
          if file_str == "galadriel.json" {
            // get the file path
            let file_path = entry.path();

            // read the content of the file
            if let Ok(content) = fs::read_to_string(&file_path) {
              // return a json containing the data
              if let Ok(json) = serde_json::from_str::<Value>(&content) {
                return Some(json);
              }
            }
          }
        }
      }
    }
  }

  None
}

fn generates_hashing_hex(str: String, is_96_bits: bool, is_32_bits: bool) -> String {
  // instantiate the hasher
  let mut hasher = Sha256::new();
  // updates the hasher with the string
  hasher.update(str);
  // finalize the digest
  let digest_hash = hasher.finalize();
  // collects the hashed string
  let hex_string: String = digest_hash.iter().rev().map(|byte| format!("{:02x}", byte)).collect();

  if is_96_bits { // if it's to return 12 chars
    // returns the last 12 chars
    return hex_string.chars().rev().take(12).collect();
  } else if is_32_bits { // if it's to return 4 chars
    // returns the last 4 chars
    return  hex_string.chars().rev().take(4).collect();
  } else { // if it's to return 8 chars
    // returns the last 8 chars
    return  hex_string.chars().rev().take(8).collect();
  }
}

fn append_style_to_styles_ast(key: String, class_rules: String, media: String) -> () {
  // lock the core ast to access the styles
  let mut core_ast_map = CORE_AST.lock().unwrap();
  // control variable
  let mut found = false;

  // loops through ast's nodes
  for (_, node) in core_ast_map.iter_mut() {
    // loops through node's properties
    for (property, data) in node.into_iter() {
      // if media is not empty and media is equal to property
      // or media is empty and key is equal to property
      if !media.is_empty() && property == &media || media.is_empty() && property == &key {
        data.push(class_rules.clone());
        found = true;
        break;
      }
    }
  }

  if !found { // if not found an existing property
    // get the other properties node
    let other_properties = core_ast_map.entry(
      "otherProperties".to_string()
    ).or_insert_with(|| HashMap::new());

    // if media is not empty, return media
    // else, return the key
    let entry_name = if !media.is_empty() { media.clone() } else { key.clone() };
    // gets or creates an instance in other properties
    let property = other_properties.entry(entry_name).or_insert_with(|| Vec::new());

    property.push(class_rules.clone());
  }
}

fn generates_css_rules_from_crafting_styles_data(objects_array: Vec<Value>, is_modular: bool, file_path: String) -> String {
  // loops over all objects
  for object in objects_array {
    if let Value::String(value) = object {
      // if the current property is key:value type
      if !value.contains("{") {
        // extracts the key and value
        let parts: Vec<String> = value.split(":").map(|s| s.to_string()).collect();

        // if the current property have key and data
        if let [key, data] = parts.as_slice() {
          if data.contains("$") {
            // generates the class name
            // collects the value from data and replaces the "$" inside it by an empty string
            let class_name = format!(
              "{}{}", serde_json::from_str::<String>(data).unwrap_or_default().replace("$", "").replace(" ", ""),
              if is_modular && !file_path.is_empty() { // if is a modular config and file_path is not empty
                format!("-{}", generates_hashing_hex(file_path.clone(), false, true)) // hash the file path and return the hashed string
              } else {
                "".to_string() // return an empty string
              }
            );

            // lock the mutex to access the hash map
            let generated_styles_map =  GENERATED_CSS_STYLES.lock().unwrap();

            // if the current selector was already used
            if generated_styles_map.contains_key(&class_name.clone()) {
              // if modular config is on
              if is_modular {
                // collects the styles
                match generated_styles_map.get(&class_name.clone()) {
                  Some(styles) => { // append the styles into the ast
                    append_style_to_styles_ast(key.to_string(), styles.to_string(), "".to_string());
                  },
                  None => {}
                }
              }

              continue;
            }

            // temp code
            append_style_to_styles_ast(key.to_string(), "AQUI".to_string(), "".to_string());
            //println!("{}", class_name);
          }
        }
      }
    } else {}
  }

  String::new()
}

#[napi]
pub fn process_content(path: String) -> Result<()> {
  // collects the contents of the galadriel config file
  let galadriel_config_data = collects_galadriel_config();
  // control to check if exists a valid config
  let mut config_control = false;
  // modular state
  let mut is_modular = false;

  // if the file exists
  if let Some(config) = galadriel_config_data.clone() {
    // get the modular config
    if let Some(module_value) = config.get("module") {
      // if the value is a boolean
      if module_value.is_boolean() {
        // if the value is true
        if module_value.as_bool().unwrap_or(false) {
          config_control = true;
          is_modular = true;
        }
      }
    }

    if !config_control {
      // if the config control stills false
      // get the output config
      if let Some(module_value) = config.get("output") {
        // if the value is a strung
        if module_value.is_string() {
          // collects the output value
          let output = module_value.as_str().unwrap_or_default();

          // if the output is not empty
          if !output.is_empty() {
            config_control = true;
          }
        }
      }
    }
  }

  // checks if the file exists and the config is valid
  if Path::new(&path).exists() && config_control {
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
            // collects the objects properties from the crafting styles callback
            let objects_array = collects_objects_from_crafting_styles(crafting_styles.to_string());

            // the objects array is not empty
            if !objects_array.is_empty() {
              // generates the CSS rules from the objects array
              let generated_classes = generates_css_rules_from_crafting_styles_data(objects_array, is_modular, path.clone());

              // if the generated classes is not empty
              if !generated_classes.is_empty() {}
            }
          }

          // Use dbg! macro to print and inspect the content
          let data = Arc::new(Mutex::new(CORE_AST.lock().unwrap().clone()));
          dbg!(&data);
        }
      }
    }
  }

  Ok(())
}
