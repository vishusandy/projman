

use std::path::{Path, PathBuf};
use ::std::collections::HashMap;

use ::structures::*;
use ::structures::executables::*;
use ::configuration::*;
use ::configuration::storage::*;



pub trait LocalDetails : ::configuration::storable::Storable {
    fn load_details() -> Self;
    fn save_details() -> bool;
    fn perform_action() -> Result<String, String>;
    fn add_autoruns(&mut self);
    
    
    
}


// in mai() or manager do something like:
// load_details()
// add_autoruns()
// perform_action()




