
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
use structures::{DEFAULT_INSTALL_PATH};
use ::structures::defaults::{INSTALL_FILENAME, USER_FILENAME, PROJECT_FILENAME, CONFIG_RECURSE_DEPTH};
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


use std::cell::RefCell;
// use std::sync::Mutex;

// use ::configuration::*;
// use ::configuration::storage::*;
// use ::structures::*;
use ::configuration::storable::*;


// pub struct ManagedConfig {
//     pub path: PathBuf,
//     pub 
// }

// pub struct ManagedResult<T> where T: ::configuration::storage::Configurable {
//     Ok( (PathBuf, ) ),
//     None,
// }



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

// lazy_static!{
//     static ref LOCAL_CONFIG: Mutex<::configuration::Local> = Mutex::new(::configuration::Local::blank(PathBuf::new()));
//     static ref INSTALL_CONFIG: Mutex<::configuration::GlobalInstall> = Mutex::new(::configuration::GlobalInstall::blank());
//     static ref USER_CONFIG: Mutex<::configuration::GlobalUser> = Mutex::new(::configuration::GlobalUser::blank());
// }
    
// impl ::configuration::Global {
//     pub fn startup(&mut self) -> ::configuration::Global {
        
//     }
    
// }


pub fn find_local() -> Option<::configuration::Local> {
    let cur = env::current_dir();
    if let Ok(cd) = cur {
        let mut local_file = cd.clone();
        // local_file.set_file_name(::structures::defaults::PROJECT_FILENAME);
        local_file.push(::structures::defaults::PROJECT_FILENAME);
        // eprintln!("Looking for {}", local_file.display());
        if local_file.exists() && local_file.is_file() {
            // eprintln!("Found local config at {}", local_file.display());
            // retrieve local config
            // let local_cfg = ::configuration::LocalCfg::retrieve_yaml(local_file);
            let local_cfg = ::configuration::LocalCfg::get_yaml(local_file, false);
            let local = local_cfg.to_local();
            Some(local)
        } else {
            
            // look up to CONFIG_RECURSE_DEPTH parent directories 
            // if let Some(par) = parent_opt {
            if let  Some(par) = cd.parent() {
                let mut cur_dir = cd.clone();
                let mut parent: PathBuf;
                let mut parent_opt = cur_dir.parent();
                
                let mut parent = par.to_path_buf();
                // eprintln!("{} not found in current directory {}.  Looking in parent directories for local config, starting with {}", ::structures::defaults::PROJECT_FILENAME, env::current_dir().unwrap().display(), parent.display());
                
                'parent_loop: for i in 0..::structures::defaults::CONFIG_RECURSE_DEPTH {
                    let mut cur_parent = parent.clone();
                    // cur_parent.set_file_name(::structures::defaults::PROJECT_FILENAME);
                    cur_parent.push(::structures::defaults::PROJECT_FILENAME);
                    if cur_parent.exists() && cur_parent.is_file() {
                        // let local_cfg = ::configuration::LocalCfg::retrieve_yaml(local_file);
                        // eprintln!("Found local config at {}", cur_parent.display());
                        // let local_cfg = ::configuration::LocalCfg::get_yaml(local_file, true);
                        let local_cfg = ::configuration::LocalCfg::get_yaml(cur_parent, false);
                        let local = local_cfg.to_local();
                        return Some(local);
                    }
                    
                    // if i+1 != ::structures::defaults::CONFIG_RECURSE_DEPTH {
                    let parent_clone = parent.clone();
                    let popt = parent_clone.parent();
                    if let Some(par_opt) = popt {
                        parent = par_opt.to_path_buf();
                        // eprintln!("Continuing to look in {}", parent.display());
                        
                    } else {
                        // eprintln!("No more parent directories found");
                        break 'parent_loop;
                    }
                    // }
                
                }
                None
            } else {
                None
            }
        }
    } else {
        None
    }
    
    
}

pub fn find_install_without_local() -> ::configuration::GlobalInstall {
    let cur_exe = env::current_exe();
    let inst = PathBuf::from(::structures::DEFAULT_INSTALL_PATH);
    let os = ::structures::OperatingSystem::new();
    // if os.get_install_path().exists() {
    let mut install_file = os.get_install_path().clone();
    // install_file.set_file_name(INSTALL_FILENAME);
    install_file.push(INSTALL_FILENAME);
    if install_file.exists() {
        // ::configuration::GlobalInstall::retrieve_yaml(install_file)
        ::configuration::GlobalInstall::get_yaml(install_file, true)
    } else if let Ok(exe_dir) = cur_exe {
        let mut cur_dir = exe_dir.clone();
        let mut cur_file = cur_dir.clone();
        // cur_file.set_file_name(::structures::defaults::INSTALL_FILENAME);
        cur_file.push(::structures::defaults::INSTALL_FILENAME);
        if cur_file.exists() {
            // ::configuration::GlobalInstall::retrieve_yaml(cur_file)
            ::configuration::GlobalInstall::get_yaml(cur_file, true)
        } else {
            let mut parent_opt = cur_dir.parent();
            let mut parent: PathBuf;
            if let Some(par) = parent_opt {
                parent = par.to_path_buf();
                for i in 0..::structures::defaults::CONFIG_RECURSE_DEPTH {
                    let mut cur_parent = parent.clone();
                    // cur_parent.set_file_name(::structures::defaults::INSTALL_FILENAME);
                    cur_parent.push(::structures::defaults::INSTALL_FILENAME);
                    if cur_parent.exists() {
                        // return ::configuration::GlobalInstall::retrieve_yaml(cur_parent);
                        return ::configuration::GlobalInstall::get_yaml(cur_parent, true);
                    }
                    
                    let parent_clone = parent.clone();
                    let popt = parent_clone.parent();
                    // parent_opt = parent.parent();
                    if let Some(par_opt) = popt {
                        parent = par_opt.to_path_buf();
                    } else {
                        break;
                    }
                }
                panic!("Could not find the global installation config file anywhere in the current executable directory or its parent directories.");
            } else {
                // TODO: Improve error message to be more helpful and allow user to do something about it
                panic!("Could not find global installation config file.");
            }
        }
    } else {
        // TODO: Improve error message to be more helpful and allow user to do something about it
        panic!("Could not find a suitable global installation configuration file.");
    }
}

