
use std::path::{Path, PathBuf};
use std::ffi::OsString;
use std::collections::HashMap;

pub mod storage;
use structures::*;

// saved as a hjson file
#[derive(Serialize, Deserialize, Debug)]
pub struct LocalCfg {
    project_name: String,
    project_path: String,
    vcs: String,
    inc_version: String,
    language: String,
    proj_type: String,
    autoruns: Vec<String>,
    custom_commands: Vec<String>,
    
}

// saved as a msgpack file
#[derive(Serialize, Deserialize, Debug)]
pub struct Local {
    project_name: String,
    project_path: PathBuf,
    vcs: VersionControl,
    inc_version: VersionInc,
    language: Language,
    proj_type: String,
    autoruns: Vec<String>,
    custom_commands: Vec<String>, // if a command is specified that is already a command, it overrides that command
    
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GlobalUser {
    user_bin_path: PathBuf,
    user_path: PathBuf,
    
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GlobalInstall {
    install_bin_path: PathBuf,
    install_path: PathBuf,
    os: OperatingSystem,
    
}

// pub struct Global<T: ::configuration::storage::Configurable> {
pub struct Global<T: ::project::Project> {
    local: T,
    user: GlobalUser,
    install: GlobalInstall,
    
}


