use async_trait::async_trait;
use bson::to_document;
use bson::Document;
use debot_utils::HasId;
use futures::stream::TryStreamExt;
use mongodb::bson::doc;
use mongodb::options::*;
use mongodb::Database;
use mongodb::{Collection, IndexModel};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::error;
use std::io::{Error, ErrorKind};

use crate::PositionLog;

use super::AppState;
use super::PnlLog;
use super::PriceLog;

pub enum SearchMode {
    Ascending,
    Descending,
    ById,
}

#[async_trait]
pub trait Entity {
    async fn insert(&self, db: &Database) -> Result<(), Box<dyn error::Error>>;
    async fn update(&self, db: &Database) -> Result<(), Box<dyn error::Error>>;
    async fn delete(&self, db: &Database) -> Result<(), Box<dyn error::Error>>;
    async fn delete_all(&self, db: &Database) -> Result<(), Box<dyn error::Error>>;

    async fn search(
        &self,
        db: &Database,
        mode: SearchMode,
        limit: Option<u32>,
        id: Option<u32>,
        sort_key: Option<&str>,
    ) -> Result<Vec<Self>, Box<dyn error::Error>>
    where
        Self: std::marker::Sized;

    fn get_collection_name(&self) -> &str;

    fn get_collection(&self, db: &Database) -> Collection<Self>
    where
        Self: std::marker::Sized,
    {
        db.collection::<Self>(self.get_collection_name())
    }

    async fn create_indexes(&self, db: &Database) -> Result<(), Box<dyn error::Error>>
    where
        Self: std::marker::Sized,
        Self: std::marker::Send;
}

pub async fn insert_item<T: Entity>(db: &Database, item: &T) -> Result<(), Box<dyn error::Error>> {
    item.insert(db).await
}

pub async fn update_item<T: Entity>(db: &Database, item: &T) -> Result<(), Box<dyn error::Error>> {
    item.update(db).await
}

#[allow(dead_code)]
pub async fn delete_item<T: Entity>(db: &Database, item: &T) -> Result<(), Box<dyn error::Error>> {
    item.delete(db).await
}

#[allow(dead_code)]
pub async fn delete_item_all<T: Entity>(
    db: &Database,
    item: &T,
) -> Result<(), Box<dyn error::Error>> {
    item.delete_all(db).await
}

pub async fn search_items<T: Entity>(
    db: &Database,
    item: &T,
    mode: SearchMode,
    limit: Option<u32>,
    id: Option<u32>,
    sort_key: Option<&str>,
) -> Result<Vec<T>, Box<dyn error::Error>> {
    item.search(db, mode, limit, id, sort_key).await
}

pub async fn search_item<T: Entity>(
    db: &Database,
    item: &T,
    id: Option<u32>,
    sort_key: Option<&str>,
) -> Result<T, Box<dyn error::Error>> {
    let mut items = item
        .search(db, SearchMode::ById, None, id, sort_key)
        .await?;
    if items.len() == 1 {
        Ok(items.pop().unwrap())
    } else {
        Err(Box::new(Error::new(
            ErrorKind::Other,
            "Multiple items are found".to_string(),
        )))
    }
}

