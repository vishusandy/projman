
// extern crate serde_json;
use std::path::{Path, PathBuf};
use std::ffi::OsStr;
use std::fs::File;
use std::io::{Write, Read};
use ::serde::{Deserialize, Serialize};
use ::rmps::{Deserializer, Serializer};
use serde_json::Error;
use std::env;

use super::*;
// use structures::*;
use structures::defaults::{DEFAULT_VCS, DEFAULT_VERSION_INC, DEFAULT_LANGUAGE};
// use ::strucutres::{DEFAULT_INSTALL_PATH, OPERATING_SYSTEM, ARCHITECTURE};
use ::structures::OperatingSystem;

pub trait Configurable {
    fn store_msgpack(&self, PathBuf) -> bool;
    fn retrieve_msgpack(PathBuf) -> Self;
    fn store_yaml(&self, PathBuf) -> bool;
    fn retrieve_yaml(PathBuf) -> Self;
    fn store_json(&self, PathBuf) -> bool;
    fn retrieve_json(PathBuf) -> Self;
    fn parse_vars(&mut self);
}

impl LocalCfg {
    pub fn to_local(&self) -> Local {
        Local {
            project_path: PathBuf::from(self.project_path.clone()),
            vcs: VersionControl::from_str(&self.vcs),
            inc_version: VersionInc::from_str(&self.inc_version),
            language: Language::from_str(&self.language),
            project_name: self.project_name.clone(),
            proj_type: self.proj_type.clone(),
            autoruns: self.autoruns.clone(),
            custom_commands: self.custom_commands.clone(),
        }
    }
    pub fn blank(proj_path: PathBuf) -> LocalCfg {
        LocalCfg {
            // project_name: if proj_path.is_dir() { proj_path.file_name().to_string_lossy().into_owned() } else {},
            project_name: proj_path.file_name().unwrap().to_string_lossy().into_owned(),
            project_path: proj_path.to_string_lossy().into_owned(),
            vcs: DEFAULT_VCS.to_str().to_string(),
            inc_version: DEFAULT_VERSION_INC.to_str().to_string(),
            language: DEFAULT_LANGUAGE.to_str().to_string(),
            proj_type: String::from(""),
            autoruns: Vec::new(),
            custom_commands: Vec::new(),
        }
    }
    
}

impl Local {
    pub fn to_local_cfg(&self) -> LocalCfg {
        LocalCfg {
            project_path: self.project_path.to_str().unwrap_or("").to_string(),
            vcs: self.vcs.to_str().to_string(),
            inc_version: self.inc_version.to_str().to_string(),
            language: self.language.to_str().to_string(),
            // .. *self
            project_name: self.project_name.clone(),
            proj_type: self.proj_type.clone(),
            autoruns: self.autoruns.clone(),
            custom_commands: self.custom_commands.clone(),
        }
    }
    pub fn blank(proj_path: PathBuf) -> Local {
        Local {
            project_name: proj_path.file_name().unwrap().to_string_lossy().into_owned(),
            project_path: if proj_path.is_dir() {
                proj_path
            } else if proj_path.is_file() {
                proj_path.parent().unwrap_or(&proj_path).to_path_buf()
            } else {
                // proj_path.parent().unwrap_or(&proj_path).to_path_buf()
                proj_path
            },
            vcs: DEFAULT_VCS,
            inc_version: DEFAULT_VERSION_INC,
            language: DEFAULT_LANGUAGE,
            proj_type: String::from(""),
            autoruns: Vec::new(),
            custom_commands: Vec::new(),
        }
    }
    pub fn new(proj_path: PathBuf, proj_name: String, proj_lang: ::structures::Language, proj_type: String) -> Local {
        Local {
            project_name: proj_name,
            project_path: proj_path,
            vcs: DEFAULT_VCS,
            inc_version: DEFAULT_VERSION_INC,
            proj_type: {
                let l = proj_lang.to_str();
                let mut t = String::with_capacity(proj_type.len() + l.len() + 3);
                t.push_str(l);
                t.push_str(".");
                t.push_str(&proj_type);
                t
            },
            language: proj_lang, // this must go after proj_type in order to prevent ownership error stuff etc
            autoruns: Vec::new(),
            custom_commands: Vec::new(),
        }
    }
    
}

impl GlobalUser {
    pub fn blank() -> GlobalUser {
        GlobalUser {
            user_bin_path: {
                let mut dir = env::home_dir().expect("Could not find user directory.");
                dir.push("proman");
                dir.push("bin");
                dir
            },
        }
    }
    pub fn new(bin_dir: PathBuf) -> GlobalUser {
        GlobalUser {
            user_bin_path: if bin_dir.is_dir() {
                bin_dir
            } else if bin_dir.is_file() {
                bin_dir.parent().unwrap_or(&bin_dir).to_path_buf()
            } else {
                bin_dir
            },
        }
    }
}

