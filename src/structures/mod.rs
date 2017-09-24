
pub mod var_str;
use self::var_str::*;

pub mod executables;
use self::executables::*;

pub mod defaults;
use structures::defaults::*;
// use structures::defaults::{DEFAULT_VCS, DEFAULT_VERSION_INC, DEFAULT_LANGUAGE};

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use ::serde::{Deserialize, Serialize};
use ::rmps::{Deserializer, Serializer};


pub trait HasVars {
    fn list_vars(&self) -> Vec<String>;
    // fn replace_vars<T: ::configuration::storage::Configurable(&self) -> VarStr;
    // fn replace_vars<T: ::configuration::storage::Configurable>(&self, T) -> VarStr;
    fn replace_with<'a>(&self, &HashMap<&'a str, &'a str>) -> VarStr;
}
pub trait Runnable {
    fn exists(&self) -> bool;
    fn run(&self) -> Result<String, String>;
}

// #[derive(Debug)]
#[derive(Serialize, Deserialize, Debug)]
pub enum Language {
    AutoHotKey,
    AutoIt,
    C,
    CPlusPlus,
    CSharp,
    Go,
    Haskell,
    Java,
    JavaScript,
    Php,
    Python,
    Ruby,
    Rust,
    Text,
    Web,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum VersionControl {
    Git,
    Hg,
    Cvs,
    // Custom(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum VersionInc {
    Major,
    Minor,
    Patch,
    None,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum OperatingSystem {
    Windows64,
    Windows32,
    Linux32,
    Linux64,
    Mac32,
    Mac64,
    Android32,
    Android64,
    Unknown,
}


#[cfg(target_os = "macos")]
static OPERATING_SYSTEM: &'static str = "Mac";
#[cfg(target_os = "macos")]
static DEFAULT_INSTALL_PATH: &'static str = "/Applications/proman";
// #[cfg(target_os = "macos")]

#[cfg(target_os = "linux")]
static OPERATING_SYSTEM: &'static str = "Linux";
#[cfg(target_os = "linux")]
static DEFAULT_INSTALL_PATH: &'static str = "/usr/local/proman";

#[cfg(target_os = "android")]
static OPERATING_SYSTEM: &'static str = "Android";
#[cfg(target_os = "android")]
static DEFAULT_INSTALL_PATH: &'static str = "/data/data/proman";

#[cfg(target_os = "windows")]
static OPERATING_SYSTEM: &'static str = "Windows";
#[cfg(target_os = "windows")]
static DEFAULT_INSTALL_PATH: &'static str = r#"C:\Program Files\proman"#;

// #[cfg(not(OPERATING_SYSTEM))]
// static OPERATING_SYSTEM: &'static str = "Unknown";
// #[cfg(not(DEFAULT_INSTALL_PATH))]
// static DEFAULT_INSTALL_PATH: &'static str = "/proman";

#[cfg(target_pointer_width = "32")]
static ARCHITECTURE: u8 = 32;
#[cfg(target_pointer_width = "64")]
static ARCHITECTURE: u8 = 64;
// #[cfg(not(ARCHITECTURE))]
// static ARCHITECTURE: u8 = 0;

// name is a little misleading, can also hold just a folder
#[derive(Serialize, Deserialize, Debug)]
pub struct Document {
    source: PathBuf,
}
// a document that is passed as a parameter to an executable
// an example of the run_str looks like "{{exe}} {{doc}}"
#[derive(Serialize, Deserialize, Debug)]
pub struct ExeDoc {
    doc: PathBuf,
    exe: Option<PathBuf>,
    run_str: Option<VarStr>,
}

impl OperatingSystem {
    pub fn new() -> OperatingSystem {
        match OPERATING_SYSTEM {
            "Windows" => match ARCHITECTURE {
                32 => OperatingSystem::Windows32,
                64 => OperatingSystem::Windows64,
                _ => OperatingSystem::Windows32,
            },
            "Mac" => match ARCHITECTURE {
                32 => OperatingSystem::Mac32,
                64 => OperatingSystem::Mac64,
                _ => OperatingSystem::Mac32,
            },
            "Linux" => match ARCHITECTURE {
                32 => OperatingSystem::Linux32,
                64 => OperatingSystem::Linux64,
                _ => OperatingSystem::Linux32,
            },
            "Anroid" => match ARCHITECTURE {
                32 => OperatingSystem::Android32,
                64 => OperatingSystem::Android64,
                _ => OperatingSystem::Android32,
            },
            _ => OperatingSystem::Unknown,
        }
    }
    pub fn to_str(&self) -> &'static str {
        match *self {
            OperatingSystem::Windows64 => "Windows64",
             OperatingSystem::Windows32 => "Windows32",
             OperatingSystem::Mac64 => "Mac64",
             OperatingSystem::Mac32 => "Mac32",
             OperatingSystem::Linux64 => "Linux64",
             OperatingSystem::Linux32 => "Linux32",
             OperatingSystem::Android64 => "Anroid64",
             OperatingSystem::Android32 => "Android32",
             OperatingSystem::Unknown => "Unknown",
             _ => "Unknown",
        }
    }
    pub fn install_path() -> PathBuf {
        // match OPERATING_SYSTEM {
        //     // "Windows" if ARCHITECTURE == 32 => PathBuf::from(r#"C:\Program files (x86)"#),
        //     "Windows" if ARCHITECTURE == 64 => PathBuf::from(r#"C:\Program files\proman"#),
        //     _ => PathBuf::from(DEFAULT_INSTALL_PATH),
        // }
        PathBuf::from(DEFAULT_INSTALL_PATH)
    }
    pub fn install_folder() -> PathBuf {
        let mut dir: PathBuf = PathBuf::from(DEFAULT_INSTALL_PATH);
        // dir.parent().expect("Could not retrieve default install path.").to_path_buf()
        dir.parent().unwrap_or(dir).to_path_buf()
    }
    pub fn architecture() -> u8 {
        ARCHITECTURE
    }
    pub fn get_install_path(&self) -> PathBuf {
        match *self {
            OperatingSystem::Windows64 | OperatingSystem::Windows32 => PathBuf::from(r#"C:\Program Files\proman"#),
            OperatingSystem::Linux64 | OperatingSystem::Linux32 => PathBuf::from(r#"/usr/local/proman"#),
            OperatingSystem::Mac64 | OperatingSystem::Mac32 => PathBuf::from(r#"/Applications/proman"#),
            OperatingSystem::Android64 | OperatingSystem::Android32 => PathBuf::from(r#"/data/data/proman"#),
            OperatingSystem::Unknown => PathBuf::from("/proman"),
            _ => PathBuf::from("/proman"),
        }
    }
    pub fn get_install_folder(&self) -> PathBuf {
        match *self {
            OperatingSystem::Windows64 | OperatingSystem::Windows32 => PathBuf::from(r#"C:\Program Files\"#),
            OperatingSystem::Linux64 | OperatingSystem::Linux32 => PathBuf::from(r#"/usr/local/"#),
            OperatingSystem::Mac64 | OperatingSystem::Mac32 => PathBuf::from(r#"/Applications/"#),
            OperatingSystem::Android64 | OperatingSystem::Android32 => PathBuf::from(r#"/data/data/"#),
            OperatingSystem::Unknown => PathBuf::from("/"),
            _ => PathBuf::from("/"),
        }
    }
    pub fn get_architecture(&self) -> u8 {
        match *self {
            OperatingSystem::Windows64 => 64,
            OperatingSystem::Windows32 => 32,
            OperatingSystem::Linux64 => 64,
            OperatingSystem::Linux32 => 32,
            OperatingSystem::Mac64 => 64,
            OperatingSystem::Mac32 => 32,
            OperatingSystem::Android64 => 64,
            OperatingSystem::Android32 => 32,
            OperatingSystem::Unknown => if ARCHITECTURE == 0 { 32 } else { ARCHITECTURE },
            _ => if ARCHITECTURE == 0 { 32 } else { ARCHITECTURE },
        }
    }
}

impl Language {
    pub fn from_str<'a>(string: &'a str) -> Language {
        match string.to_lowercase().as_str() {
            "autohotkey" | "ahk" => Language::AutoHotKey,
            "autoit" => Language::AutoIt,
            "c" => Language::C,
            "cplusplus" | "c++" => Language::CPlusPlus,
            "csharp" | "c#" => Language::CSharp,
            "go" | "golang" => Language::Go,
            "haskell" => Language::Haskell,
            "java" => Language::Java,
            "javascript" | "js" => Language::JavaScript,
            "php" => Language::Php,
            "python" => Language::Python,
            "ruby" => Language::Ruby,
            "rust" => Language::Rust,
            "text" => Language::Text,
            "web" | "html" | "css" | "xml" | "xhtml" => Language::Web,
            _ => DEFAULT_LANGUAGE,
        }
    }
    pub fn to_str(&self) -> &'static str {
        match *self {
            Language::AutoHotKey => "autohotkey",
            Language::AutoIt => "autoit",
            Language::C => "c",
            Language::CPlusPlus => "cplusplus",
            Language::CSharp => "csharp",
            Language::Go => "go",
            Language::Haskell => "haskell",
            Language::Java => "java",
            Language::JavaScript => "javascript",
            Language::Php => "php",
            Language::Python => "python",
            Language::Ruby => "ruby",
            Language::Rust => "rust",
            Language::Text => "text",
            Language::Web => "web",
        }
    }
}

impl VersionControl {
    pub fn from_str<'a>(string: &'a str) -> VersionControl {
        match string.to_lowercase().as_str() {
            "git" => VersionControl::Git,
            "hg" | "mercurial" => VersionControl::Hg,
            "cvs" => VersionControl::Cvs,
            _ => DEFAULT_VCS,
        }
    }
    pub fn to_str(&self) -> &'static str {
        match *self {
            VersionControl::Git => "git",
            VersionControl::Hg => "hg",
            VersionControl::Cvs => "cvs",
            
        }
    }
}

impl VersionInc {
    pub fn from_str<'a>(string: &'a str) -> VersionInc {
        match string.to_lowercase().as_str() {
            "major" => VersionInc::Major,
            "minor" => VersionInc::Minor,
            "patch" => VersionInc::Patch,
            _ => DEFAULT_VERSION_INC,
        }
    }
    pub fn to_str(&self) -> &'static str {
        match *self {
            VersionInc::Major => "major",
            VersionInc::Minor => "minor",
            VersionInc::Patch => "patch",
            _ => "",
        }
    }
}




