async fn get_existing_indexes<T>(
    collection: &Collection<T>,
) -> Result<Vec<String>, Box<dyn error::Error>> {
    let mut indexes = collection.list_indexes(None).await?;
    let mut index_names = Vec::new();

    while let Some(index) = indexes.try_next().await? {
        let document: Document = to_document(&index)?;
        if let Some(name) = document.get_str("name").ok() {
            index_names.push(name.to_string());
        }
    }

    Ok(index_names)
}
pub async fn create_unique_index(db: &Database) -> Result<(), Box<dyn error::Error>> {
    async fn create_index<T: Entity>(
        db: &Database,
        entity: &T,
    ) -> Result<(), Box<dyn error::Error>> {
        let collection = entity.get_collection(db);
        let existing_indexes = get_existing_indexes(&collection).await?;

        let indexes = vec![
            (
                "id_1",
                IndexModel::builder()
                    .keys(doc! {"id": 1})
                    .options(IndexOptions::builder().unique(true).build())
                    .build(),
            ),
            (
                "open_timestamp_1",
                IndexModel::builder()
                    .keys(doc! {"open_timestamp": 1})
                    .build(),
            ),
            (
                "open_timestamp_-1",
                IndexModel::builder()
                    .keys(doc! {"open_timestamp": -1})
                    .build(),
            ),
            (
                "price_point.timestamp_1",
                IndexModel::builder()
                    .keys(doc! {"price_point.timestamp": 1})
                    .build(),
            ),
            (
                "price_point.timestamp_-1",
                IndexModel::builder()
                    .keys(doc! {"price_point.timestamp": -1})
                    .build(),
            ),
        ];

        for (index_name, index_model) in indexes {
            if !existing_indexes.contains(&index_name.to_string()) {
                log::info!("Creating index `{}`...", index_name);
                collection.create_index(index_model, None).await?;
                log::info!("Index `{}` has been created successfully!", index_name);
            } else {
                log::info!("Index `{}` already exists, skipping.", index_name);
            }
        }

        Ok(())
    }

    create_index(db, &PositionLog::default()).await?;
    create_index(db, &AppState::default()).await?;
    create_index(db, &PriceLog::default()).await?;
    create_index(db, &PnlLog::default()).await?;

    Ok(())
}

#[async_trait]
impl Entity for PositionLog {
    async fn create_indexes(&self, db: &Database) -> Result<(), Box<dyn error::Error>> {
        let collection = self.get_collection(db);

        let id_index = IndexModel::builder()
            .keys(doc! {"id": 1})
            .options(IndexOptions::builder().unique(true).build())
            .build();

        let open_timestamp_index = IndexModel::builder()
            .keys(doc! {"open_timestamp": 1})
            .build();

        let open_timestamp_index_2 = IndexModel::builder()
            .keys(doc! {"open_timestamp": -1})
            .build();

        collection.create_index(id_index, None).await?;
        collection.create_index(open_timestamp_index, None).await?;
        collection
            .create_index(open_timestamp_index_2, None)
            .await?;

        Ok(())
    }

    async fn insert(&self, db: &Database) -> Result<(), Box<dyn error::Error>> {
        let collection = self.get_collection(db);
        collection.insert_one(self, None).await?;
        Ok(())
    }

    async fn update(&self, db: &Database) -> Result<(), Box<dyn error::Error>> {
        let query = doc! { "id": self.id() };
        let update = bson::to_bson(self).unwrap();
        let update = doc! { "$set" : update };
        let collection = self.get_collection(db);
        collection.update(query, update, true).await
    }

    async fn delete(&self, _db: &Database) -> Result<(), Box<dyn error::Error>> {
        panic!("Not implemented")
    }

    async fn delete_all(&self, db: &Database) -> Result<(), Box<dyn error::Error>> {
        let collection = self.get_collection(db);
        collection.delete_all().await
    }

    async fn search(
        &self,
        db: &Database,
        mode: SearchMode,
        limit: Option<u32>,
        id: Option<u32>,
        sort_key: Option<&str>,
    ) -> Result<Vec<Self>, Box<dyn error::Error>> {
        let mut query = doc! { "id": { "$gt": 0 }};
        if self.id() != None {
            query = doc! { "id": self.id().unwrap() };
        }
        let collection = self.get_collection(db);
        let sort_key = sort_key.unwrap_or("id");
        collection.search(query, mode, limit, id, &sort_key).await
    }

    fn get_collection_name(&self) -> &str {
        "position"
    }
}

#[async_trait]
impl Entity for PnlLog {
    async fn create_indexes(&self, db: &Database) -> Result<(), Box<dyn error::Error>> {
        let collection = self.get_collection(db);

        let id_index = IndexModel::builder()
            .keys(doc! {"id": 1})
            .options(IndexOptions::builder().unique(true).build())
            .build();

        collection.create_index(id_index, None).await?;

        Ok(())
    }

    async fn insert(&self, db: &Database) -> Result<(), Box<dyn error::Error>> {
        let collection = self.get_collection(db);
        collection.insert_one(self, None).await?;
        Ok(())
    }

