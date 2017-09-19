extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate semver;
extern crate rmp_serde as rmps;
extern crate serde_yaml;

extern crate os_type;


// mod configuration;
// mod project;
// mod structures;
// mod manager;
// mod helpers;

mod configuration;
mod project;
mod structures;
mod manager;
mod helpers;

use manager::*;

use std::process::{Command, Output, ExitStatus};
use std::path::{Path, PathBuf};
use std::env;
use std::ffi::OsString;

// extern crate regex;
// use regex::Regex;


fn main() {
    println!("Hello, world!");
    println!("This is the `run` command!");
    
    ::configuration::storage::Debug::store_configs_blank();
    
    
    
    
    
    
    
    
    
}




