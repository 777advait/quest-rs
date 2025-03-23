use std::path::PathBuf;

use uuid::Uuid;

use crate::utils::get_home_dir::get_home_dir;

#[derive(Debug)]
pub struct Vault {
    pub id: Uuid,
    pub name: String,
    pub path: PathBuf,
}

#[derive(Debug)]
pub struct CreateVault {
    name: String,
}

impl Vault {
    pub fn new(data: CreateVault) -> Self {
        let mut path = get_home_dir().expect("Failed to locate home directory");

        path.push(&data.name);

        Self {
            id: Uuid::new_v4(),
            name: data.name,
            path,
        }
    }
}

impl CreateVault {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
