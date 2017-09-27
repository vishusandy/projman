


use std::path::{Path, PathBuf};
use ::std::collections::HashMap;

use ::structures::*;
use ::structures::executables::*;
use ::configuration::*;
use ::configuration::storage::*;


pub struct DefaultLang {
    
}

// TODO:
//   make the language / project type generate a behavior file
//   the behavior file is what determines what happens
//   each project type can have its own behavior reader/handler.
//   Each language/project type has its own behavior variant

pub trait Language {
    fn language(&self) -> String; // maybe a 'static str ?
    fn projtype(&self) -> String; // maybe a 'static str ?
    
    fn load_details(PathBuf) -> Self;
    fn save_details(&self, PathBuf) -> bool;
    fn perform_action<'a, 'b>(&'a str, &Vec<&'b str>) -> Result<String, String>; // action, arguments
    
    
    
    // Custom will be called elsewhere.  Parse arguments with VarStr's replace_with from either main() or manager before passing to perform_custom_action
    // fn perform_custom_action<'a, 'b>(&'a str, &Vec<&VarStr>, &Executable) -> Result<String, String>; // action, arguments, action to execute
}




// have a <language>.cfg file in the project directory
// if needed the <language>.cfg file can be unique to the language and project type not just language


