#![deny(clippy::all)]

mod ast {
  pub mod core_ast;
}

mod core {
  pub mod static_core;
  pub mod dynamic_core;
}

use ast::core_ast::CORE_AST;

use core::{
  static_core::STATIC_CORE,
  dynamic_core::DYNAMIC_CORE
};

use lazy_static::lazy_static;
use std::collections::HashMap;
use linked_hash_map::LinkedHashMap;
use std::sync::Mutex;
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
  static ref CRAFT_STYLES_JSON: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

fn collects_static_core_data(key: String, property: String) -> Option<Vec<(String, String)>> {
  // loops through the static core content
  for (container, elements) in STATIC_CORE.iter() {
    // if the container is equal to the key
    if container == &key {
      // loops through the elements inside the container
      for (element, value) in elements {
        // if the element is equal to the property
        if element == &property {
          let mut result = Vec::new();

          // extracts the value's properties
          for (css_property, css_value) in value {
            result.push((css_property.clone(), css_value.clone()));
          }

          // returns the value's rules
          return Some(result);
        }
      }
    }
  }

  None
}

fn collects_dynamic_core_data(key: String) -> Option<String> {
  // loops through the dynamic properties
  for (container, element) in DYNAMIC_CORE.iter() {
    // if the container is equal to the key
    if container == &key {
      // returns the container's value
      return Some(element.clone());
    }
  }

  None
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
        result.push(c);
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

fn collects_objects_from_crafting_styles(crafting_styles: String) -> Vec<String> {
  // split the string into the the &B94#K; tags
  let crafting_styles_parts: Vec<&str> = crafting_styles.split("&B94#K;").collect();
  // accumulator to store the collected properties
  let mut accumulator: Vec<String> = Vec::new();
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
                      accumulator.push(
                        serde_json::to_string(&nested_content).unwrap()
                      );
                      nested_content.clear();
                      is_nested = false;
                    } else {
                      nested_content.push(el.to_string().clone());
                    }
                  } else {
                    accumulator.push(el.to_string().clone());
                  }
                }
              }
              // if item does not include an equals sign
            } else if !item.contains("=") {
              // if is a nested object operation
              if is_nested {
                nested_content.push(item.to_string().clone());
              } else {
                accumulator.push(item.to_string().clone());
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
              accumulator.push(item.to_string().clone())
            }
          }
        }

        // if is a nested object operation
        if is_nested {
          accumulator.push(
            serde_json::to_string(&nested_content).unwrap()
          );
          nested_content.clear();
          is_nested = false;
        }
        // if part includes colon and not include an equals sign
      } else if part.contains(":") && !part.contains("=") {
        // if is a nested object operation
        if is_nested {
          nested_content.push(part.to_string().clone());
        } else {
          accumulator.push(part.to_string().clone());
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
        // if node does not contain the current rules
        if !data.contains(&class_rules) {
          data.push(class_rules.clone());
          found = true;
          break;
          // if the current node does contain the current rules
        } else if data.contains(&class_rules) {
          found = true;
          break;
        }
      }
    }
  }

  if !found { // if not found an existing property
    // get the other properties node
    let other_properties = core_ast_map.entry(
      "otherProperties".to_string()
    ).or_insert_with(|| LinkedHashMap::new());

    // if media is not empty, return media
    // else, return the key
    let entry_name = if !media.is_empty() { media.clone() } else { key.clone() };
    // gets or creates an instance in other properties
    let property = other_properties.entry(entry_name).or_insert_with(|| Vec::new());

    // if current CSS class does not exist in property
    if !property.contains(&class_rules.clone()) {
      property.push(class_rules.clone());
    }
  }
}

