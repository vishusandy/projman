

use std::path::{Path, PathBuf};
use ::std::collections::HashMap;

use ::structures::*;
use ::structures::executables::*;
use ::configuration::*;
use ::configuration::storage::*;


// pub struct 
// action, Vec<Executable>
// use = for assigning action executable, += and =+ (maybe -=) for prepending and appending autoruns?
// action = Executable
// action += Executable // prepend
// action =+ Executable // append
// first try json deserialization, then Executable::from_str

// look for project_behavior.cfg in local, as well as for
//      <language>.<projtype>cfg in user, then install directories.
// The possible three files should be aggregated, filtering out
//      languages and project types that do not match the current lang/type
// The local file is assumed to have all actions and variables assigned
//      to the current language/project type
//
// also allow globs for language/projtype
// If there is no local config, or no language specified, actions that
// are global will filter out all commands/autoruns that are not globs

// The User and Install behavior files will have a <language>.<projtype> prefix
//    [actions]
//      <language>.<projtype>.<action> = Executable
//      <language>.<projtype>.Global = Executable  // Any global action, must be uppercase Global
//      <language>.<projtype>.Local = Executable   // Any local action, must be uppercase Local
//      Rust-MultiBinary|Rust-Library|C-*|C++-*|D.*
//      Formula: splitn(3, "")
//              OLD Formula: if (langs== "*" || (Rust, C, C++, D).contains(CurrentLanguage)) && (projtypes == "*" || (<types>).contains(CurrentType))
// To attach a command or autorun 
//    [autorun-before|autorun-after]
//    [variables]
//      <language>.<projtype>.<var> = var value
//
// can use globs for replacing language, projtype, and action
//      <language>.*.<action>
//      this will not actually add anything to every single language behavior
//      but will not check if the language matches the current language before
//      adding the action/autorun/var to the current




// make structs that hold a Behavior struct and implement Behave
// for each language

pub struct AutoRun {
    
}

pub struct Action {
    before: Vec<Executable>,
    command: Vec<Executable>,
    after: Vec<Executable>,
}

pub struct Behavior {
    backup: Vec<Executable>,
    
    // how to do autoruns, how to trigger custom functionality
    custom: HashMap<String, Vec<Executable>>,
    vars: HashMap<String, VarStr>,
}

pub trait Behave {
    fn blank() -> Self;
    fn get_behavior() -> Self; // uses the parse_behavior function to load into custom language struct
    fn save_behavior() -> bool;
    fn perform_action<'a, 'v>(&'a str, Vec<&'v str>) -> Result<String, String> {
        // default action is to just run the command
    }
    fn parse_vars(&mut self) {
        // parse VarStr's
        // the vars should be parsed first, then all other VarStr's
        // so that the vars in behavior can be added to the list
        // of vars to parse within other VarStr's
    }
    fn parse_behavior() -> Behavior {
        let mut section = "actions";
        // read file
        // iterate over lines
        // split on ` = ` or += or =+ by finding first space then reading next two chars
        let mut behavior: Behavior = self::blank();
        
        let section_variables = |key: &str, val: &str, behavior: &mut Behavior| {
            
        };
        
        
        
    }
    
}

impl Behavior {
    pub fn blank() -> Behavior {
        Behavior {
            
        }
    }
}











