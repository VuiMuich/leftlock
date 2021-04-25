
use std::collections::HashMap;

use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use itertools::Itertools;

use std::ffi::{CStr, CString};

use libc::getenv;

const CONFIG_MSG: &'static str = "
#Do not edit/remove this lines. Change color for each screen by editing only the right hand side of following lines.
#If file is not parseable, will revert to default config.
#The following line configures the colors for: Init, Input, Failed
";

macro_rules! map (
    {$($key:expr => $value:expr), + } => {
        {
            let mut m = HashMap::new();
            $(
                m.insert($key, $value);
             )+
            m
        }
    };
);

pub fn getusername() -> String {
    let username: String;
    unsafe {
        username = CStr::from_ptr(getenv(CString::new("USER").unwrap().as_ptr())).to_string_lossy().into_owned();
    }
    username
}

fn create_color_map(init: &str, input: &str, failed: &str) -> HashMap<u32, String> {
    map! {
        0 /* Init */ => init.to_string(),
        1 /* Input */ => input.to_string(),
        2 /* Failed */ => failed.to_string()
    }
}

fn create_default_config() -> HashMap<u32, String> {
    // Create the default config
    println!("Used default config");
    create_color_map("black", "#006400", "#8B0000")
}

pub fn parse_contents(mut contents: String) -> HashMap<u32, String> {
    // Remove the message from the file contents and then separate using whitespaces
    let config = contents.split_off(CONFIG_MSG.len() - 1);
    let mut iter = config.split_whitespace();
    iter.next();
    match iter.next() {
        Some(init_col) => {
            iter.next();
            match iter.next() {
                Some(inp_col) => {
                    iter.next();
                    match iter.next() {
                        Some(fail_col) => return create_color_map(init_col, inp_col, fail_col),
                        None => {}
                    }
                }
                None => {}
            }
        }
        None => {}
    }
    create_default_config()
}

pub fn write_default_config(path: String) {
    // let config = create_default_config().iter().for_each(|(k, v)| v.to_string());
    let config: String = create_default_config()
      .values()
      .cloned()
      .join(" ")
      ;
    match File::create(&path) {
        Ok(mut f) => {
          match f.write((String::from(CONFIG_MSG).to_owned() + &config).as_bytes()) {
              Ok(_) => return,
              Err(_) => println!("Failed to write config to ?{}", path)
          }
          return
        },
        Err(_) => println!("Failed to create config at path ?{}", path)
    }
}

pub fn read_config() -> HashMap<u32, String> {
    let file_prefix = String::from("/home/");
    let file_suffix = String::from("/.config/leftlock");

    let username = getusername();

    let path = file_prefix + &username + &file_suffix;

    if ! Path::new(&path).exists() {
        write_default_config(path.clone())
    }
    match File::open(path) {
        Ok(f) => {
            println!("Reading from config");
            let mut file = f;
            let mut contents = String::new();
            match file.read_to_string(&mut contents) {
                Ok(_) => return parse_contents(contents),
                Err(_) => {}
            }
        }

        Err(_) => println!("Error: Faild to read from config!") 
    }
    create_default_config()
}
