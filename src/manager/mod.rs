
// mod configuration;
// mod structures;
// mod project;
// mod helpers;


// use super::configuration::*;
use ::configuration::*;
use ::configuration::storage::*;
use ::project::*;
use ::structures::*;
use ::structures::var_str::*;
use ::structures::executables::*;
use ::structures::defaults::*;
use ::helpers::*;


use std::process::{Command, Output, ExitStatus};
use std::path::{Path, PathBuf};
use std::env;
use std::ffi::OsString;
use std::collections::HashMap;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;


// check for existence of global_install.cfg, global_user.cfg, and proman_local.cfg (local project cfg)
// if needed create serialized blank config files
// deserialize all config files


// when installing run as admin as only admin can write
// to files located in program files directory
// separate the writing to the global install into an install
// executable and run that as admin.  Otherwise do not write
// data into the install path after install, let user do that


// on install add the global install folder to the path
// this allows the config file to be found anywhere
// the GlobalInstall struct should specify where the
// global user directory is located


// pub fn managed_deserialize<T: ::project::Project>() -> Global<T> {
pub fn managed_deserialize() {
    let install_file = "proman_install.cfg";
    let install_config = PathBuf::from(install_file);
    let file_opt = File::open(install_file);
    match file_opt {
        Ok(file) => {
            println!("The proman_install.cfg file was located!");
        },
        Err(_) => {
            println!("The proman_install.cfg file could not be located :'(");
        },
        
    }
    
    if install_config.exists() {
        println!("The proman_install.cfg file was found!");
    } else {
        println!("The proman_install.cfg file could not be found :(");
    }
    
    
}


