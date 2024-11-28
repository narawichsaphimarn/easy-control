use crate::shared::types::file_store_type::{ScreenMappingMatrix, ScreenSelector};
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;
use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Stores {
    pub screen_mapping_matrix: Vec<ScreenMappingMatrix>,
    pub screen_selector: Vec<ScreenSelector>,
}

static NANE: LazyLock<String> = LazyLock::new(|| String::from("share-mouse.conf.json"));

impl Stores {
    pub async fn init() -> Stores {
        Self::handle_new_file().await
    }

    fn init_new() -> Stores {
        let json_data = r#"
            {
                "screen_mapping_matrix": [],
                "screen_selector": []
            }
            "#;

        serde_json::from_str(json_data).unwrap()
    }

    async fn handle_new_file() -> Stores {
        let file = Self::get_file().await;
        match file {
            Ok(mut file) => {
                let mut contents = String::new();
                let data = file
                    .read_to_string(&mut contents)
                    .await
                    .map_err(|e| e.to_string());
                match data {
                    Ok(_) => {
                        let file: Stores = serde_json::from_str(&contents).unwrap();
                        file
                    }
                    Err(e) => panic!("Error: {}", e),
                }
            }
            Err(_) => {
                let file = File::create(NANE.clone()).await.map_err(|e| e.to_string());
                match file {
                    Ok(file) => {
                        let file_store: Stores = Self::init_new();
                        Self::write_file_store(file, file_store.clone()).await;
                        file_store
                    }
                    Err(e) => panic!("Error: {}", e),
                }
            }
        }
    }

    pub async fn write_file_store(mut file: File, data: Stores) {
        let data = serde_json::to_string(&data).unwrap();
        let result = file
            .write_all(data.as_bytes())
            .await
            .map_err(|e| e.to_string());
        match result {
            Ok(_) => {}
            Err(e) => panic!("Error: {}", e),
        }
    }

    pub async fn get_file() -> Result<File, String> {
        File::open(NANE.clone()).await.map_err(|e| e.to_string())
    }

    pub async fn read_file() -> Stores {
        let result_file = Self::get_file().await;
        match result_file {
            Ok(mut file) => {
                let mut contents = String::new();
                let data = file
                    .read_to_string(&mut contents)
                    .await
                    .map_err(|e| e.to_string());
                match data {
                    Ok(_) => {
                        let file: Stores = serde_json::from_str(&contents).unwrap();
                        file
                    }
                    Err(e) => {
                        panic!("{}", e);
                    }
                }
            }
            Err(e) => {
                panic!("{:?}", e);
            }
        }
    }

    pub async fn write_file(data: Stores) -> Result<(), String> {
        let mut file = OpenOptions::new()
            .write(true) // Open for writing
            .truncate(true) // Truncate the file to 0 bytes
            .open(NANE.clone())
            .await
            .map_err(|e| e.to_string())?;
        let data = serde_json::to_string(&data).unwrap();
        let result = file
            .write_all(data.as_bytes())
            .await
            .map_err(|e| e.to_string());
        let _ = file.flush().await;
        match result {
            Ok(_) => Ok(()),
            Err(e) => panic!("Error: {}", e),
        }
    }
}
