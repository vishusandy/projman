
use std::path::{Path, PathBuf};
use std::ffi::OsString;
use std::collections::HashMap;

use std::marker::PhantomData;

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
    abort_on_fail: bool,
    custom_commands: HashMap<String, String>,
    
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
    abort_on_fail: bool,
    custom_commands: HashMap<String, String>, // if a command is specified that is already a command, it overrides that command
    
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GlobalUser {
    user_bin_path: PathBuf,
    // user_path: PathBuf, // moved to the GlobalInstall struct
    
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GlobalInstall {
    user_dir: PathBuf,
    install_bin_path: PathBuf,
    install_path: PathBuf,
    os: OperatingSystem,
    
}

// pub struct Global<T: ::configuration::storage::Configurable> {

// The Global struct holds all the config files/config data
// it should implement Configurable but only to find out where
// to store/retrieve each file and call the appropriate
// serialization/deserialzation (store/retrieve) method on the
// related struct.
// The parse_vars() method should only call the parse_vars()
// method on each of the config structs inside it.

// pub struct Global<'de, T: ::project::Project<'de>> {

// pub struct Global<'de, T: ::project::Project<'de>> {
// pub struct Global<'PhantomData, T> where T: ::project::Project<'PhantomData> {
pub struct Global<T> where T: ::project::Project {

    local: Local,
    local_details: T,
    user: GlobalUser,
    install: GlobalInstall,
    
}


