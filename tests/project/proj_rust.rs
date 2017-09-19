
use super::*;

/*pub enum RustReleaseChannels {
    Stable,
    Beta,
    Nightly,
}*/
/*
pub enum GlobalCommand {
    Docs,
    Info,
    List,
    New,
    Template,
}
pub enum InjectCommand {
    Backup,
    Build,
    Check,
    Commit,
    Interpret,
    Run,
    Save,
    Serve,
    Upload,
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum Command {
    Backup,
    Bench,
    Build,
    Check,
    Commit,
    Docs,
    Info,
    Interpret,
    List,
    New,
    Run,
    Save,
    Serve,
    Test,
    Template,
    Upload,
    Error,
}
*/


pub enum RustBuildMode {
    Debug,
    Release,
    Custom(String),
}

pub struct RustVars {
    test_threads: u8,
    version: Version,
    default_mode: RustBuildMode,
    default_compiler: String, // used by rustup
    
}

pub struct RustMultiBinary {
    vars: RustVars,
}

impl RustMultiBinary {
    
}

// impl Project for RustVars {
    
// }