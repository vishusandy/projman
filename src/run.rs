extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate rmp_serde as rmps;
extern crate serde_yaml;

#[macro_use] extern crate log;
extern crate env_logger;

#[macro_use] extern crate lazy_static;

extern crate os_type;
extern crate regex;
extern crate semver;


// mod configuration;
// mod project;
// mod structures;
// mod manager;
// mod helpers;

mod configuration;
mod project;
mod structures;
mod manager;
mod helpers;

use manager::*;

use std::process::{Command, Output, ExitStatus};
use std::path::{Path, PathBuf};
use std::env;
use std::ffi::OsString;

// extern crate regex;
// use regex::Regex;


#[cfg(test)]
mod tests {
    use ::structures::*;
    use ::structures::var_str::*;
    // use ::structures::var_str::{VarStr};
    use std::collections::HashMap;
    // use structures::*;
    // use structures::var_str::*;
    
    #[test]
    fn print_replace_with() {
        let mut reps: HashMap<&str, &str> = HashMap::new();
        reps.insert("proj_type", "Rust.Binary");
        reps.insert("language", "Rust");
        reps.insert("proj_dir", r#"c:\code\proj\protest"#);
        reps.insert("smh", "shake_my_head");
        // reps.insert("", "");
        // reps.insert("", "");
        println!("Result1: {}", VarStr::from_str("-a [[smh]] -b [[proj_type]] -c [[language]] -d [[proj_dir]]").replace_with(reps).string());
        
    }
    #[test]
    fn test_replace_with() {
        let test = "-p [[proj_type]] --cpu_level [[env:PROCESSOR_LEVEL]]";
        let result = "-p Rust.Binary --cpu_level 6";
        let mut reps: HashMap<&str, &str> = HashMap::new();
        reps.insert("proj_type", "Rust.Binary");
        reps.insert("language", "Rust");
        reps.insert("proj_dir", r#"c:\code\proj\protest"#);
        // reps.insert("", "");
        // reps.insert("", "");
        // reps.insert("", "");
        let vs: VarStr = VarStr::from_str(test);
        let vs2: VarStr = vs.replace_with(reps);
        println!("Original str: {}", test);
        println!("Replaced str: {}", vs2.string());
        println!("Replace Result: \n{:?}", vs2);
        assert_eq!(vs2.string(), result);
    }

}

fn main() {
    
    // error, warn, info, debug, trace
    env_logger::init();
    
    println!("Hello, world!");
    println!("This is the `run` command!");
    
    ::configuration::storage::Debug::store_configs_blank();
    
    
    
    
    
    
    
    
    
}




