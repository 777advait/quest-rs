use std::io::ErrorKind;

use models::{
    log_model::{CreateLog, Log},
    vault_model::CreateVault,
};
use repository::vault_repository::VaultRepository;

mod core;
mod models;
mod repository;
mod utils;

fn main() {
    let vault_name = "idk";
    let vault = VaultRepository::create_vault(CreateVault::new(vault_name.to_string()))
        .or_else(|err| {
            if err.kind() == ErrorKind::AlreadyExists {
                println!("Vault \"{vault_name}\" already exists... Deleting and creating new one!");

                VaultRepository::delete_vault(&vault_name)
                    .expect(&format!("Failed to delete \"{vault_name}\""));

                VaultRepository::create_vault(CreateVault::new(vault_name.to_string()))
            } else {
                Err(err)
            }
        })
        .expect("Failed to create/delete vault");

    println!("{vault:#?}");

    let log = Log::new(CreateLog::new(
        "My new log".to_string(),
        "some content".to_string(),
        vault,
    ));

    println!("{log:#?}");
}
