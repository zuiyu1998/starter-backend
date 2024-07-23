pub mod error;

pub use error::*;

use std::{path::PathBuf, sync::Arc};

use abi::{async_trait::async_trait, config::Config, tokio::fs, tracing};
use serde_json::{Map, Value};

pub struct FileStorage {
    pub file: PathBuf,
}

impl FileStorage {
    pub async fn from_config(config: &Config) -> Result<Self> {
        //创建文件夹
        fs::create_dir_all(config.system.get_config_dir())
            .await
            .expect("config dir create fail.");

        //创建文件
        let config_path = config.system.get_config_path();
        if !fs::try_exists(&config_path).await? {
            fs::File::create(&config_path).await?;
        }

        Ok(FileStorage { file: config_path })
    }
}

pub async fn load_path(path: &PathBuf) -> Value {
    let json_str = fs::read_to_string(path).await.unwrap_or("".to_string());

    let value = serde_json::from_str(&json_str).unwrap_or(Value::Object(Map::default()));

    value
}

pub async fn save_path(path: &PathBuf, value: Value) {
    let json_str = serde_json::to_string(&value).unwrap_or("".to_string());

    if let Err(e) = fs::write(path, json_str.as_bytes()).await {
        tracing::error!("save failed: {}", e);
    }
}

#[async_trait]
impl Storage for FileStorage {
    async fn set_item(&self, key: &str, value: &str) {
        let mut object = load_path(&self.file).await;

        if let Some(obj) = object.as_object_mut() {
            obj.insert(key.to_string(), Value::String(value.to_string()));
        }
    }

    async fn get_item(&self, key: &str) -> Option<String> {
        let object = load_path(&self.file).await;
        object.as_object().and_then(|object| {
            object
                .get(key)
                .and_then(|value| value.as_str().map(|str| str.to_string()))
        })
    }

    async fn remove_item(&self, key: &str) -> Option<String> {
        let mut object = load_path(&self.file).await;

        let res;

        if let Some(obj) = object.as_object_mut() {
            res = obj
                .remove(key)
                .and_then(|value| value.as_str().map(|str| str.to_string()));
        } else {
            res = None;
        }

        save_path(&self.file, object).await;

        res
    }

    async fn clear(&self) {
        let value = Value::Object(Map::default());

        save_path(&self.file, value).await;
    }
}

#[async_trait]
pub trait Storage: 'static + Send + Sync {
    async fn set_item(&self, key: &str, value: &str);

    async fn get_item(&self, key: &str) -> Option<String>;

    async fn remove_item(&self, key: &str) -> Option<String>;

    async fn clear(&self);
}

#[derive(Clone)]
pub struct Persistent {
    #[allow(dead_code)]
    storage: Arc<dyn Storage>,
}

impl Persistent {
    pub async fn from_config(config: &Config) -> Result<Self> {
        let storage = FileStorage::from_config(config).await?;

        Ok(Persistent {
            storage: Arc::new(storage),
        })
    }
}