    async fn update(&self, _db: &Database) -> Result<(), Box<dyn error::Error>> {
        panic!("Not implemented")
    }

    async fn delete(&self, _db: &Database) -> Result<(), Box<dyn error::Error>> {
        panic!("Not implemented")
    }

    async fn delete_all(&self, _db: &Database) -> Result<(), Box<dyn error::Error>> {
        panic!("Not implemented")
    }

    async fn search(
        &self,
        db: &Database,
        mode: SearchMode,
        limit: Option<u32>,
        id: Option<u32>,
        sort_key: Option<&str>,
    ) -> Result<Vec<Self>, Box<dyn error::Error>> {
        let mut query = doc! { "id": { "$gt": 0 }};
        if self.id != None {
            query = doc! { "id": self.id.unwrap() };
        }
        let collection = self.get_collection(db);
        let sort_key = sort_key.unwrap_or("id");
        collection.search(query, mode, limit, id, &sort_key).await
    }

    fn get_collection_name(&self) -> &str {
        "balance"
    }
}

#[async_trait]
impl Entity for AppState {
    async fn create_indexes(&self, db: &Database) -> Result<(), Box<dyn error::Error>> {
        let collection = self.get_collection(db);

        let id_index = IndexModel::builder()
            .keys(doc! {"id": 1})
            .options(IndexOptions::builder().unique(true).build())
            .build();

        collection.create_index(id_index, None).await?;

        Ok(())
    }

    async fn insert(&self, _db: &Database) -> Result<(), Box<dyn error::Error>> {
        panic!("Not implemented")
    }

    async fn update(&self, db: &Database) -> Result<(), Box<dyn error::Error>> {
        let query = doc! { "id": 1 };
        let update = bson::to_bson(self).unwrap();
        let update = doc! { "$set" : update };
        let collection = self.get_collection(db);
        collection.update(query, update, true).await
    }

    async fn delete(&self, _db: &Database) -> Result<(), Box<dyn error::Error>> {
        panic!("Not implemented")
    }

    async fn delete_all(&self, db: &Database) -> Result<(), Box<dyn error::Error>> {
        let collection = self.get_collection(db);
        collection.delete_all().await
    }

    async fn search(
        &self,
        db: &Database,
        mode: SearchMode,
        limit: Option<u32>,
        id: Option<u32>,
        sort_key: Option<&str>,
    ) -> Result<Vec<Self>, Box<dyn error::Error>> {
        let query = doc! { "id": 1 };
        let collection = self.get_collection(db);
        let sort_key = sort_key.unwrap_or("id");
        collection.search(query, mode, limit, id, &sort_key).await
    }

    fn get_collection_name(&self) -> &str {
        "app-state"
    }
}

#[async_trait]
impl Entity for PriceLog {
    async fn create_indexes(&self, db: &Database) -> Result<(), Box<dyn error::Error>> {
        let collection = self.get_collection(db);

        let id_index = IndexModel::builder()
            .keys(doc! {"id": 1})
            .options(IndexOptions::builder().unique(true).build())
            .build();

        let price_point_timestamp_index = IndexModel::builder()
            .keys(doc! {"price_point.timestamp": 1})
            .build();

        let price_point_timestamp_index_2 = IndexModel::builder()
            .keys(doc! {"price_point.timestamp": -1})
            .build();

        collection.create_index(id_index, None).await?;
        collection
            .create_index(price_point_timestamp_index, None)
            .await?;
        collection
            .create_index(price_point_timestamp_index_2, None)
            .await?;

        Ok(())
    }

    async fn insert(&self, db: &Database) -> Result<(), Box<dyn error::Error>> {
        let collection = self.get_collection(db);
        collection.insert_one(self, None).await?;
        Ok(())
    }

    async fn update(&self, db: &Database) -> Result<(), Box<dyn error::Error>> {
        let query = doc! { "id": self.id };
        let update = bson::to_bson(self).unwrap();
        let update = doc! { "$set" : update };
        let collection = self.get_collection(db);
        collection.update(query, update, true).await
    }

    async fn delete(&self, _db: &Database) -> Result<(), Box<dyn error::Error>> {
        panic!("Not implemented")
    }

