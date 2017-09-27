extern crate serde_hjson;

use std::path::{Path, PathBuf};
use std::ffi::OsStr;
use std::fs::File;
use std::io::{Write, Read};
use ::serde::{Deserialize, Serialize};
use ::rmps::{Deserializer, Serializer};
// use ::serde_hjson::Value;
// use self::serde_hjson::{Map,Value};
// use ::serde_hjson::{Map,Value};
use serde_json::Error;
use std::collections::HashMap;
use std::env;

use super::*;
// use structures::*;
use structures::defaults::{DEFAULT_VCS, DEFAULT_VERSION_INC, DEFAULT_LANGUAGE};
// use ::strucutres::{DEFAULT_INSTALL_PATH, OPERATING_SYSTEM, ARCHITECTURE};
use ::structures::OperatingSystem;
use ::structures::LINE_ENDING;



pub trait Storable {
    // type C = Self;
    
    fn parse_vars(&mut self) {
        
    }
    fn blank_data() -> Self;
    fn save_yaml(&self, path: PathBuf) -> bool
        where Self: ::serde::Serialize
    // <Self as ::configuration::storage::Configurable>::C: ::serde::Serialize
    {
        let mut f = File::create(path.to_str().expect("Could not convert save_yaml path to string.")).expect("Could not create file for global_install config serialization.");
        let mut ser = ::serde_yaml::to_string(self).expect("Could not serialize yaml configuration data.");
        if ::structures::LINE_ENDING == "\r\n" {
            ser = ser.replace("\n", ::structures::LINE_ENDING);
        }
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
    fn get_yaml(path: PathBuf, create: bool) -> Self 
        where for<'de> Self: ::serde::Deserialize<'de> + ::serde::Serialize + ::std::marker::Sized
    // -> Result<Self::C, Self::C> where
        // for<'de> <Self as ::configuration::storage::Configurable>::C: ::serde::Deserialize<'de>
    {
        let mut open = File::open(path.to_str().expect("Could not convert get_yaml path to a string."));
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
    
    fn save_msgpack(&self, path: PathBuf) -> bool 
        where Self: ::serde::Serialize
    {
        let mut f = File::create(path.to_str().expect("Could not convert save_msgpack path to string.")).expect("Could not create file for config serialization.");
        let mut buffer = Vec::new();
        self.serialize(&mut Serializer::new(&mut buffer)).expect("Could not serialize msgpack configuration data.");
        
        #[allow(dead_code)]
        let rst = f.write(&buffer);
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
    fn get_msgpack(path: PathBuf, create: bool) -> Self 
        where for <'de> Self: ::serde::Deserialize<'de> + ::serde::Serialize + ::std::marker::Sized
    {
        let mut open = File::open(path.to_str().expect("Could not convert path to a string."));
        match open {
            Ok(mut f) => {
                let mut buffer = Vec::new();
                f.read_to_end(&mut buffer);
                let mut de = Deserializer::new(&buffer[..]);
                let output: Self = Deserialize::deserialize(&mut de).expect("Could not deserialize msgpack configuration data.");
                output
            },
            Err(_) => {
                let output: Self = Self::blank_data();
                if create {
                    output.save_msgpack(path);
                }
                // output.store_msgpack(path);
                output
            }
        }
    }
    
    fn save_json(&self, path: PathBuf) -> bool 
        where Self: ::serde::Serialize
    {
        let mut f = File::create(path.to_str().expect("Could not convert save_msgpack path to string.")).expect("Could not create file for config serialization.");
        
        // let ser = ::serde_json::to_string_pretty(self).expect("Could not serialize json configuration data.");
        
        let ser = ::serde_json::to_vec_pretty(self).expect("Could not serialize ");
        
        #[allow(dead_code)]
        let rst = f.write(&ser);
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
    fn get_json(path: PathBuf, create: bool) -> Self 
        // where for <'de> Self: ::serde::Deserialize<'de> + serde::de::Deserialize<'de> + ::serde::de::Deserialize<'de> + ::serde::Serialize + ::std::marker::Sized
        where for <'de> Self: ::serde::Deserialize<'de> + ::serde::de::Deserialize<'de> + ::serde::Serialize + ::std::marker::Sized
    {
        let mut open = File::open(path.to_str().expect("Could not convert path to a string."));
        match open {
            Ok(mut f) => {
                let mut buff: Vec<u8> = Vec::new();
                f.read_to_end(&mut buff);
                let output: Self = ::serde_json::from_slice(&mut buff).expect("Could not deserialize json configuration data.");
                
                // let mut buffer = String::new();
                // f.read_to_string(&mut buffer);
                // let output: Self = ::serde_json::from_str(&mut buffer).expect("Could not deserialize json configuration data.");
                output
            },
            Err(_) => {
                let output: Self = Self::blank_data();
                if create {
                    output.save_json(path);
                }
                output
            }
        }
    }
    
    
    
    
    // fn save_hjson(&self, path: PathBuf) -> bool 
    //     // where Self: ::serde::Serialize + ::serde::ser::Serialize + ::std::marker::Sized
    //     where Self: ::serde::Serialize + ::serde::ser::Serialize + ::std::marker::Sized
    // {
    //     let mut f = File::create(path.to_str().expect("Could not convert save_msgpack path to string.")).expect("Could not create file for config serialization.");
        
    //     // let ser = ::serde_json::to_string_pretty(self).expect("Could not serialize json configuration data.");
        
    //     // let ser = ::serde_hjson::to_vec(self).expect("Could not serialize ");
    //     let ser = ::configuration::storable::serde_hjson::to_vec(self).expect("Could not serialize ");
        
    //     #[allow(dead_code)]
    //     let rst = f.write(&ser);
    //     if let Ok(res) = rst {
    //         if res != 0 {
    //             true
    //         } else {
    //             false
    //         }
    //     } else {
    //         false
    //     }
    // }
    // fn get_hjson(path: PathBuf, create: bool) -> Self 
    //     where for <'de> Self: ::serde::Deserialize<'de> + ::serde::de::Deserialize<'de> + ::serde::Serialize + ::std::marker::Sized
    // {
    //     let mut open = File::open(path.to_str().expect("Could not convert path to a string."));
    //     match open {
    //         Ok(mut f) => {
    //             let mut buff: Vec<u8> = Vec::new();
    //             f.read_to_end(&mut buff);
    //             let output: Self = ::configuration::storable::serde_hjson::from_slice(&mut buff).expect("Could not deserialize json configuration data.");
                
    //             // let mut buffer = String::new();
    //             // f.read_to_string(&mut buffer);
    //             // let output: Self = ::serde_json::from_str(&mut buffer).expect("Could not deserialize json configuration data.");
    //             output
    //         },
    //         Err(_) => {
    //             let output: Self = Self::blank_data();
    //             if create {
    //                 output.save_json(path);
    //             }
    //             output
    //         }
    //     }
    // }
    
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


