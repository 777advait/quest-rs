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
    let vault = VaultRepository::create_vault(CreateVault::new("idk".to_string()))
        .expect("Failed to create the vault!");

    println!("{vault:#?}");

    let log = Log::new(CreateLog::new(
        "My new log".to_string(),
        "some content".to_string(),
        vault,
    ));

    println!("{log:#?}");
}
