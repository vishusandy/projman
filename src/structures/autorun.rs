
use std::path::{Path, PathBuf};
use ::std::collections::HashMap;

use ::structures::*;
use ::structures::executables::*;
use ::configuration::*;
use ::configuration::storage::*;
use ::configuration::storable::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AutoRun {
    Before(Executable),
    After(Executable),
    // OnceBefore(Executable),
    // OnceAfter(Executable),
    AlwaysBefore(Executable), // does not break if the command or other scripts fail
    AlwaysAfter(Executable), // does not break if the command or other scripts fail
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AutoRuns {
    autoruns: Vec<AutoRun>,
    
}

pub trait AutoRunnable {
    fn get_before(&self) -> AutoRuns;
    fn get_after(&self) -> AutoRuns;
    fn run_all(&self) -> Vec<Result<String, String>>;
    fn run_before(&self) -> Vec<Result<String, String>> {
        self.get_before().run_all()
    }
    fn run_after(&self) -> Vec<Result<String, String>> {
        self.get_after().run_all()
    }
}
    
impl AutoRunnable for AutoRuns {
    fn get_before(&self) -> AutoRuns {
        let mut new: Vec<AutoRun> = Vec::new();
        for a in &self.autoruns {
            match a {
                &AutoRun::Before(_) | &AutoRun::AlwaysBefore(_)
                    => {
                        new.push(a.clone())
                    },
                _ => {},
            }
        }
        AutoRuns {
            autoruns: new,
        }
    }
    fn get_after(&self) -> AutoRuns {
        let mut new: Vec<AutoRun> = Vec::new();
        for a in &self.autoruns {
            match a {
                &AutoRun::After(_) | &AutoRun::AlwaysAfter(_)
                    => {
                        new.push(a.clone())
                    },
                _ => {},
            }
        }
        AutoRuns {
            autoruns: new,
        }
    }
    fn run_all(&self) -> Vec<Result<String, String>> {
        let mut error = false;
        let mut results: Vec<Result<String, String>> = Vec::new();
        for a in &self.autoruns {
            match a {
                &AutoRun::Before(ref exe) | &AutoRun::After(ref exe) if error == false
                    => {
                        let rst = exe.run();
                        match rst {
                            Err(e) => {
                                error = true;
                                results.push(Err(e));
                            },
                            Ok(o) => {
                                results.push(Ok(o));
                            },
                        }
                    },
                &AutoRun::AlwaysBefore(ref exe) | &AutoRun::AlwaysAfter(ref exe) 
                    => {
                        let rst = exe.run();
                        match rst {
                            Err(e) => {
                                error = true;
                                results.push(Err(e));
                            },
                            Ok(o) => {
                                results.push(Ok(o));
                            },
                        }
                        // results.push(rst.clone()); 
                        // if let Err(err) = rst {
                        //     error = true;
                        // }
                    },
                _ => {},
            }
            
        }
        results
    }
}
