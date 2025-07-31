use std::path::{PathBuf};
use std::env;

pub fn expand_home(path: &str) -> String {
    if let Some(stripped) = path.strip_prefix("~/") {
        if let Ok(home) = env::var("HOME") {
            return PathBuf::from(home).join(stripped).to_string_lossy().into_owned();
        }
    }
    path.to_string()
}
