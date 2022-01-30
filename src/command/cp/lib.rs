use std::error::Error;

use crate::command::error::ArgumentError;
use crate::command::{into_directory_path, into_file_path};

pub fn copy(src: &str, dst: &str) -> Result<(), Box<dyn Error>> {
    let src = into_file_path(src)?;
    let dst = into_directory_path(dst)?;
    let path_to_write_to = dst.join(src);

    if path_to_write_to.exists() {
        let loc = path_to_write_to.to_str().unwrap();
        return Err(Box::new(ArgumentError::new(format!(
            "the path {} already exists",
            loc
        ))));
    }

    match std::fs::copy(src, path_to_write_to) {
        Ok(_) => Ok(()),
        Err(e) => Err(Box::new(e)),
    }
}
