pub mod file_store {
    use crate::shared::types::file_store_type::{
        ScreenMappingMatrix, ScreenMappingRefer, ScreenSelector, Setting,
    };
    use serde::{Deserialize, Serialize};
    use tokio::fs::File;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    #[derive(Debug, Clone, Default, Deserialize, Serialize)]
    pub struct FileStore {
        pub screen_mapping_matrix: Vec<ScreenMappingMatrix>,
        pub screen_selector: Vec<ScreenSelector>,
        pub setting: Vec<Setting>,
        pub screen_mapping_refer: Vec<ScreenMappingRefer>,
    }

    impl FileStore {
        pub async fn init() -> FileStore {
            Self::handle_new_file(String::from("share-mouse.conf.json")).await
        }

        fn new() -> FileStore {
            let json_data = r#"
            {
                "screen_mapping_matrix": [],
                "screen_selector": [],
                "setting": [
                    {
                        "id" : 1,
                        "parameter_key" : "NETWORK_ROLE",
                        "parameter_group" : "NETWORK",
                        "parameter_value" : "CLIENT"
                    }
                ],
                "screen_mapping_refer": [
                    {
                        "id" : 1,
                        "parameter_key" : "SCREEN_NUMBER",
                        "parameter_group" : "1",
                        "parameter_value" : "2,4"
                    },
                    {
                        "id" : 2,
                        "parameter_key" : "SCREEN_NUMBER",
                        "parameter_group" : "2",
                        "parameter_value" : "1,3,5"
                    },
                    {
                        "id" : 3,
                        "parameter_key" : "SCREEN_NUMBER",
                        "parameter_group" : "3",
                        "parameter_value" : "2,6"
                    },
                    {
                        "id" : 4,
                        "parameter_key" : "SCREEN_NUMBER",
                        "parameter_group" : "4",
                        "parameter_value" : "1,5,7"
                    },
                    {
                        "id" : 5,
                        "parameter_key" : "SCREEN_NUMBER",
                        "parameter_group" : "5",
                        "parameter_value" : "2,4,6,8"
                    },
                    {
                        "id" : 6,
                        "parameter_key" : "SCREEN_NUMBER",
                        "parameter_group" : "6",
                        "parameter_value" : "3,5,9"
                    },
                    {
                        "id" : 7,
                        "parameter_key" : "SCREEN_NUMBER",
                        "parameter_group" : "7",
                        "parameter_value" : "4,8"
                    },
                    {
                        "id" : 8,
                        "parameter_key" : "SCREEN_NUMBER",
                        "parameter_group" : "8",
                        "parameter_value" : "7,5,9"
                    },
                    {
                        "id" : 9,
                        "parameter_key" : "SCREEN_NUMBER",
                        "parameter_group" : "9",
                        "parameter_value" : "6,8"
                    },
                    {
                        "id" : 10,
                        "parameter_key" : "SCREEN_NUMBER",
                        "parameter_group" : "1,2",
                        "parameter_value" : "RIGHT"
                    },
                    {
                        "id" : 11,
                        "parameter_key" : "SCREEN_NUMBER",
                        "parameter_group" : "1,4",
                        "parameter_value" : "BOTTOM"
                    },
                    {
                        "id" : 12,
                        "parameter_key" : "SCREEN_NUMBER",
                        "parameter_group" : "2,1",
                        "parameter_value" : "LEFT"
                    },
                    {
                        "id" : 13,
                        "parameter_key" : "SCREEN_NUMBER",
                        "parameter_group" : "2,3",
                        "parameter_value" : "RIGHT"
                    },
                    {
                        "id" : 14,
                        "parameter_key" : "SCREEN_NUMBER",
                        "parameter_group" : "2,5",
                        "parameter_value" : "BOTTOM"
                    },
                    {
                        "id" : 15,
                        "parameter_key" : "SCREEN_NUMBER",
                        "parameter_group" : "3,2",
                        "parameter_value" : "LEFT"
                    },
                    {
                        "id" : 16,
                        "parameter_key" : "SCREEN_NUMBER",
                        "parameter_group" : "3,6",
                        "parameter_value" : "BOTTOM"
                    },
                    {
                        "id" : 17,
                        "parameter_key" : "SCREEN_NUMBER",
                        "parameter_group" : "4,1",
                        "parameter_value" : "TOP"
                    },
                    {
                        "id" : 18,
                        "parameter_key" : "SCREEN_NUMBER",
                        "parameter_group" : "4,5",
                        "parameter_value" : "RIGHT"
                    },
                    {
                        "id" : 19,
                        "parameter_key" : "SCREEN_NUMBER",
                        "parameter_group" : "4,7",
                        "parameter_value" : "BOTTOM"
                    },
                    {
                        "id" : 20,
                        "parameter_key" : "SCREEN_NUMBER",
                        "parameter_group" : "5,2",
                        "parameter_value" : "TOP"
                    },
                    {
                        "id" : 21,
                        "parameter_key" : "SCREEN_NUMBER",
                        "parameter_group" : "5,4",
                        "parameter_value" : "LEFT"
                    },
                    {
                        "id" : 22,
                        "parameter_key" : "SCREEN_NUMBER",
                        "parameter_group" : "5,6",
                        "parameter_value" : "RIGHT"
                    },
                    {
                        "id" : 23,
                        "parameter_key" : "SCREEN_NUMBER",
                        "parameter_group" : "5,8",
                        "parameter_value" : "BOTTOM"
                    },
                    {
                        "id" : 24,
                        "parameter_key" : "SCREEN_NUMBER",
                        "parameter_group" : "6,3",
                        "parameter_value" : "TOP"
                    },
                    {
                        "id" : 25,
                        "parameter_key" : "SCREEN_NUMBER",
                        "parameter_group" : "6,5",
                        "parameter_value" : "LEFT"
                    },
                    {
                        "id" : 26,
                        "parameter_key" : "SCREEN_NUMBER",
                        "parameter_group" : "6,9",
                        "parameter_value" : "BOTTOM"
                    },
                    {
                        "id" : 27,
                        "parameter_key" : "SCREEN_NUMBER",
                        "parameter_group" : "7,4",
                        "parameter_value" : "TOP"
                    },
                    {
                        "id" : 28,
                        "parameter_key" : "SCREEN_NUMBER",
                        "parameter_group" : "7,8",
                        "parameter_value" : "RIGHT"
                    },
                    {
                        "id" : 29,
                        "parameter_key" : "SCREEN_NUMBER",
                        "parameter_group" : "8,5",
                        "parameter_value" : "TOP"
                    },
                    {
                        "id" : 30,
                        "parameter_key" : "SCREEN_NUMBER",
                        "parameter_group" : "8,7",
                        "parameter_value" : "LEFT"
                    },
                    {
                        "id" : 31,
                        "parameter_key" : "SCREEN_NUMBER",
                        "parameter_group" : "8,9",
                        "parameter_value" : "RIGHT"
                    },
                    {
                        "id" : 32,
                        "parameter_key" : "SCREEN_NUMBER",
                        "parameter_group" : "9,6",
                        "parameter_value" : "TOP"
                    },
                    {
                        "id" : 33,
                        "parameter_key" : "SCREEN_NUMBER",
                        "parameter_group" : "9,8",
                        "parameter_value" : "LEFT"
                    }
                ]
            }
            "#;

            let root: FileStore = serde_json::from_str(json_data).unwrap();
            root
        }

