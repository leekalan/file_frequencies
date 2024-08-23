use std::{
    borrow::Borrow,
    env,
    path::{Path, PathBuf},
};

pub fn boot() {
    env::set_current_dir(dirs::home_dir().expect("Unable to get home directory"))
        .expect("Unable to set home directory");
}

pub struct CanonicalPath {
    path: PathBuf,
}
impl CanonicalPath {
    pub fn new(path: PathBuf) -> Option<Self> {
        if let Ok(canonicalized) = path.canonicalize() {
            Some(Self {
                path: canonicalized,
            })
        } else {
            None
        }
    }
    pub unsafe fn new_unchecked(path: PathBuf) -> Self {
        Self { path }
    }
    pub fn view(&self) -> &Path {
        &self.path
    }
}
impl Borrow<Path> for CanonicalPath {
    fn borrow(&self) -> &Path {
        self.view()
    }
}

pub fn truncate_path(path: &CanonicalPath) -> String {
    let temp = path.view().as_os_str().to_string_lossy();
    let stringified = temp.trim_start_matches(['\\', '?']);

    if stringified.len() >= 30 {
        let trimmed =
            unsafe { stringified.get_unchecked(stringified.len() - 30..stringified.len()) };
        format!("...{}", trimmed)
    } else {
        stringified.to_string()
    }
}
