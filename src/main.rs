mod configuration;
mod project;
mod structures;
mod manager;
mod helpers;

#[macro_use] extern crate lazy_static;
extern crate regex;

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