    async fn delete_all(&self, _db: &Database) -> Result<(), Box<dyn error::Error>> {
        panic!("Not implemented")
    }

    async fn search(
        &self,
        db: &Database,
        mode: SearchMode,
        limit: Option<u32>,
        id: Option<u32>,
        sort_key: Option<&str>,
    ) -> Result<Vec<Self>, Box<dyn error::Error>> {
        let mut query = doc! { "id": { "$gt": 0 }};
        if self.id != None {
            query = doc! { "id": self.id.unwrap() };
        }
        let collection = self.get_collection(db);
        let sort_key = sort_key.unwrap_or("id");
        collection.search(query, mode, limit, id, &sort_key).await
    }

    fn get_collection_name(&self) -> &str {
        "price"
    }
}

#[async_trait]
pub trait HelperCollection<T> {
    async fn update(
        &self,
        query: Document,
        update: Document,
        upsert: bool,
    ) -> Result<(), Box<dyn error::Error>>;
    async fn delete(&self, query: Document) -> Result<(), Box<dyn error::Error>>;
    async fn delete_all(&self) -> Result<(), Box<dyn error::Error>>;
    async fn search(
        &self,
        query: Document,
        mode: SearchMode,
        limit: Option<u32>,
        id: Option<u32>,
        sort_key: &str,
    ) -> Result<Vec<T>, Box<dyn error::Error>>;
}

#[async_trait]
impl<T> HelperCollection<T> for Collection<T>
where
    T: DeserializeOwned + Unpin + Send + Sync + Serialize + std::fmt::Debug,
{
    async fn update(
        &self,
        query: Document,
        update: Document,
        upsert: bool,
    ) -> Result<(), Box<dyn error::Error>> {
        let options = FindOneAndUpdateOptions::builder()
            .upsert(upsert)
            .return_document(ReturnDocument::After)
            .build();
        let _ = self.find_one_and_update(query, update, options).await?;
        Ok(())
    }

    async fn delete(&self, query: Document) -> Result<(), Box<dyn error::Error>> {
        let result = self.delete_one(query, None).await?;
        if result.deleted_count == 1 {
            return Ok(());
        } else {
            panic!("Not implemented")
        }
    }

    async fn delete_all(&self) -> Result<(), Box<dyn error::Error>> {
        let options = DropCollectionOptions::builder().build();
        self.drop(options).await?;
        Ok(())
    }

    async fn search(
        &self,
        mut query: Document,
        mode: SearchMode,
        limit: Option<u32>,
        id: Option<u32>,
        sort_key: &str,
    ) -> Result<Vec<T>, Box<dyn error::Error>> {
        let mut items: Vec<T> = vec![];

        match sort_key {
            "id" | "open_timestamp" | "price_point.timestamp" => {}
            _ => {
                return Err(Box::new(Error::new(
                    ErrorKind::InvalidInput,
                    "Invalid sort key",
                )))
            }
        };

        let find_options = match mode {
            SearchMode::Ascending => {
                let builder = FindOptions::builder()
                    .allow_disk_use(Some(true))
                    .sort(doc! { sort_key: 1 });

                if let Some(limit_value) = limit {
                    builder.limit(limit_value as i64).build()
                } else {
                    builder.build()
                }
            }
            SearchMode::Descending => {
                let builder = FindOptions::builder()
                    .allow_disk_use(Some(true))
                    .sort(doc! { sort_key: -1 });

                if let Some(limit_value) = limit {
                    builder.limit(limit_value as i64).build()
                } else {
                    builder.build()
                }
            }
            SearchMode::ById => {
                if let Some(id_value) = id {
                    query.insert("id", id_value);
                } else {
                    return Err(Box::new(Error::new(
                        ErrorKind::InvalidInput,
                        "ID not provided".to_string(),
                    )));
                }
                FindOptions::builder().allow_disk_use(Some(true)).build()
            }
        };

        let mut cursor = self.find(query, find_options).await?;
        while let Some(item) = cursor.try_next().await? {
            items.push(item);
        }

        if items.is_empty() {
            Err(Box::new(Error::new(
                ErrorKind::Other,
                "Item not found".to_string(),
            )))
        } else {
            Ok(items)
        }
    }
}