pub fn find_install(local_config: Option<::configuration::Local>) -> ::configuration::GlobalInstall {
    // if local_config.is_some() {
        // let local = local_config.unwrap();
    if let Some(local) = local_config {
        if let Some(global_path) = local.global_install {
            // let mut install_path: PathBuf = local.global_install.unwrap().clone();
            let mut install_path: PathBuf = global_path.clone();
            // install_path.set_file_name(::structures::defaults::INSTALL_FILENAME);
            install_path.push(::structures::defaults::INSTALL_FILENAME);
            if install_path.exists() {
                // ::configuration::GlobalInstall::retrieve_yaml(install_path)
                ::configuration::GlobalInstall::get_yaml(install_path, true)
            } else {
                find_install_without_local()
            }
        } else {
            find_install_without_local()
        }
    } else {
        find_install_without_local()
    }
}


pub fn find_user(install: &GlobalInstall, local_opt: Option<::configuration::Local>) -> ::configuration::GlobalUser {
    // look in global's user_dir then home_dir
    let mut user_dir = install.user_dir.clone();
    let mut global_user_file = user_dir.clone();
    // global_user_file.set_file_name(::structures::defaults::USER_FILENAME);
    global_user_file.push(::structures::defaults::USER_FILENAME);
    if global_user_file.exists() {
        // ::configuration::GlobalUser::retrieve_yaml(global_user_file)
        ::configuration::GlobalUser::get_yaml(global_user_file, true)
    } else {
        let home_opt = env::home_dir();
        if let Some(home_dir) = home_opt {
            let mut home = home_dir.clone();
            home.push("proman");
            // home.set_file_name(::structures::defaults::USER_FILENAME);
            home.push(::structures::defaults::USER_FILENAME);
            if home.exists() {
                // ::configuration::GlobalUser::retrieve_yaml(home)
                ::configuration::GlobalUser::get_yaml(home, true)
            } else {
                panic!("Could not find user configuration file, no such file exists in home directory.");
            }
        } else {
            panic!("Could not find user configuration file, home directory could not be found.");
        }
    }
}

/*  check_custom_install(user, global)
    
*/

pub fn check_custom_install(user: &GlobalUser, install: &GlobalInstall) -> Option<GlobalInstall> {
    if let Some(ref user_default) = user.user_default_install {
        let mut user_install: PathBuf = user_default.clone();
        // user_install.set_file_name(INSTALL_FILENAME);
        user_install.push(INSTALL_FILENAME);
        if user_install.exists() && user_install != install.install_path {
            // return Some(::configuration::GlobalInstall::retrieve_yaml(user_install));
            return Some(::configuration::GlobalInstall::get_yaml(user_install, true));
        }
    }
    None
}

pub fn find_configs() -> (Option<::configuration::Local>, ::configuration::GlobalUser, ::configuration::GlobalInstall) {
    // look for local project config file
        // if not exist look check if command is global command, if not exit 
        // if it does exist it doesn't matter the scope of the command
    // then look for global
    // then look for user
    
    
    let local = find_local();
    let local_opt = local.clone();
    let local_opt2 = local_opt.clone();
    
    
    let mut install = find_install(local_opt);
    let user = find_user(&install, local_opt2);
    
    let install_opt = check_custom_install(&user, &install);
    if let Some(new_install) = install_opt {
        install = new_install;
    }
    
    (local, user, install)
}

// pub fn find_proj_details<T>(local_opt: Option<Local>) -> Option<T> where T: ::project::Project {
//     if let Some(local) = local_opt {
        
//     } else {
//         None
//     }
// }

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
    
    // ::configuration::storage::Configurable::store_config_yaml(user as &::configuration::storage::Configurable<::configuration::GlobalUser>, path)
    // store::<Type>(assocVal, path)
    
    // ::configuration::storage::Configurable::store_config_yaml::<::configuration::GlobalUser>(user, path)
    
    // ::configuration::storage::Configurable::store_config_yaml<>(user, path)
    true
    
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

