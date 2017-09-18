



use std::env;
use std::path::{Path, PathBuf};

use std::ffi::OsString;
use regex::Regex;
use super::*;
use structures::var_str::VarStr::*;
use std::collections::HashMap;
use configuration::storage::Configurable;

use ::serde::{Deserialize, Serialize};
use ::rmps::{Deserializer, Serializer};

#[derive(Serialize, Deserialize, Debug)]
pub enum VarStr {
    Parsed(ParsedVar),
    Unparsed(UnparsedVar),
    // Error,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ParsedVar {
    pub string: String,
}
#[derive(Serialize, Deserialize, Debug)]
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
impl HasVars for VarStr {
    fn list_vars(&self) -> Vec<String> {
        if let &VarStr::Unparsed(ref unparsed) = self {
            let string = unparsed.string.clone();
            lazy_static! {
                static ref VARS: Regex = Regex::new(r#"[[(.*?)]]"#).unwrap();
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
    fn replace_vars<T: Configurable>(&self, cfg: T) -> VarStr {
        let list: Vec<String> = self.list_vars();
        if let &Unparsed(ref unparsed) = self {
            let string = unparsed.string.clone();
            let mut new = string.clone();
            for var in list {
                // Method A: Add trait method to retrieve a variable
                // cfg.get_var
                
                // Method B: Provide a HashMap or Tuple Vector listing all variables
                
                // Method C: Match each var found to a specific field
                let replace: String = match var.to_lowercase().as_str() {
                    "lang" | "language" =>  "".to_string(), //Global.local.proj_type.project_type(),
                    
                    // anything else
                    a => {
                        let mut original: String = String::with_capacity(var.len()+6);
                        original.push_str("[[");
                        original.push_str(&var);
                        original.push_str("]]");
                        original
                    },
                };
                new.replace(&var, &replace);
                
            }
            VarStr::Parsed(ParsedVar{
                string: new
            })
        } else {
            self.clone()
        }
    }
    fn replace_custom<'a>(&self, vars: HashMap<&'a str, &'a str>) -> VarStr {
        
        // TODO: actually implement this
        VarStr::Unparsed( UnparsedVar {
            string: String::new()
        })
        
        
    }
}

