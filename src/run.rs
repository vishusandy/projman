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
use std::collections::HashMap;
// extern crate regex;
// use regex::Regex;

use ::structures::*;
use ::structures::var_str::*;

#[cfg(test)]
mod tests {
    use ::structures::*;
    use ::structures::var_str::*;
    // use ::structures::var_str::{VarStr};
    use std::collections::HashMap;
    // use structures::*;
    // use structures::var_str::*;
    use std::sync::Mutex;

    lazy_static! {
        // static ref REPS: Mutex<HashMap<&'static str, &'static str>> = {
        static ref REPS: HashMap<&'static str, &'static str> = {
            // let replacements: HashMap<&'static str, &'static str>;
            let mut reps = HashMap::new();
            reps.insert("proj_type", "Rust.Binary");
            reps.insert("language", "Rust");
            reps.insert("proj_dir", r#"c:\code\proj\protest"#);
            reps.insert("smh", "shake_my_head");
            // replacements = reps;
            // replacements
            reps
            // Mutex::new(reps)
        };
    }
    
    
    // set RUST_TEST_NOCAPTURE=1
    #[test]
    fn print_replace_with() {
        let mut reps: HashMap<&str, &str> = HashMap::new();
        reps.insert("proj_type", "Rust.Binary");
        reps.insert("language", "Rust");
        reps.insert("proj_dir", r#"c:\code\proj\protest"#);
        reps.insert("smh", "shake_my_head");
        // reps.insert("", "");
        // reps.insert("", "");
        println!("Result1: {}", VarStr::from_str("-a [[smh]] -b [[proj_type]] -c [[language]] -d [[proj_dir]]").replace_with(&reps).string());
        
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
        let vs2: VarStr = vs.replace_with(&reps);
        println!("Original str: {}", test);
        println!("Replaced str: {}", vs2.string());
        // println!("Replace Result: \n{:?}", vs2);
        assert_eq!(vs2.string(), result);
    }
    #[test]
    fn test_replace_with_arg() {
        // let test = "-a [[arg:*]] -b [[arg:0]] -c [[arg:1]] -d [[arg:-b]] -e [[arg:-s,--some]] -f [[env:2]]";
        // let result = "";
        // let vs: VarStr = VarStr::from_str(test);
        // let rst: VarStr = vs.replace_with(&REPS);
        // println!("Original str: {}\nReplaced str: {}", test, rst.string());
        // assert_eq!(test, rst.string());
    }
    // fn test_replace_with_arg2() {
    //     let test = "";
    //     let result = "";
    //     let vs: VarStr = VarStr::from_str(test);
    //     let rst: VarStr = vs.replace_with(REPS);
    //     println!("Original str: {}\nReplaced str: {}", test, rst.string());
    //     assert_eq!(test, rst.string());
    // }
}

fn main() {
    
    // error, warn, info, debug, trace
    env_logger::init();
    
    println!("Hello, world!");
    println!("This is the `run` command!");
    
    // ::configuration::storage::Debug::store_configs_blank();
    
    let mut reps: HashMap<&str, &str> = HashMap::new();
    reps.insert("proj_type", "Rust.Binary");
    reps.insert("language", "Rust");
    reps.insert("proj_dir", r#"c:\code\proj\protest"#);
    reps.insert("smh", "shake_my_head");
    
    let test = "-c [[env:~]] - d [[env:$]] -a [[arg:~]] -b [[arg:$]] -z [[!flag: --some   ]] -y [[flag:  --something    ]] -x [[flag: -s   , --some    ]] -w [[!flag:  -a,    -b   ]] -a [[arg:$]] -b [[arg:0]] -c [[arg:1]] -d [[arg:-b]] -e [[arg:-s,--some]] -f [[env:2]] -g [[flag:-b]] - h [[!flag:-z,--some]] -i [[!flag:-z,--zinger]] -j [[language]]";
    // let result = "";
    let vs: VarStr = VarStr::from_str(test);
    let rst: VarStr = vs.replace_with(&reps);
    println!("Original str: {}\nReplaced str: {}", test, rst.string());
    
    // println!("--------------------");
    println!("--------------------");
    
    // println!("Printing out env variables:");
    // for (key, val) in env::vars() {
    //     println!("key: {key: <width$} val:{val}", key=key, width=30 , val=val);
    // }
    
    
    
    
    
    
}




