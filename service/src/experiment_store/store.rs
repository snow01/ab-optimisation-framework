use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;

use anyhow::Context;
#[allow(unused_imports)]
use log::{debug, error, info, warn};
use serde::Serialize;
use walkdir::WalkDir;

use crate::settings;

pub enum Store {
    Local(LocalStore),
    S3(S3Store),
}

impl Store {
    pub fn new_local_store(local_store_config: settings::LocalStoreConfig) -> anyhow::Result<Store> {
        Ok(Store::Local(LocalStore { path: local_store_config.path }))
    }

    pub fn new_s3_store(s3_store_config: settings::S3StoreConfig) -> anyhow::Result<Store> {
        let store = S3Store::new(s3_store_config)?;
        Ok(Store::S3(store))
    }

    pub(crate) fn path(&self) -> &str {
        match self {
            Store::Local(store) => &store.path,
            Store::S3(store) => &store.path,
        }
    }

    pub(crate) fn write_data<T>(&self, data: &T, path: &str) -> anyhow::Result<()>
    where
        T: Serialize,
    {
        match self {
            Store::Local(store) => store.write_data(data, path),
            Store::S3(store) => store.write_data(data, path),
        }
    }

    pub(crate) fn visit_path<F>(&self, path: &str, visitor: F) -> anyhow::Result<()>
    where
        F: Fn(&Path, i64) -> anyhow::Result<()>,
    {
        match self {
            Store::Local(store) => store.visit_path(path, visitor),
            Store::S3(store) => store.visit_path(path, visitor),
        }
    }

    pub(crate) fn read_data<T>(&self, path: &Path) -> anyhow::Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        match self {
            Store::Local(store) => store.read_data(path),
            Store::S3(store) => store.read_data(path),
        }
    }
}

pub struct LocalStore {
    path: String,
}

impl LocalStore {
    fn write_data<T>(&self, data: &T, path: &str) -> anyhow::Result<()>
    where
        T: Serialize,
    {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);

        // Write the JSON contents to the file.
        serde_json::to_writer(&mut writer, data).with_context(|| format!("Error in writing data to file: {}", path))?;

        writer.flush()?;

        Ok(())
    }

    fn visit_path<F>(&self, path: &str, visitor: F) -> anyhow::Result<()>
    where
        F: Fn(&Path, i64) -> anyhow::Result<()>,
    {
        for entry in WalkDir::new(path).follow_links(true).into_iter().filter_map(|e| e.ok()) {
            visitor(entry.path(), 0)?;
        }

        Ok(())
    }

    fn read_data<T>(&self, path: &Path) -> anyhow::Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        serde_json::from_reader(reader).with_context(|| format!("Error in parsing data from path: {}", path.to_string_lossy()))
    }
}

pub struct S3Store {
    bucket: s3::bucket::Bucket,
    path: String,
}

impl S3Store {
    pub fn new(s3_store_config: settings::S3StoreConfig) -> anyhow::Result<S3Store> {
        use s3::bucket::Bucket;
        use s3::creds::Credentials;

        let region = s3_store_config.region.parse()?;
        let credentials = Credentials::default()?;
        let bucket = Bucket::new(&s3_store_config.bucket, region, credentials)?;

        Ok(S3Store {
            bucket,
            path: s3_store_config.path,
        })
    }

    fn write_data<T>(&self, data: &T, path: &str) -> anyhow::Result<()>
    where
        T: Serialize,
    {
        let buffer = serde_json::to_vec(data).with_context(|| format!("Error in writing data to buffer: {}", path))?;
        self.bucket
            .put_object_blocking(path, &buffer)
            .with_context(|| format!("Error in writing data to S3 file: {}", path))?;

        Ok(())
    }

    fn visit_path<F>(&self, path: &str, visitor: F) -> anyhow::Result<()>
    where
        F: Fn(&Path, i64) -> anyhow::Result<()>,
    {
        let mut consumed = false;
        let mut continuation_token = None;

        // info!("Visiting path: {:?}", path);

        while !consumed {
            let (list_results, _) = self
                .bucket
                .list_page_blocking(path.to_string(), None, continuation_token.clone(), None, Some(100))
                .with_context(|| format!("Error in listing objects in path: {}", path))?;

            // info!("Found list results: {:?}", list_results);

            for entry in list_results.contents {
                // info!("Found entry: {:?}", entry);
                let object_path = Path::new(&entry.key);
                let modified_time =
                    chrono::DateTime::parse_from_rfc3339(&entry.last_modified).with_context(|| format!("Error in parsing last_modified time"))?;

                visitor(object_path, modified_time.timestamp_millis())?;
            }

            consumed = !list_results.is_truncated;
            if !consumed {
                continuation_token = list_results.next_continuation_token;
            }
        }

        Ok(())
    }

    fn read_data<T>(&self, path: &Path) -> anyhow::Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let (buffer, _) = self
            .bucket
            .get_object_blocking(path.to_string_lossy())
            .with_context(|| format!("Error in reading data from path: {}", path.to_string_lossy()))?;

        serde_json::from_slice(&buffer).with_context(|| format!("Error in parsing data from path: {}", path.to_string_lossy()))
    }
}
