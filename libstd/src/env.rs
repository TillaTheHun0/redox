//! Enviroment data

use alloc::boxed::Box;

use fs::File;
use path::{Path, PathBuf};
use io::Result;
use string::{String, ToString};
use vec::Vec;

use system::error::{Error, ENOENT};
use system::syscall::sys_chdir;

static mut _args: *mut Vec<&'static str> = 0 as *mut Vec<&'static str>;

pub struct Args {
    i: usize
}

impl Iterator for Args {
    //Yes, this is supposed to be String, do not change it!
    //Only change it if https://doc.rust-lang.org/std/env/struct.Args.html changes from String
    type Item = String;
    fn next(&mut self) -> Option<String> {
        if let Some(arg) = unsafe { (*_args).get(self.i) } {
            self.i += 1;
            Some(arg.to_string())
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = if self.i <= unsafe { (*_args).len() } {
            unsafe { (*_args).len() - self.i }
        } else {
            0
        };
        (len, Some(len))
    }
}

impl ExactSizeIterator for Args {}

/// Arguments
pub fn args() -> Args {
    Args {
        i: 0
    }
}

/// Initialize arguments
pub unsafe fn args_init(args: Vec<&'static str>) {
    _args = Box::into_raw(box args);
}

/// Destroy arguments
pub unsafe fn args_destroy() {
    if _args as usize > 0 {
        drop(Box::from_raw(_args));
    }
}

/// Private function to get the path from a custom location
/// If the custom directory cannot be found, None will be returned
fn get_path_from(location : &str) -> Result<PathBuf> {
    match File::open(location) {
        Ok(file) => {
            match file.path() {
                Ok(path) => Ok(path),
                Err(err) => Err(err),
            }
        }
        Err(err) => Err(err),
    }
}

/// Method to return the current directory
pub fn current_dir() -> Result<PathBuf> {
    // Return the current path
    get_path_from("./")
}

/// Method to return the home directory
pub fn home_dir() -> Result<PathBuf> {
    get_path_from("/home/")
}

/// Set the current directory
pub fn set_current_dir<P: AsRef<Path>>(path: P) -> Result<()> {
    let file_result = if path.as_ref().inner.is_empty() || path.as_ref().inner.ends_with('/') {
        File::open(&path.as_ref().inner)
    } else {
        File::open(&(path.as_ref().inner.to_string() + "/"))
    };

    match file_result {
        Ok(file) => {
            match file.path() {
                Ok(path) => {
                    if let Some(path_str) = path.to_str() {
                        let path_c = path_str.to_string() + "\0";
                        unsafe {
                            sys_chdir(path_c.as_ptr()).and(Ok(()))
                        }
                    } else {
                        Err(Error::new(ENOENT))
                    }
                }
                Err(err) => Err(err),
            }
        }
        Err(err) => Err(err),
    }
}

// TODO: Fully implement `env::var()`
pub fn var(_key: &str) -> Result<String> {
    Ok("This is code filler".to_string())
}
