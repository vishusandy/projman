
use super::*;

pub const DEFAULT_LANGUAGE: Language = Language::Text;
pub const DEFAULT_VCS: VersionControl = VersionControl::Git;
pub const DEFAULT_VERSION_INC: VersionInc = VersionInc::Major;
pub const INSTALL_FILENAME: &'static str = "proman_install.cfg";
pub const USER_FILENAME: &'static str = "proman_user.cfg";
pub const PROJECT_FILENAME: &'static str = "proman_project.cfg";
pub const PROJECT_BEHAVIOR: &'static str = "proman_behavior.cfg"; // Maybe details instead of behavior??
pub const CONFIG_RECURSE_DEPTH: u8 = 4;  // Number of parent directories to check for config file


