



use std::env;
use std::path::{Path, PathBuf};

use std::ffi::OsString;
use regex::Regex;
use super::*;
use structures::var_str::VarStr::*;
use std::collections::HashMap;
use configuration::storage::Configurable;
use std::ffi::OsStr;

use ::serde::{Deserialize, Serialize};
use ::rmps::{Deserializer, Serializer};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum VarStr {
    Parsed(ParsedVar),
    Unparsed(UnparsedVar),
    // Error,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ParsedVar {
    pub string: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UnparsedVar {
    pub string: String,
}

impl VarStr {
    pub fn blank() -> VarStr {
        VarStr::Unparsed( UnparsedVar { string: String::from("") } )
    }
    pub fn new(string: String) -> VarStr {
        VarStr::Unparsed( UnparsedVar { string: string } )
    }
    pub fn from(string: String) -> VarStr {
        VarStr::Unparsed( UnparsedVar { string: string } )
    }
    pub fn from_str<'a>(string: &'a str) -> VarStr {
        VarStr::Unparsed( UnparsedVar { string: string.to_string() } )
    }
    pub fn string(&self) -> String {
        match *self {
            VarStr::Unparsed(ref var) => var.string.clone(),
            VarStr::Parsed(ref var) => var.string.clone(),
            // _ => String::new(),
        }
    }
    pub fn clone(&self) -> VarStr {
        match self {
            &VarStr::Unparsed(ref var) => VarStr::Unparsed(UnparsedVar { string: var.string.clone() } ),
            &VarStr::Parsed(ref var) => VarStr::Parsed(ParsedVar { string: var.string.clone() } ),
        }
    }
    // pub fn str<'a>(&self) -> &'a str {
    //     if let &Unparsed(ref var) = self {
    //         &var.string
    //     } else if let &Parsed(ref var) = self {
    //         &var.string
    //     } else {
    //         ""
    //     }
    //     // match *self {
    //     //     VarStr::Unparsed(var) => string(),
    //     //     VarStr::Parsed(var) => var.string.clone().as_str(),
    //     //     // _ => "",
    //     // }
    // }
}

// if an argument has a value after the -a/--anything part
// like -a something
// the value will not be removed, just the flag -a
pub fn remove_flags(all_args: &Vec<String>) -> Vec<String> {
    // let all_args: Vec<String> = env::args().collect();
    let mut new_args: Vec<String> = Vec::new();
    for i in 1..all_args.len() {
        if !all_args[i].trim().starts_with("-") {
            new_args.push(all_args[i].trim().to_string());
        }
    }
    new_args
}

