
use std::path::{Path, PathBuf};
use std::ffi::OsString;
use std::collections::HashMap;

use std::marker::PhantomData;

pub mod storage;
pub mod storable;

use structures::*;
use ::configuration::storable::*;

// saved as a hjson file
#[derive(Serialize, Deserialize, Debug)]
pub struct LocalCfg {
    pub project_name: String,
    pub project_path: String,
    pub global_install: String,
    pub vcs: String,
    pub inc_version: String,
    pub language: String,
    pub proj_type: String,
    pub autoruns: Vec<String>,
    pub abort_on_fail: bool,
    pub custom_commands: HashMap<String, String>,
    
}

// saved as a msgpack file
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Local {
    pub project_name: String,
    pub project_path: PathBuf,
    pub global_install: Option<PathBuf>,
    pub vcs: VersionControl,
    pub inc_version: VersionInc, // TODO: remove from here, move to language details
    pub language: Language,
    pub proj_type: String,
    pub autoruns: Vec<String>,
    pub abort_on_fail: bool,
    pub custom_commands: HashMap<String, String>, // if a command is specified that is already a command, it overrides that command
    
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GlobalUser {
    pub user_bin_path: PathBuf,
    pub user_default_install: Option<PathBuf>,
    // user_path: PathBuf, // moved to the GlobalInstall struct
    
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GlobalInstall {
    pub user_dir: PathBuf,
    pub install_bin_path: PathBuf,
    pub install_path: PathBuf,
    pub os: OperatingSystem,
    
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

    pub local: Option<Local>,
    pub local_details: Option<T>,
    pub user: GlobalUser,
    pub install: GlobalInstall,
    
}


