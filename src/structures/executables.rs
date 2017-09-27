
use helpers::*;

use super::VarStr;

use std::path::{Path, PathBuf};
use std::process::{Command, Output, ExitStatus};
use std::env;
use std::ffi::OsString;
use regex::Regex;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Executable {
    source: PathBuf,
    runin: Option<PathBuf>,
    args: Option<VarStr>,
}

// #[derive(Serialize, Deserialize, Debug)]
// pub struct Exe <T: ::structures::HasVars> {
//     source: PathBuf,
//     runin: Option<PathBuf>,
//     args: Option<T>,
// }

impl Executable {
    pub fn blank() -> Executable {
            Executable {
            source: PathBuf::from(""),
            runin: None,
            args: None,
        }
    }
    pub fn new(source: PathBuf, args: VarStr) -> Executable {
        Executable {
            source,
            runin: None,
            // args: if args.args.is_some() && args.args.unwrap_or("") !="" {
            args: if &args.string() != "" { // && args.args.unwrap_or("") !="" {
                Some(args)
            } else {
                None
            },
        }
    }
    pub fn from(src: String, runin: String, args: String) -> Executable {
        Executable {
            source: PathBuf::from(src),
            runin: if runin != "".to_string() {
                let runin_path: PathBuf = PathBuf::from(&runin);
                if runin_path.exists() {
                    Some(PathBuf::from(runin))
                } else {
                    None
                }
            } else {
                None
            },
            args: if args != "".to_string() {
                Some(VarStr::from(args))
            } else {
                None
            }
        }
    }
    pub fn from_str<'a>(src: &'a str) -> Executable {
        let v: Vec<&str>;
        lazy_static! {
            // static ref RUNIN: Regex = Regex::new(r"(?P<run>run [])");
            // static ref RUNIN: Regex = Regex::new(r#"run ((['"]).*?\2|[^ ]*)(?: in (.*?))?(?: with (.*))"#);
            static ref RUNIN: Regex = Regex::new(r#"run ((['"]).*?\2|[^ ]*)(?: in (.*?))?(:? with (.*))?$"#).unwrap();
        }
        // let r = RUNIN.captures(src);
        if let Some(cap) = RUNIN.captures(src) {
            // let output = format!("{}{}{}", cap.get(0), cap.get(2), cap.get(3));
            /*Executable {
                source: PathBuf::from(cap.get(0).unwrap().as_str()),
                runin: if cap.get(2).is_some() && cap.get(2).unwrap_or("".to_string()).as_str() != "" {
                    Some( cap.get(2).unwrap_or("".to_string()) )
                } else {
                    None
                },
                args: if cap.get(3).is_some() && cap.get(3).unwrap_or("".to_string()).as_str() != "" {
                    Some( cap.get(3).unwrap_or("".to_string()) )
                } else {
                    None
                },
            }*/
            return Executable {
                source: PathBuf::from(cap.get(0).unwrap().as_str()),
                runin: if let Some(res) = cap.get(2) {
                        Some(PathBuf::from(res.as_str()))
                    } else {
                        None
                    },
                
                // if cap.get(2).is_some() && cap.get(2).unwrap_or("".to_string()).as_str() != "" {
                //     Some( cap.get(2).unwrap_or("".to_string()) )
                // } else {
                //     None
                // },
                args: if let Some(res) = cap.get(3) {
                    Some(VarStr::from_str(res.as_str()))
                } else {
                    None
                },
                // args: if cap.get(3).is_some() && cap.get(3).unwrap_or("".to_string()).as_str() != "" {
                //     Some( cap.get(3).unwrap_or("".to_string()) )
                // } else {
                //     None
                // },
            }
        }
        // find next double quote that is followed by a space
        // this is because the exe must have a space before args
        // and this way it removes both the space and the quote
        else if src.starts_with("\"") {
            // let v: Vec<&str> = src.splitn(2, "\" ").collect();
            // remove first character (a double quote) and split with up to 2 items
            v = src[1..].splitn(2, "\" ").collect();
        } else if src.starts_with("' ") {
             v = src[1..].splitn(2, "' ").collect();
        }else {
            // split based on first space
            // let v: Vec<&str> = src.splitn(2, " ").collect();
            v = src.splitn(2, " ").collect();
        }
        match v.len() {
            0 => {
                Executable {
                    source: PathBuf::new(),
                    runin: None,
                    args: None,
                }
            }, 
            1 => {
                Executable {
                    source: PathBuf::from(String::from(v[0])),
                    runin: None,
                    args: None,
                } 
            },
            2 => {
                Executable {
                    source: PathBuf::from(String::from(v[0])),
                    runin: None,
                    args: if v[1] != "" {
                        Some(::structures::var_str::VarStr::from_str(v[1]))
                    } else {
                        None
                    },
                }
            },
            _ => {
                unreachable!();
            }
        }
    }
}




impl ::structures::Runnable for Executable {
    fn exists(&self) -> bool {
        self.source.exists()
    }
    fn run(&self) -> Result<String, String> {
        if self.exists() {
            let mut cmd = Command::new(self.source.as_os_str());
            lazy_static! {
                static ref SPLIT_ARGS: Regex = Regex::new(r#"([^ ]*)[^"] "#).unwrap();
            }
            let mut args: Vec<&str>;
            if self.args.is_some() {
                // let arg = match self.args {
                //     Some(VarStr::Parsed(var)) => var.string,
                //     // maybe add a global static for containing the Global config
                //     Some(VarStr::Unparsed(var)) => var.string, // call the parse_vars()
                //     _ => String::new(),
                // };
                // let arg = match self.args.unwrap() {
                //     VarStr::Parsed(var) => var.string,
                //     // maybe add a global static for containing the Global config
                //     VarStr::Unparsed(var) => var.string, // call the parse_vars()
                //     _ => String::new(),
                // };
                if self.args.is_some() {
                    if let Some(ref argd) = self.args {
                        let arg = argd.string();
                        // let arg = self.args.unwrap().string();
                        let args = ::helpers::split_string(&arg);
                        cmd.args(&args);
                        // for a in args {
                        //     cmd = *cmd.arg(a);
                        // }
                    }
                }
                if let Some(ref runin) = self.runin {
                    if runin.exists() {
                        cmd.current_dir(&runin);
                    }
                }
            }
            // let result = cmd.output().unwrap_or(Ouput { status: ExitStatus {}, stdout: vecu8, stderr: vecu8};
            if let Ok(result) = cmd.output() {
                if result.stderr.len() != 0 {
                    Err(String::from_utf8_lossy(&result.stderr).into_owned())
                } else {
                    Ok(String::from_utf8_lossy(&result.stdout).into_owned())
                }
            } else {
                Err("Error: Executable could not be executed".to_string())
            }
            
            // if let Ok(result) = cmd.output() {
            //     Ok(result)
            // } else {
            //     Err("Error: Executable could not be executed.".to_string())
            // }
        } else {
            Err("Error: Executable specified does not exist.".to_string())
        }
    }
}
