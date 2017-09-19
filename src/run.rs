extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate rmp_serde as rmps;
extern crate serde_yaml;

#[macro_use] extern crate log;
extern crate env_logger;

#[macro_use] extern crate lazy_static;

extern crate os_type;
extern crate regex;
extern crate semver;


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
    
    // error, warn, info, debug, trace
    env_logger::init();
    
    println!("Hello, world!");
    println!("This is the `run` command!");
    
    ::configuration::storage::Debug::store_configs_blank();
    
    
    
    
    
    
    
    
    
}