impl Configurable for GlobalUser {
    fn store_msgpack(&self, path: PathBuf) -> bool {
        let mut f = File::create(path.to_str().expect("Could not convert path to string.")).expect("Could not create file for config serialization.");
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
    fn retrieve_msgpack(path: PathBuf) -> GlobalUser {
        let mut open = File::open(path.to_str().expect("Could not convert path to a string."));
        match open {
            Ok(mut f) => {
                let mut buffer = Vec::new();
                f.read_to_end(&mut buffer);
                let mut de = Deserializer::new(&buffer[..]);
                let user: GlobalUser = Deserialize::deserialize(&mut de).expect("Could not deserialize msgpack configuration data.");
                user
            },
            Err(_) => {
                let user: GlobalUser = GlobalUser::blank();
                user.store_msgpack(path);
                user
            }
        }
    }
    fn store_yaml(&self, path: PathBuf) -> bool {
        // if !path.exists() {
        //     println!("Path `{}` does not exist!", path.display());
        //     return false;
        // }
        let mut f = File::create(path.to_str().expect("Could not convert global_user path to string.")).expect("Could not create file for global_user config serialization.");
        // let ser = ::serde_json::to_string(self).expect("Could not serialize global_user configuration data.");
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
    fn retrieve_yaml(path: PathBuf) -> GlobalUser {
        let mut open = File::open(path.to_str().expect("Could not convert global_user path to a string."));
        match open {
            Ok(mut f) => {
                let mut buffer: String = String::new();
                f.read_to_string(&mut buffer);
                // let user: GlobalUser = ::serde_json::from_str(&mut buffer).expect("Could not deserialize global_user configuration data.");
                let user: GlobalUser = ::serde_yaml::from_str(&buffer).expect("Could not deserialize yaml configuration data.");
                user
            },
            Err(_) => {
                let user: GlobalUser = GlobalUser::blank();
                user.store_yaml(path);
                user
            }
        }
    }
    fn store_json(&self, path: PathBuf) -> bool {
        // if !path.exists() {
        //     println!("Path `{}` does not exist!", path.display());
        //     return false;
        // }
        let mut f = File::create(path.to_str().expect("Could not convert global_user path to string.")).expect("Could not create file for global_user config serialization.");
        let ser = ::serde_json::to_string(self).expect("Could not serialize global_user configuration data.");
        
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
    fn retrieve_json(path: PathBuf) -> GlobalUser {
        let mut open = File::open(path.to_str().expect("Could not convert global_user path to a string."));
        match open {
            Ok(mut f) => {
                let mut buffer: String = String::new();
                f.read_to_string(&mut buffer);
                let user: GlobalUser = ::serde_json::from_str(&mut buffer).expect("Could not deserialize global_user configuration data.");
                user
            },
            Err(_) => {
                let user: GlobalUser = GlobalUser::blank();
                user.store_json(path);
                user
            }
        }
    }
    fn parse_vars(&mut self) {
        
    }
}

impl GlobalInstall {
    pub fn blank() -> GlobalInstall {
        let os = OperatingSystem::new();
        GlobalInstall {
            user_dir: {
                // TODO: if linux make more specific default paths using the os_type crate
                let mut dir = env::home_dir().expect("Could not find user directory.");
                dir.push("proman");
                dir
            },
            // install_path: PathBuf::from(::structures::DEFAULT_INSTALL_PATH),
            install_path: PathBuf::from(os.get_install_path()),
            install_bin_path: {
                let mut dir = PathBuf::from(os.get_install_path());
                dir.push("bin");
                dir
            },
            os,
        }
    }
    pub fn new(user_dir: PathBuf, install_path: PathBuf) -> GlobalInstall {
        GlobalInstall {
            user_dir: user_dir.clone(),
            install_path: install_path.clone(),
            install_bin_path: {
                let mut dir = install_path.clone();
                dir.push("bin");
                dir
            },
            os: OperatingSystem::new(),
        }
    }
}

impl Configurable for GlobalInstall {
    fn store_msgpack(&self, path: PathBuf) -> bool {
        let mut f = File::create(path.to_str().expect("Could not convert path to string.")).expect("Could not create file for config serialization.");
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
    fn retrieve_msgpack(path: PathBuf) -> GlobalInstall {
        let mut open = File::open(path.to_str().expect("Could not convert path to a string."));
        match open {
            Ok(mut f) => {
                let mut buffer = Vec::new();
                f.read_to_end(&mut buffer);
                let mut de = Deserializer::new(&buffer[..]);
                let install: GlobalInstall = Deserialize::deserialize(&mut de).expect("Could not deserialize msgpack configuration data.");
                install
            },
            Err(_) => {
                let install: GlobalInstall = GlobalInstall::blank();
                install.store_msgpack(path);
                install
            }
        }
    }
    fn store_yaml(&self, path: PathBuf) -> bool {
        // if !path.exists() {
        //     println!("Path `{}` does not exist!", path.display());
        //     return false;
        // }
        let mut f = File::create(path.to_str().expect("Could not convert global_install path to string.")).expect("Could not create file for global_install config serialization.");
        // let ser = ::serde_json::to_string(self).expect("Could not serialize global_install configuration data.");
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
    fn retrieve_yaml(path: PathBuf) -> GlobalInstall {
        let mut open = File::open(path.to_str().expect("Could not convert global_install path to a string."));
        match open {
            Ok(mut f) => {
                let mut buffer: String = String::new();
                f.read_to_string(&mut buffer);
                // let install: GlobalInstall = ::serde_json::from_str(&mut buffer).expect("Could not deserialize global_install configuration data.");
                let install: GlobalInstall = ::serde_yaml::from_str(&buffer).expect("Could not deserialize yaml configuration data.");
                install
            },
            Err(_) => {
                let install: GlobalInstall = GlobalInstall::blank();
                install.store_yaml(path);
                install
            }
        }
    }
    fn store_json(&self, path: PathBuf) -> bool {
        // if !path.exists() {
        //     println!("Path `{}` does not exist!", path.display());
        //     return false;
        // }
        let mut f = File::create(path.to_str().expect("Could not convert global_install path to string.")).expect("Could not create file for global_install config serialization.");
        let ser = ::serde_json::to_string(self).expect("Could not serialize global_install configuration data.");
        
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
    fn retrieve_json(path: PathBuf) -> GlobalInstall {
        let mut open = File::open(path.to_str().expect("Could not convert global_install path to a string."));
        match open {
            Ok(mut f) => {
                let mut buffer: String = String::new();
                f.read_to_string(&mut buffer);
                let install: GlobalInstall = ::serde_json::from_str(&mut buffer).expect("Could not deserialize global_install configuration data.");
                install
            },
            Err(_) => {
                let install: GlobalInstall = GlobalInstall::blank();
                install.store_json(path);
                install
            }
        }
    }
    fn parse_vars(&mut self) {
        
    }
}

impl Configurable for Local {
    fn store_msgpack(&self, path: PathBuf) -> bool {
        let mut f = File::create(path.to_str().expect("Could not convert path to string.")).expect("Could not create file for config serialization.");
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
    fn retrieve_msgpack(path: PathBuf) -> Local {
        let mut open = File::open(path.to_str().expect("Could not convert path to a string."));
        match open {
            Ok(mut f) => {
                let mut buffer = Vec::new();
                f.read_to_end(&mut buffer);
                let mut de = Deserializer::new(&buffer[..]);
                let local: Local = Deserialize::deserialize(&mut de).expect("Could not deserialize msgpack configuration data.");
                local
            },
            Err(_) => {
                let local: Local = Local::blank(path.parent().unwrap().to_path_buf());
                local.store_msgpack(path);
                local
            }
        }
    }
    fn store_yaml(&self, path: PathBuf) -> bool {
        let mut f = File::create(path.to_str().expect("Could not convert path to string.")).expect("Could not create file for config serialization.");
        
        // let ser = ::serde_json::to_string(self).expect("Could not serialize configuration data.");
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
    fn retrieve_yaml(path: PathBuf) -> Local {
        let mut open = File::open(path.to_str().expect("Could not convert path to a string."));
        match open {
            Ok(mut f) => {
                let mut buffer: String = String::new();
                f.read_to_string(&mut buffer);
                
                // let local: Local = ::serde_json::from_str(&mut buffer).expect("Could not deserialize configuration data.");
                let local: Local = ::serde_yaml::from_str(&buffer).expect("Could not deserialize yaml configuration data.");
                
                local
            },
            Err(_) => {
                // let local: Local = Local::new(path.parent().unwrap().to_path_buf());
                let local: Local = Local::blank(path.parent().unwrap().to_path_buf());
                local.store_yaml(path);
                local
            }
        }
    }
    fn store_json(&self, path: PathBuf) -> bool {
        // if !path.exists() {
        //     println!("Path `{}` does not exist!", path.display());
        //     return false;
        // }
        let mut f = File::create(path.to_str().expect("Could not convert path to string.")).expect("Could not create file for config serialization.");
        let ser = ::serde_json::to_string(self).expect("Could not serialize configuration data.");
        
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
    fn retrieve_json(path: PathBuf) -> Local {
        let mut open = File::open(path.to_str().expect("Could not convert path to a string."));
        match open {
            Ok(mut f) => {
                let mut buffer: String = String::new();
                f.read_to_string(&mut buffer);
                let local: Local = ::serde_json::from_str(&mut buffer).expect("Could not deserialize configuration data.");
                local
            },
            Err(_) => {
                // let local: Local = Local::new(path.parent().unwrap().to_path_buf());
                let local: Local = Local::blank(path.parent().unwrap().to_path_buf());
                local.store_json(path);
                local
            }
        }
    }
    fn parse_vars(&mut self) {
        // TODO: implement this
    }
    
}


// TODO: Implement configurable for LocalCfg
//          store_json(&self, PathBuf) -> bool  
//          retrieve_json(PathBuf) -> Local
//          parse_vars(&mut self)
//
// impl Configurable for LocalCfg {
//  
// }



// pub fn find_proj_cfg(cd: PathBuf) -> Result<PathBuf, String> {
//  
//  
// }


// // pub fn configure<T: ::project::Project>(cd: PathBuf) -> Global<T> {
// pub fn configure() -> Global {
//     let cfg_dir_opt = find_proj_cfg(<cur_dir>);
//     if let Ok(dir) = cfg_dir_opt {
//      
//      
//     } else {
//         panic!("Could not find configuration information.");
//     }
// }


// pub fn store_configs_blank(path: PathBuf) {
    // let mut dir: PathBuf;
    // let mut file: PathBuf;
    // if path.is_dir() {
    //     dir = path;
    //     file = dir.clone();
    //     file.set_file_name("");
    // } else if path.is_file() {
    //     file = path.clone();
    //     dir = path.parent().unwrap_or(&path).to_path_buf();
    // } else {
    //     panic!("Path is neither Directory nor File.");
    // }
    // let mut local: Local = Local::blank(dir);
    // let mut user: GlobalUser = GlobalUser::blank();
    // let mut install: GlobalInstall = GlobalInstall::blank();
    
pub mod Debug {
    use std::path::{Path, PathBuf};
    use std::ffi::OsStr;
    use std::fs::File;
    use std::io::{Write, Read};
    use ::serde::{Deserialize, Serialize};
    use ::rmps::{Deserializer, Serializer};
    use serde_json::Error;
    use std::env;

    use super::*;
    // use structures::*;
    use structures::defaults::{DEFAULT_VCS, DEFAULT_VERSION_INC, DEFAULT_LANGUAGE};
    // use ::strucutres::{DEFAULT_INSTALL_PATH, OPERATING_SYSTEM, ARCHITECTURE};
    use ::structures::OperatingSystem;

    pub fn store_configs_blank() {
        let install_path = PathBuf::from(r#"c:\program files\proman\install.cfg"#);
        let user_path = PathBuf::from(r#"c:\users\Andrew\proman\user.cfg"#);
        let local_path = PathBuf::from(r#"c:\code\proj\protest\proman.cfg"#);
        
        // let install_new = GlobalInstall::blank();
        // let user_new = GlobalUser::blank();
        // let local_new = Local::blank(local_path);
        let install = GlobalInstall::blank();
        let user = GlobalUser::blank();
        let local = Local::blank(local_path.clone());
        
        println!("-----BLANK'd Data-----");
        println!("Global Install Config: {:?}", install);
        println!("Global User Config: {:?}", user);
        println!("Local Pojrect Config: {:?}", local);
        
        println!("-----STORE'd Results-----");
        if !install_path.exists() {
            println!("Install Config Store: {:?}" , install.store_msgpack(install_path.clone()));
        } else {
            println!("Install Config Exists, skipping as it requires admin privileges. ");
        }
        println!("User Config Store: {:?}" , user.store_msgpack(user_path.clone()));
        println!("Local Config Store: {:?}" , local.store_msgpack(local_path.clone()));
        
        
        println!("-----RETRIEVE'd Results-----");
        let install_get = GlobalInstall::retrieve_msgpack(PathBuf::from(install_path.clone()));
        let user_get = GlobalUser::retrieve_msgpack(PathBuf::from(user_path.clone()));
        let local_get = Local::retrieve_msgpack(PathBuf::from(local_path.clone()));
        
        println!("-----RETRIEVE'd Data-----");
        println!("Global InstallGet Config: {:?}", install_get);
        println!("Global UserGet Config: {:?}", user_get);
        println!("Local PojrectGet Config: {:?}", local_get);
        
        
        // let mut global: Global {
        //     local,
        //     local_details: {
        //         let proj_lang = local.language;
        //         let proj_type = local.proj_type;
        //     }
        //     user,
        //     install,
        // }
    }

}


// TODO: Implement Configurable for the config structs
// impl Configurable for LocalCfg {
    
// }

// impl Configurable for GlobalUser {
    
// }

// impl Configurable for GlobalInstall {
    
// }




















