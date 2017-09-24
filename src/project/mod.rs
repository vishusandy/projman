use semver::Version;
use std::path::{Path, PathBuf};

mod proj_rust;
use self::proj_rust::*;
use ::serde::{Deserialize, Serialize};
use ::rmps::{Deserializer, Serializer};
use std::marker::PhantomData;

// Implementors of Project trait must also implement Configurable to save/load the config data

// pub trait Project : ::configuration::storage::Configurable {
// pub trait Project<'de> : ::configuration::storage::Configurable<'de> {

// pub trait Project<'de> : ::configuration::storage::Configurable<'de> {
pub trait Project : ::configuration::storage::Configurable {
    // type B: Project<'de>; // collection of actions, a Behavior
    type B: Project; // collection of actions, a Behavior
    
    fn language(&self) -> String;
    fn proj_type(&self) -> String;
    // fn store(&self, PathBuf) -> bool; // the PathBuf indicates the name/path of the file
    // fn retrieve(PathBuf) -> Behavior; // the PathBuf indicates the name/path of the file so config does not need to be passed in to figure out where to go
    
    // https://doc.rust-lang.org/std/slice/trait.SliceConcatExt.html#tymethod.join
    // behavior.action(ExeFile, Arguments)
    // see env::current_exe() -> PathBuf
    // and env::args()
    
    // fn action<'a>(&self, &'a str, Vec<String>) -> Action;
    
    // The Configurable trait's parse_vars() covers this already
    // fn list_custom_vars<'a>(&self) -> Vec<&'a str>;
    // fn replace_custom_vars(VarStr) -> String;
    
    // change from String to VarStr
    fn action_build<'a>(proj_details: &Self::B, args: Vec<String>) -> Action;
    
}

pub fn find_proj_details<T>(local_opt: Option<Local>) -> Option<T> where T: ::project::Project {

}

#[derive(Serialize, Deserialize, Debug)]
pub struct Action {
    executable: PathBuf,
    args: Vec<String>,
    runin: PathBuf,
}

impl Action {
    
}


pub trait Actionable {
    fn execute(&self) -> Result<String, String>;
}

// impl <T: Configurable> Something<T>

// impl Actionable for Action {
    
// }

