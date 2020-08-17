use std::path::{Path,PathBuf};
use std::io::{self, Read};
use std::ffi;
use std::fs;


#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    FileContainsNil,
    FailedToGetExePath,
}

impl From<io::Error> for Error {
    fn from(other: io::Error) -> Self {
        Error::Io(other)
    }
}

pub struct ResourceLoader {
    root_path: PathBuf,
}

impl ResourceLoader {
    pub fn from_relative_exe_path(rel_path: &Path) -> Result<ResourceLoader, Error> {
        let exe_file_name = std::env::current_exe().map_err(|_| Error::FailedToGetExePath)?;
        let exe_path = exe_file_name.parent().ok_or(Error::FailedToGetExePath)?;

        Ok(ResourceLoader{
            root_path: exe_path.join(rel_path),
        })
    }

    pub fn load_cstring(&self, resource_name: &str) -> Result<ffi::CString, Error> {
        let mut file = fs::File::open(resource_name_to_path(&self.root_path, resource_name))?;
        
        let mut buffer: Vec<u8> = Vec::with_capacity(file.metadata()?.len() as usize + 1);
        file.read_to_end(&mut buffer)?;

        // Check for nul byte
        if buffer.iter().find(|i| **i == 0).is_some() {
            return Err(Error::FileContainsNil);
        }

        Ok(unsafe { ffi::CString::from_vec_unchecked(buffer) })
    }

    pub fn load_string(&self, resource_name: &str) -> Result<String, Error> {
        match fs::read_to_string(resource_name_to_path(&self.root_path, resource_name)) {
            Ok(s) => Ok(s),
            Err(e) => Err(Error::Io(e))
        }
    }

    pub fn resolve_path(&self, resource_name: &str) -> Result<PathBuf, Error> {
        Ok(resource_name_to_path(&self.root_path, resource_name))
    }
}

fn resource_name_to_path(root_dir: &Path, location: &str) -> PathBuf {
    location
        .split("/")
        .fold(root_dir.into(), |prev, part| prev.join(part))
}