fn process_css_rules(value: String, is_modular: bool, file_path: String, pseudo: String, css_file_creation: &mut bool) -> () {
  // extracts the key and value
  let parts: Vec<String> = value.split(":").map(|s| s.to_string()).collect();
  // if the pseudo is not empty
  let pseudo_property = if !pseudo.is_empty() {
    // collects the pseudo property from dynamic value
    if let Some(pseudo_key) = collects_dynamic_core_data(pseudo.to_string()) {
      pseudo_key.to_string()
    } else { "".to_string() }
  } else { "".to_string() };

  // if the current property have key and data
  if let [key, data] = parts.as_slice() {
    // transform the json into a string
    let transformed_json = serde_json::from_str::<String>(data).unwrap_or_default();

    // returns the transformed json
    let string_data = if !transformed_json.is_empty() {
      transformed_json
    } else { // replace the single quotes to double quotes and transform the json into a string
      serde_json::from_str::<String>(&data.replace("'", "\"")).unwrap_or_default()
    };
    
    // if is a modular config and file_path is not empty
    // hash the file path and return the hashed string
    let modular_name = if is_modular && !file_path.is_empty() {
      format!("-{}", generates_hashing_hex(file_path.clone(), false, true))
    } else { // return an empty string
      "".to_string() 
    };

    // checks if the pseudo is a media
    let media = if pseudo_property.starts_with("#") { pseudo.to_string() } else { "".to_string() };

    // if the current pseudo is a media
    // hash the media and return the hashed string
    let media_name = if !media.is_empty() { 
      format!("-{}", generates_hashing_hex(media.clone(), false, true)) 
    } else { // return an empty string
      "".to_string() 
    };

    // formatted string data
    let formatted_data = string_data.clone().replace("$", "").replace(" ", "");

    // generates the class name
    let class_name = if string_data.contains("$") {
      // replaces the "$" inside it by an empty string and removes spaces
      format!("{}{}{}", formatted_data, media_name, modular_name)
    } else {  // transform the key and the its value into key:value format
      format!("galadriel_{}{}{}", 
        generates_hashing_hex(format!("{}:{}", key.clone().to_string(), string_data.clone()), false, false),
        media_name, modular_name
      )
    };

    // lock the mutex to access the hash map
    let mut generated_styles_map =  GENERATED_CSS_STYLES.lock().unwrap();

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

      return;
    }

    if string_data.starts_with("$") { // if the current data starts with "$"
      // if the collects static handler returns true
      if let Some(collected_rules) = collects_static_core_data(key.to_string(), string_data.clone()) {
        // extracts property:value pairs from the returned data
        for (collected_property, collected_value) in collected_rules.iter() {
          let class_rules = format!( // creates the CSS utility class
            ".{}{} {{ {}: {} }}", class_name,
            if !pseudo_property.starts_with("#") { pseudo_property.clone() } else { "".to_string() }, 
            collected_property, collected_value
          );

          // insert the utility class into the tracker
          generated_styles_map.insert(class_name.to_string(), class_rules.to_string());
          // insert the utility class into the ast
          append_style_to_styles_ast(key.to_string(), class_rules.to_string(), media.clone());
          // set the CSS file creation to true
          *css_file_creation = true;
        }
      } else {
        // lock the craft styles to store data
        let craft_styles_map = CRAFT_STYLES_JSON.lock().unwrap();

        if let Some(content) = craft_styles_map.get("craftStyles") {
          // parsed json content
          let parsed_craft_styles = serde_json::from_str::<HashMap<String, HashMap<String, String>>>(&content).unwrap_or_default();

          // loops through the craft styles content
          for (config_key, config_value) in parsed_craft_styles.iter() {
            // loops through values' content
            for (config_class_name, config_property_value) in config_value.iter() {
              // if config class name is the same as the string data
              if config_class_name == &formatted_data {
                // collects the dynamic property
                if let Some(collected_property) = collects_dynamic_core_data(config_key.to_string()) {
                  let class_rules = format!( // creates the CSS utility class
                    ".{}{} {{ {}: {} }}", config_class_name,
                    if !pseudo_property.starts_with("#") { pseudo_property.clone() } else { "".to_string() }, 
                    collected_property, config_property_value.to_string()
                  );

                  // insert the utility class into the tracker
                  generated_styles_map.insert(config_class_name.to_string(), class_rules.to_string());
                  // insert the utility class into the ast
                  append_style_to_styles_ast(config_key.to_string(), class_rules.to_string(), media.clone());
                  // set the CSS file creation to true
                  *css_file_creation = true;

                  return;
                }
              }
            }
          }
        }
      }
    } else {
      if let Some(collected_property) = collects_dynamic_core_data(key.to_string()) {
        let class_rules = format!( // creates the CSS utility class
          ".{}{} {{ {}: {} }}", class_name,
          if !pseudo_property.starts_with("#") { pseudo_property.clone() } else { "".to_string() }, 
          collected_property, string_data.to_string()
        );

        // insert the utility class into the tracker
        generated_styles_map.insert(class_name.to_string(), class_rules.to_string());
        // insert the utility class into the ast
        append_style_to_styles_ast(key.to_string(), class_rules.to_string(), media.clone());
        // set the CSS file creation to true
        *css_file_creation = true;
      }
    }
  }
}

