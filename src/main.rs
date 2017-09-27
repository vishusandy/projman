// extern crate serde;
// extern crate serde_json;
// #[macro_use] extern crate serde_derive;
// #[macro_use] extern crate lazy_static;
// extern crate regex;
// extern crate semver;
// extern crate rmp_serde as rmps;
// extern crate serde_yaml;
// extern crate winreg;
// extern crate os_type;

extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate rmp_serde as rmps;
extern crate serde_yaml;

#[macro_use] extern crate log;
extern crate env_logger;

#[macro_use] extern crate lazy_static;

extern crate winreg;
extern crate os_type;
extern crate regex;
extern crate semver;

// extern crate serde_hjson;
// extern crate serde_hjson;

mod project;
mod configuration;
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
}


