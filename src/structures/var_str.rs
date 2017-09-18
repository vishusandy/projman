



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
    Error,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ParsedVar {
    string: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UnparsedVar {
    string: String,
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
}
impl HasVars for VarStr {
    fn list_vars(&self) -> Vec<String> {
        if let VarStr::Unparsed(unparsed) = self {
            let string = unparsed.string;
            lazy_static! {
                static ref VARS: Regex = Regex::new(r#"[[(.*?)]]"#).unwrap();
            }
            // this is -a [[test of "epic" proportions]] and [[more]]
            let mut found: Vec<String> = Vec::new();
            for var in VARS.captures_iter(self) {
                // if let Some() = caps.get(1 { }
                found.push(&var[1].to_string());
            }
            found
        } else {
            Vec::new()
        }
    }
    fn replace_vars<T: Configurable>(&self, cfg: T) -> VarStr {
        let list: Vec<String> = self.list_vars();
        if let Unparsed(unparsed) = self {
            let string = unparsed.string;
            let mut new = string.clone();
            for var in list {
                // Method A: Add trait method to retrieve a variable
                // cfg.get_var
                
                // Method B: Provide a HashMap or Tuple Vector listing all variables
                
                // Method C: Match each var found to a specific field
                let replace = match var.to_lowercase().as_str() {
                    "lang" | "language" =>  "", //Global.local.proj_type.project_type(),
                    
                    // anything else
                    a => {
                        let mut original: String = String::with_capacity(var.len()+6);
                        original.push_str("[[");
                        original.push_str(var);
                        original.push_str("]]");
                        original
                    },
                };
                new.replace(&var, replace);
                
            }
            
        } else {
            self
        }
    }
    fn replace_custom<'a>(&self, vars: HashMap<&'a str, &'a str>) -> VarStr {
        
        // TODO: actually implement this
        VarStr::Unparsed( UnparsedVar {
            string: String::new()
        })
        
        
    }
}

