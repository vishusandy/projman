use std::path::{Path, PathBuf};
use std::ffi::OsStr;
use std::fs::File;
use std::io::{Write, Read};
use ::serde::{Deserialize, Serialize};
use ::rmps::{Deserializer, Serializer};
use serde_json::Error;
use std::collections::HashMap;
use std::env;

use super::*;
// use structures::*;
use structures::defaults::{DEFAULT_VCS, DEFAULT_VERSION_INC, DEFAULT_LANGUAGE};
// use ::strucutres::{DEFAULT_INSTALL_PATH, OPERATING_SYSTEM, ARCHITECTURE};
use ::structures::OperatingSystem;

pub trait Storable {
    // type C = Self;
    fn blank_data() -> Self;
    fn save_yaml(&self, path: PathBuf) -> bool
        where Self: ::serde::Serialize
    // <Self as ::configuration::storage::Configurable>::C: ::serde::Serialize
    {
        let mut f = File::create(path.to_str().expect("Could not convert global_install path to string.")).expect("Could not create file for global_install config serialization.");
        let ser = ::serde_yaml::to_string(self).expect("Could not serialize yaml configuration data.");
        #[allow(dead_code)]
        let rst = f.write(ser.as_bytes());
        if let Ok(res) = rst {
            if res != 0 {
                true
            } else {
                false
            }
        } else {
            false
        }
    }
    
    // fn retrieve_config_yaml(path: PathBuf) -> Result<Self::C, Self::C> where
        // for<'de> <Self as ::configuration::storage::Configurable>::C: ::serde::Deserialize<'de>
    
    // fn get_yaml(path: PathBuf) -> Result<Self, Self> where for<'de> Self: ::serde::Deserialize<'de> + ::std::marker::Sized
    fn get_yaml(path: PathBuf, create: bool) -> Self where for<'de> Self: ::serde::Deserialize<'de> + ::serde::Serialize + ::std::marker::Sized
    // -> Result<Self::C, Self::C> where
        // for<'de> <Self as ::configuration::storage::Configurable>::C: ::serde::Deserialize<'de>
    {
        let mut open = File::open(path.to_str().expect("Could not convert global_install path to a string."));
        match open {
            Ok(mut f) => {
                let mut buffer: String = String::new();
                f.read_to_string(&mut buffer);
                let output: Self = ::serde_yaml::from_str(&buffer).expect("Could not deserialize yaml configuration data.");
                // Ok(output)
                output
            },
            Err(_) => {
                let output: Self = Self::blank_data();
                if create {
                    output.save_yaml(path);
                }
                // let output: Self::C = <Self as ::configuration::storage::Configurable>::blank();
                // Err(output)
                output
            }
        }
    }
}

impl Storable for LocalCfg {
    fn blank_data() -> Self {
        ::configuration::LocalCfg::blank(env::current_dir().unwrap_or(PathBuf::new()))
    }
}

impl Storable for Local {
    fn blank_data() -> Self {
        ::configuration::Local::blank(env::current_dir().unwrap_or(PathBuf::new()))
    }
}

impl Storable for GlobalUser {
    fn blank_data() -> Self {
        ::configuration::GlobalUser::blank()
    }
}

impl Storable for GlobalInstall {
    fn blank_data() -> Self {
        ::configuration::GlobalInstall::blank()
    }
}


