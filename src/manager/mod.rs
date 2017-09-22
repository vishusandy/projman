
// mod configuration;
// mod structures;
// mod project;
// mod helpers;


use ::configuration::*;
use ::configuration::storage::*;
use ::project::*;
use ::structures::*;
use ::structures::var_str::*;
use ::structures::executables::*;
use ::helpers::*;
use ::structures::defaults::{INSTALL_FILENAME, USER_FILENAME, PROJECT_FILENAME};
// use ::structures::defaults::*;

use std::process::{Command, Output, ExitStatus};
use std::path::{Path, PathBuf};
use std::env;
use std::fs::File;
use std::io::{Write, Read};
use std::ffi::OsString;
use std::collections::HashMap;
use std::io::prelude::*;
use std::error::Error;



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
    // let install_file = "proman_install.cfg";
    // let install_config = PathBuf::from(install_file);
    // let file_opt = File::open(install_file);
    // match file_opt {
    //     Ok(file) => {
    //         println!("The proman_install.cfg file was located!");
    //     },
    //     Err(_) => {
    //         println!("The proman_install.cfg file could not be located :'(");
    //     },
        
    // }
    
    // if install_config.exists() {
    //     println!("The proman_install.cfg file was found!");
    // } else {
    //     println!("The proman_install.cfg file could not be found :(");
    // }
}


// pub fn create_user_config<T: ::configuration::storage::Configurable>(user: T, path: PathBuf) -> bool {
// pub fn create_user_config<T: ::configuration::storage::Configurable>(user: T, path: PathBuf) -> bool {
    
    
// pub fn create_user_cfg<C: ::configuration::GlobalUser>(user: &C, path: PathBuf) -> bool {
pub fn create_user_cfg<C: ::configuration::storage::Configurable>(user: &C, path: PathBuf) -> bool {
    // let user = ::configuration::GlobalUser::blank();
    // ::configuration::storage::Configurable::store_config_yaml<::configuration::storage::Configurable::C= ::configuration::GlobalUser>(user, path)
    // ::configuration::storage::Configurable::store_config_yaml<::configuration::storage::Configurable::C = ::configuration::GlobalUser>(user, path)
    
    // ::configuration::storage::Configurable::store_config_yaml(user as ::configuration::storage::Configurable<::configuration::GlobalUser>, path)
    ::configuration::storage::Configurable::store_config_yaml(user as &::configuration::storage::Configurable<::configuration::GlobalUser>, path)
    
    
    
}

// pub fn create_user_config<C: ::configuration::storage::Configurable>(user: &C, path: PathBuf) -> bool {
    
    // ::configuration::storage::Configurable::store_config_yaml(&user, path)
    
    
// }


// pub fn create_install_config<T: ::configuration::storage::Configurable>(install: T, path: PathBuf) -> bool {

// pub fn create_install_config<T: ::configuration::storage::Configurable>(install: T, path: PathBuf) -> bool {
//     let install = ::configuration::GlobalInstall::blank();
//     ::configuration::storage::Configurable::store_config_yaml(&install, path)
// }


pub fn find_user_cfg() -> Option<PathBuf> {
    let home_dir = env::home_dir();
    // match home_dir {
    //     Some(home) => { home },
    //     None => { PathBuf::new() },
    // }
    if let Some(home) = home_dir {
        let mut proman_dir: PathBuf = home.clone();
        proman_dir.push("proman");
        // if let (proman_path) = proman_dir {
        if proman_dir.exists() {
            let mut user_cfg: PathBuf = proman_dir.clone();
            user_cfg.set_file_name(USER_FILENAME);
            // let user_cfg = proman_dir.set_file_name(USER_FILENAME);
            if user_cfg.exists() {
                Some(proman_dir)
            } else {
                // create blank GlobalUser and store it
                let user = GlobalUser::blank();
                // if ::manager::create_user_config(&user, user_cfg) {
                if ::manager::create_user_cfg(&user, user_cfg) {
                    Some(proman_dir)
                } else {
                    None
                }
            }
        } else {
            // create install dir 
            None
        }
        
    } else {
        // check the directory of the executable and its parent for an install config file
        // oterhwise
        None
    }
}

/*pub fn find_global{
    // look in user directory 
    let home: PathBuf;
    let mut home_cfg: PathBuf;
    match env::home_dir() {
        Some(home_dir) => {
            // first look for file in user dir, then parent, up to 3 levels up
            let install_file = PathBuf::from(INSTALL_FILENAME);
            if install_file.exists() {
                
            } else {
                let mut install_dir = PathBuf::new();
                for i in 0..
            }
            
            home = home_dir;
            home_cfg = home_dir;
            home_cfg.set_file_name("proman_dir.cfg");
            match File::open(&home) {
                Ok() => {
                    
                },
                Err(err) => {
                    
                },
            }
        },
        None => {
            match env::current_dir() {
                Ok(cd) => {
                    eprintln!("Could not find home directory, creating config file in `{}`", cd.display());
                    
                    
                },
                _ => { eprintln!("Could not find home directory or current directory."); },
            }
        },
    }
    
    
}
*/

