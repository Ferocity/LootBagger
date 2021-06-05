
use std::path::Path;

use log::*;

use crate::workspace::fields::{Workspace, Settings, FolderConvention};
use std::fs::File;
use std::io::{Write, Error, ErrorKind};

const WORKSPACE_FILE: &str = "workspace.yaml";

impl Workspace {
    pub fn create(directory: &Path) -> std::io::Result<()> {
        if !directory.is_dir() {
            return Err(Error::last_os_error())
        }

        let file = directory.join(WORKSPACE_FILE);
        if file.exists() {
            return Err(Error::new(ErrorKind::AlreadyExists, "Workspace does already exist!"))
        }

        let mut fd = File::create(file)?;

        let ws = Workspace {
            name: "Test".to_string(),
            url: "".to_string(),
            settings: Settings {
                preferred_folder_convention: FolderConvention::Name,
                update_time: 100
            }
        };
        let toml = toml::to_string(&ws).unwrap();
        fd.write_all(toml.as_bytes())?;
        Ok(())
    }

    pub fn load(filename: &Path) {
        debug!("Load {}", filename.to_str().unwrap());
    }

    pub fn unload() {
        debug!("UnLoad");
    }


}