impl HasVars for VarStr {
    fn list_vars(&self) -> Vec<String> {
        if let &VarStr::Unparsed(ref unparsed) = self {
            let string = unparsed.string.clone();
            lazy_static! {
                static ref VARS: Regex = Regex::new(r#"\[\[(.*?)\]\]"#).unwrap();
            }
            // this is -a [[test of "epic" proportions]] and [[more]]
            let mut found: Vec<String> = Vec::new();
            for var in VARS.captures_iter(&self.string()) {
                // if let Some() = caps.get(1 { }
                found.push(var[1].to_string());
            }
            found
        } else {
            Vec::new()
        }
    }
    
    fn replace_with<'a>(&self, vars: &HashMap<&'a str, &'a str>) -> VarStr {
        let list: Vec<String> = self.list_vars();
        if let &Unparsed(ref unparsed) = self {
            let string = unparsed.string.clone();
            let mut new = string.clone();
            // default behavior is to replace a [[...]] that does not
            // match with a blank string, to leave the [[...]] in place
            // uncomment out the: `continue 'varlist;` lines found below
            lazy_static! {
                static ref IS_NUMERIC: Regex = Regex::new(r#"^[0-9]{1,2}$"#).unwrap();
            }
            'varlist: for variable in list {
                // what is arg_t ?????
                let mut replace: String = "".to_string();
                let mut reverse: bool = false;
                let mut var = variable;
                if var.starts_with("!flag:") {
                    reverse = true;
                    var = (&var[1..]).to_string();
                    // println!("!flag: found, setting var to `{}`", var);
                }
                if var.trim().starts_with("arg:") {
                    let argument = &var.trim()[4..].trim();
                    if argument.trim() == "~" {
                        let cur_proc = env::current_exe();
                        match cur_proc {
                            Ok(p) => {
                                let path = p.clone();
                                replace = format!("{}", path.display());
                            },
                            _ => {
                                // println!("Current_exe() failed for ~, falling back");
                                let all_args: Vec<String> = env::args().collect();
                                replace = all_args[0].clone();
                            },
                        }
                    } else if argument.trim() == "$" {
                        // Display just the running process name
                        // let all_args: Vec<String> = env::args().collect();
                        // replace = all_args[0].clone();
                        let cur_proc = env::current_exe();
                        match cur_proc {
                            Ok(p) => {
                                let path = p.clone();
                                replace = path.file_name().unwrap_or(OsStr::new("")).to_str().unwrap_or("").to_string();
                            },
                            _ => {
                                // println!("Current_exe() failed for $, falling back");
                                let all_args: Vec<String> = env::args().collect();
                                replace = all_args[0].clone();
                            },
                        }
                    } else if argument == &"*" {
                        // entire command argument string
                        let all_args: Vec<String> = env::args().collect();
                        let arg_str = all_args.join(" ");
                        // println!("Joined args into `{}`", arg_str);
                        replace = arg_str;
                    } else if IS_NUMERIC.is_match(argument) {
                        let num_result = (&var[4..]).trim().parse::<u8>();
                        match num_result {
                            Ok(num) => {
                                // let all_args: Vec<String> = env::args().collect();
                                let env_args: Vec<String> = env::args().collect();
                                let all_args = remove_flags(&env_args);
                                if ((num) as usize) < all_args.len() {
                                    replace = all_args[(num) as usize].clone();
                                } else {
                                    // continue 'varlist;
                                }
                            },
                            _ => {
                                // continue 'varlist;
                            },
                        }
                    } else {
                        let all_args: Vec<String> = env::args().collect();
                        if argument.contains(",") {
                            'argslist: for part_raw in argument.split(',') {
                                let part = part_raw.trim();
                                if !part.starts_with("-") {
                                    // warn!("arg list item does not start with a -");
                                    continue 'argslist;
                                }
                                // if all_args.contains()
                                'argmatch: for idx in 1..all_args.len() {
                                    if part.to_lowercase() == all_args[idx].to_lowercase() {
                                        if idx+1 < all_args.len() {
                                            replace = all_args[idx+1].clone();
                                            break 'argmatch;
                                        } else {
                                            // warn!("arg `{}` found in arglist but is last item", part);
                                            continue 'argslist;
                                        }
                                    }
                                }
                            }
                        } else if argument.starts_with("-") {
                            let mut matched: bool = false;
                            'allargslist: for idx in 1..all_args.len() {
                                if argument.to_lowercase() == all_args[idx].to_lowercase() {
                                    if idx+1 < all_args.len() {
                                        matched = true;
                                        replace = all_args[idx+1].clone();
                                        break 'allargslist;
                                    } else {
                                        // warn!("Argument `{}` found in env::args() but it is the last argument, so continuing..", argument);
                                        // continue 'varlist;
                                    }
                                }
                            }
                            if !matched {
                                // if the desired behavior when no match is found
                                // is to replace the [[...]] with an empty string
                                // leave this if section blank, otherwise if the
                                // [[...]] should be left as is uncomment the line:
                                // continue 'varlist;
                            }
                        } else {
                            // error in the arg: format
                            continue 'varlist;
                        }
                    }
                } else if var.starts_with("flag:") {
                    let all_args: Vec<String> = env::args().collect();
                    let arguments = &var[5..].trim();
                    if arguments.contains(",") {
                        let mut matched: bool = false;
                        'flaglist: for raw_argument in arguments.split(",") {
                            // skip checking if the argument starts_with("-") to allow any argument not just flags to be found
                            let argument = raw_argument.trim();
                            if all_args.contains(&argument.to_string()) {
                                matched = true;
                                replace = if !reverse { "true".to_string() } else { "false".to_string() };
                                break 'flaglist;
                            }
                        }
                        if !matched {
                            replace = if !reverse { "false".to_string() } else { "true".to_string() };
                        }
                    } else {
                        if all_args.contains(&arguments.to_string()) {
                            replace = if !reverse { "true".to_string() } else { "false".to_string() };
                        } else {
                            replace = if !reverse { "false".to_string() } else { "true".to_string() };
                        }
                    }
                } else if var.starts_with("env:") {
                    let env_args: Vec<String> = env::args().collect();
                    let all_args = remove_flags(&env_args);
                    let argument = &var.trim()[4..].trim();
                    if argument.trim() == "~" {
                        let cur_proc = env::current_exe();
                        match cur_proc {
                            Ok(p) => {
                                let path = p.clone();
                                replace = format!("{}", path.display());
                            },
                            _ => {
                                // println!("Current_exe() failed for ~, falling back");
                                let all_args: Vec<String> = env::args().collect();
                                replace = all_args[0].clone();
                            },
                        }
                    } else if argument.trim() == "$" {
                        let cur_proc = env::current_exe();
                        match cur_proc {
                            Ok(p) => {
                                let path = p.clone();
                                replace = path.file_name().unwrap_or(OsStr::new("")).to_str().unwrap_or("").to_string();
                            },
                            _ => {
                                // println!("Current_exe() failed for $, falling back");
                                let all_args: Vec<String> = env::args().collect();
                                replace = all_args[0].clone();
                            },
                        }
                    } else if IS_NUMERIC.is_match(argument) {
                        let num_raw = (&var[4..]).trim().parse::<u8>();
                        match num_raw{
                            Ok(num) => {
                                if ((num) as usize) < all_args.len() {
                                    replace = all_args[((num) as usize)].to_string();
                                }
                            },
                            _ => { 
                                // println!("env:# Index too large")
                            },
                        }
                    } else {
                        // check if an env variable exist and use that
                        match env::var(argument) {
                            Ok(val) => {
                                replace = val;
                            },
                            Err(_) => {
                                // continue 'varlist;
                            }
                        }
                    }
                } else {
                    let lower = var.trim().to_lowercase();
                    match vars.get(&lower.as_str()) {
                        Some(val) => {
                            replace = val.to_string();
                        },
                        _ => { 
                            // continue 'varlist;
                        },
                    }
                }
                // debug!("Replacing {} with {}", var, replace);
                if !reverse {
                    new = new.replace(&format!("[[{}]]", var), &replace);
                } else {
                    new = new.replace(&format!("[[!{}]]", var), &replace);
                }
                
            }
            VarStr::Parsed(ParsedVar{
                string: new,
            })
        } else {
            self.clone()
        }
    }
}