fn generates_css_rules_from_crafting_styles_data(objects_array: Vec<String>, is_modular: bool, file_path: String, mut css_file_creation: &mut bool) -> () {
  // loops over all objects
  for value in objects_array {
    // if the current property is key:value type
    if !value.contains("[") && !value.contains("]") {
      // process the value
      process_css_rules(value, is_modular, file_path.to_string(), "".to_string(), &mut css_file_creation);
    } else if value.contains("[") && value.contains("]") {
      // parse the json into a vector of strings
      let parsed_vec = serde_json::from_str::<Vec<String>>(&value.to_string()).unwrap_or_default();
      // holds the pseudo property
      let mut pseudo_property = String::new();

      // loops through the parsed content
      for (i, value) in parsed_vec.iter().enumerate() {
        if i == 0 { // if index is 0, get the pseudo property
          pseudo_property = value.to_string().replace(":", "");
          continue;
        }

        // process the value
        process_css_rules(value.to_string(), is_modular, file_path.to_string(), pseudo_property.clone(), &mut css_file_creation);
      }
    }
  }
}

fn clear_core_ast_data() -> () {
  // lock the core ast
  let mut core_ast_map = CORE_AST.lock().unwrap();

  // loops through the nodes inside the core ast
  for (_, node) in core_ast_map.iter_mut() {
    // loops through the node's data
    for (_, data) in node.into_iter() {
      data.clear();
    }
  }
}

fn collects_core_ast_data() -> String {
  // lock the core ast to access the data
  let core_ast_map = CORE_AST.lock().unwrap();
  // variable to store the collected data
  let mut collected_data = String::new();
  // variable to store the media queries values
  let mut media_queries = String::new();

  // loops through the nodes inside the core ast
  for (container, node) in core_ast_map.iter() {
    // loops through the node's data
    for (property, data) in node.into_iter() {
      // if the data vec is not empty
      if !data.is_empty() {
        // stores the media query value
        let mut media_value = String::new();

        // loops through the data content
        for item in data {
          // the container is a media query
          if container == "mediaQueryVariables" {
            // collects the media query value
            if let Some(collected_property) = collects_dynamic_core_data(property.to_string()) {
              // if the collected value starts with "#"
              if collected_property.starts_with("#") {
                // format the collected property value
                media_value = collected_property.replace("#", "").to_string();
                media_queries += &format!("\t{}\n", item).to_string();
              }
            }
          } else { // if the container is regular rules
            // format the collected property value
            let formatted_item = format!("{}\n", item).to_string();

            // if the collected data does not contain the current value
            if !collected_data.contains(&formatted_item) {
              collected_data += &formatted_item;
            }
          }
        }

        // if the container is a media query and the media query's value is not empty
        if container == "mediaQueryVariables" && !media_value.is_empty() {
          // format the media query data
          let formatted_media = format!("@media screen and  ({}) {{ \n{} }}", media_value, media_queries);

          media_value.clear();
          media_queries.clear();

          // format the collected property value
          let formatted_item = &format!("{}\n", formatted_media).to_string();

          // if the collected data does not contain the current value
          if !collected_data.contains(formatted_item) {
            collected_data += &formatted_item;
          }
        }
      }
    }
  }

  collected_data
}

#[napi]
pub fn process_content(path: String) -> Result<()> {
  // creates CSS file state
  let mut css_file_creation = false;
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

    // get the craft styles content
    if let Some(craft_styles) = config.get("craftStyles") {
      // lock the craft styles to store data
      let mut craft_styles_map = CRAFT_STYLES_JSON.lock().unwrap();

      craft_styles_map.insert("craftStyles".to_string(), craft_styles.to_string());
    }
  }

  // if modular flag is enabled
  // clears the ast content
  if is_modular {
    clear_core_ast_data();
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
      let clean_code = clear_white_spaces_and_break_lines_from_code(file_content.clone())?;

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
              generates_css_rules_from_crafting_styles_data(objects_array, is_modular, path.clone(), &mut css_file_creation);
            }
          }

          // if the modular flag is enabled
          // or the global creation state is true
          if is_modular || css_file_creation {
            // collects the generated data from the core ast
            let collected_css_rules = collects_core_ast_data();

            // if collected CSS rules is not empty
            if !collected_css_rules.is_empty() {
              // split the path into path and extension
              let file_path: Vec<&str> = path.split(".").collect();

              // if path without extension is not empty
              if !file_path[0].is_empty() {
                // generates the CSS file path
                let css_file_path = format!("{}.css", file_path[0]);

                // if formatted CSS file path is not empty
                if !css_file_path.is_empty() {
                  // Create the CSS file write the CSS content
                  if let Err(_) = fs::write(css_file_path.clone(), collected_css_rules) {
                    println!("CSS file not generated!");
                  } else {
                    println!("CSS file generated successfully!");
                  };

                  // creates new path from the CSS file path
                  let css_path = Path::new(&css_file_path);

                  // collects CSS file path name
                  if let Some(file_name) = css_path.file_name() {
                    // transform the path into a str
                    if let Some(name_str) = file_name.to_str() {
                      // if the file content does not contain the import of the CSS file
                      if !&file_content.contains(name_str) {
                        println!("{}", name_str);
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
  }

  Ok(())
}
