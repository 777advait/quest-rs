use crate::{
    core::constants::QUEST_DIR_LOC_ERR_MSG,
    utils::{clean_string::get_clean_string, quest_dir::get_quest_dir},
};
use std::path::PathBuf;

#[derive(Debug)]
pub struct Vault {
    pub name: String,
    pub path: PathBuf,
}

#[derive(Debug)]
pub struct CreateVault {
    pub name: String,
}

impl Vault {
    pub fn new(data: CreateVault) -> Self {
        let mut path = get_quest_dir().expect(QUEST_DIR_LOC_ERR_MSG);
        let name = get_clean_string(&data.name).expect("Vault name cannot be empty");
        path.push(&name);

        Self { name, path }
    }
}

impl CreateVault {
    pub fn new(name: String) -> Self {
        Self {
            name: get_clean_string(&name).expect("Vault name cannot be empty"),
        }
    }
}
