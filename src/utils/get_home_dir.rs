use std::{env, path::PathBuf};

pub fn get_home_dir() -> Result<PathBuf, env::VarError> {
    env::var("HOME") // for MacOS/Linux
        // For Windows
        .or_else(|_| env::var("USERPROFILE"))
        .or_else(|_| {
            let drive = env::var("HOMEDRIVE")?;
            let path = env::var("HOMEPATH")?;

            Ok(format!("{drive}{path}"))
        })
        .map(PathBuf::from)
}
