
mod var_str;
use var_str::*;

mod executables;
use executables::*;


pub trait HasVars {
    fn list_vars(&self) -> Vec<String>;
    fn replace_vars(&self) -> VarStr;
    fn replace_custom<'a>(&self, HashMap<&'a str, &'a str>) -> VarStr;
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
    Linux64,
    Linux32,
    Mac64,
    Android64,
}



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
        match self {
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
        match self {
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
        match self {
            VersionInc::Major => "major",
            VersionInc::Minor => "minor",
            VersionInc::Patch => "patch",
        }
    }
}




























