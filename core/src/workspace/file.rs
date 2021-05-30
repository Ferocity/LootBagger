
use std::path::Path;

use log::*;

use crate::workspace::fields::Workspace;


impl Workspace {
    pub fn create(filename: &Path) {
        debug!("Create {}", filename.to_str().unwrap());
    }

    pub fn load(filename: &Path) {
        debug!("Load {}", filename.to_str().unwrap());
    }

    pub fn unload() {
        debug!("UnLoad");
    }


}