        async fn handle_new_file(file_name: String) -> FileStore {
            let file = Self::get_file(file_name.clone()).await;
            match file {
                Ok(mut file) => {
                    let mut contents = String::new();
                    let data = file
                        .read_to_string(&mut contents)
                        .await
                        .map_err(|e| e.to_string());
                    match data {
                        Ok(_) => {
                            let file: FileStore = serde_json::from_str(&contents).unwrap();
                            file
                        }
                        Err(e) => panic!("Error: {}", e),
                    }
                }
                Err(_) => {
                    let file = File::create(file_name).await.map_err(|e| e.to_string());
                    match file {
                        Ok(file) => {
                            let file_store: FileStore = FileStore::new();
                            Self::write_file_store(file, file_store.clone()).await;
                            file_store
                        }
                        Err(e) => panic!("Error: {}", e),
                    }
                }
            }
        }

        pub async fn get_file(file_name: String) -> Result<File, String> {
            File::open(file_name.clone())
                .await
                .map_err(|e| e.to_string())
        }

        pub async fn write_file_store(mut file: File, data: FileStore) {
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

        pub async fn write_file(data: FileStore) -> Result<(), String> {
            let mut file = Self::get_file(String::from("share-mouse.conf.json"))
                .await
                .map_err(|e| e.to_string())?;
            let data = serde_json::to_string(&data).unwrap();
            let result = file
                .write_all(data.as_bytes())
                .await
                .map_err(|e| e.to_string());
            match result {
                Ok(_) => Ok(()),
                Err(e) => panic!("Error: {}", e),
            }
        }
    }
}
