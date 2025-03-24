use std::{
    fs,
    io::{Error, ErrorKind},
};

use crate::{
    core::constants::QUEST_DIR_LOC_ERR_MSG,
    models::vault_model::{CreateVault, Vault},
    utils::quest_dir::get_quest_dir,
};

pub struct VaultRepository;

impl VaultRepository {
    pub fn create_vault(data: CreateVault) -> Result<Vault, Error> {
        let mut vault_path = get_quest_dir().expect(QUEST_DIR_LOC_ERR_MSG);
        vault_path.push(&data.name);

        if vault_path.exists() {
            return Err(Error::new(
                ErrorKind::AlreadyExists,
                format!("Vault \"{}\" already exists!", vault_path.display()),
            ));
        }

        fs::create_dir_all(&vault_path)?;

        println!("Successfully created vault at \"{}\"", vault_path.display());

        Ok(Vault::new(data))
    }

    pub fn delete_vault(name: &str) -> Result<(), Error> {
        let mut vault_path = get_quest_dir().expect(QUEST_DIR_LOC_ERR_MSG);
        vault_path.push(name);

        if !vault_path.exists() {
            return Err(Error::new(
                ErrorKind::NotADirectory,
                format!("Vault \"{}\" doesn't exist!", vault_path.display()),
            ));
        }

        fs::remove_dir(&vault_path)?;

        println!("Successfully deleted vault at \"{}\"", vault_path.display());

        Ok(())
    }
}
