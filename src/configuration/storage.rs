
extern crate serde_json;

use serde::{Deserialize, Serialize};
use rmps::{Deserializer, Serializer};
use std::path::{Path, PathBuf};
use std::ffi::OsStr;
use std::fs::File;
use std::io::{Write, Read};
use serde::{Deserialize, Serialize};
use rmps::{Deserializer, Serializer};

use super::*;
use defaults::*;



pub trait Configurable {
    fn store(&self, PathBuf) -> bool;
    fn retrieve(PathBuf) -> Self;
    fn parse_vars(&mut self);
}

impl LocalCfg {
    pub fn to_local(&self) -> Local {
        project_path: PathBuf::from(self.project_path),
        vcs: VersionControl::from_str(&self.vcs),
        inc_version: VersionControl::from_str(&self.inc_version),
        language: Language::from_str(&self.language),
        .. self
    }
    pub fn new(proj_path: PathBuf) -> LocalCfg {
        LocalCfg {
            // project_name: if proj_path.is_dir() { proj_path.file_name().to_string_lossy().into_owned() } else {},
            project_name: proj_path.file_name().to_string_lossy().into_owned(),
            project_path: proj_path.to_string_lossy().into_owned(),
            vcs: DEFAULT_VCS.to_str(),
            inc_version: DEFAULT_VERSION_INC.to_str(),
            language: DEFAULT_LANGUAGE.to_str(),
            proj_type: String::from(""),
            autoruns: Vec::new(),
            custom_commands: Vec::new(),
        }
    }
    
}

impl Local {
    pub fn to_local_cfg(&self) -> LocalCfg {
        project_path: self.project_path.to_str().unwrap_or(""),
        vcs: self.vcs.to_str().to_string(),
        inc_version: self.inc_version.to_str().to_string(),
        language: self.language.to_str().to_string(),
        .. self
    }
    pub fn new(proj_path: PathBuf) -> Local {
        Local {
            project_name: proj_path.file_name().to_string_lossy().into_owned(),
            project_path: proj_path,
            vcs: DEFAULT_VCS,
            inc_version: DEFAULT_VERSION_INC,
            language: DEFAULT_LANGUAGE,
            proj_type: String::from(""),
            autoruns: Vec::new(),
            custom_commands: Vec::new(),
        }
    }
    
}


impl Configurable for Local {
    pub fn store(&self, path: PathBuf) -> bool {
        if !path.exists() {
            return false;
        }
        let mut f = File::create(path.to_str().expect("Could not convert path to string.")).expect("Could not create file for config serialization.");
        let ser = serde_json::to_string(self).expect("Could not serialize configuration data.");
        
        #[allow(dead_code)]
        let rst = f.write(ser.as_bytes());
        
    }
    pub fn retrieve(path: PathBuf) -> Local {
        let mut open = File::open(path.to_str().expect("Could not convert path to a string."));
        match open {
            Ok(mut f) => {
                let mut buffer: String = String::new();
                f.read_to_string(&mut buffer);
                let out: Local = serde+json::from_str(&mut buffer).expect("Could not deserialize configuration data.");
            },
            Err(_) => {
                let local: Local = Local::new(PathBuf.parent());
                local.store();
                local
            }
        }
    }
    pub fn parse_vars(&mut self) {
        
    }
}

// impl Configurable for LocalCfg {
    
// }

// impl Configurable for GlobalUser {
    
// }

// impl Configurable for GlobalInstall {
    
// }




















