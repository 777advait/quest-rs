use std::{fs, io::Error};

use crate::{
    models::vault_model::{CreateVault, Vault},
    utils::quest_dir::get_quest_dir,
};

pub struct VaultRepository;

impl VaultRepository {
    pub fn create_vault(data: CreateVault) -> Result<Vault, Error> {
        let mut vault_path = get_quest_dir().expect("Failed to locate `.quest` directory");
        vault_path.push(&data.name);

        if vault_path.exists() {
            panic!("Vault \"{}\" already exists", vault_path.display());
        }

        fs::create_dir_all(&vault_path)?;

        println!("Successfully created vault at \"{}\"", vault_path.display());

        Ok(Vault::new(data))
    }

    // pub fn delete_vault()
}
