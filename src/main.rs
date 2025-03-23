use models::{
    log_model::{CreateLog, Log},
    vault_model::{CreateVault, Vault},
};

mod models;
mod repository;
mod utils;

fn main() {
    let vault = Vault::new(CreateVault::new("portfolio thoughts".to_string()));

    println!("{vault:#?}");

    let log = Log::new(CreateLog::new(
        "My new log".to_string(),
        "some content".to_string(),
        vault,
    ));

    println!("{log:#?}");
}
