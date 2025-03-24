use std::{
    env::{self, VarError},
    error::Error,
    fs,
    path::PathBuf,
};

use crate::core::constants::QUEST_DIR;

pub fn get_quest_dir() -> Result<PathBuf, Box<dyn Error>> {
    let mut quest_dir = env::var("HOME")
        .or_else(|_| env::var("USERPROFILE"))
        .or_else(|_| {
            let drive = env::var("HOMEDRIVE")?;
            let path = env::var("HOMEPATH")?;

            Ok(format!("{drive}{path}"))
        } as Result<String, VarError>)
        .map(PathBuf::from)?;

    quest_dir.push(QUEST_DIR);

    if !quest_dir.exists() {
        fs::create_dir_all(&quest_dir)?;
    }

    Ok(quest_dir)
}
