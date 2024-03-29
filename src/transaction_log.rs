// transaction_log.rs

use debot_market_analyzer::PricePoint;
use debot_position_manager::{State, TradePosition};
use debot_utils::HasId;
use mongodb::{
    options::{ClientOptions, Tls, TlsOptions},
    Database,
};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use shared_mongodb::{database, ClientHolder};
use std::collections::HashMap;
use std::error;
use std::sync::Arc;
use std::time::SystemTime;
use tokio::sync::Mutex;

use crate::{
    create_unique_index, insert_item, search_item, search_items, update_item, Counter, CounterType,
    Entity,
};

async fn get_last_id<T: Default + Entity + HasId>(db: &Database) -> u32 {
    let item = T::default();
    match search_items(db, &item).await {
        Ok(mut items) => items.pop().and_then(|item| item.id()).unwrap_or(0),
        Err(e) => {
            log::warn!("get_last_id: {:?}", e);
            0
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppState {
    pub id: u32,
    pub last_execution_time: Option<SystemTime>,
    pub last_equity: Option<Decimal>,
    pub curcuit_break: bool,
    pub error_time: Vec<String>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            id: 1,
            last_execution_time: None,
            last_equity: None,
            curcuit_break: false,
            error_time: vec![],
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct PnlLog {
    pub id: Option<u32>,
    pub date: String,
    pub pnl: Decimal,
}

impl HasId for PnlLog {
    fn id(&self) -> Option<u32> {
        self.id
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct PriceLog {
    pub id: Option<u32>,
    pub name: String,
    pub token_name: String,
    pub price_point: PricePoint,
}

impl HasId for PriceLog {
    fn id(&self) -> Option<u32> {
        self.id
    }
}

pub struct TransactionLog {
    counter: Counter,
    db_name: String,
    client_holder: Arc<Mutex<ClientHolder>>,
}

impl TransactionLog {
    pub async fn new(
        max_position_counter: u32,
        max_price_counter: u32,
        max_pnl_counter: u32,
        mongodb_uri: &str,
        db_name: &str,
    ) -> Self {
        // Set up the DB client holder
        let mut client_options = match ClientOptions::parse(mongodb_uri).await {
            Ok(client_options) => client_options,
            Err(e) => {
                panic!("{:?}", e);
            }
        };
        let tls_options = TlsOptions::builder().build();
        client_options.tls = Some(Tls::Enabled(tls_options));
        let client_holder = Arc::new(Mutex::new(ClientHolder::new(client_options)));

        let db = shared_mongodb::database::get(&client_holder, &db_name)
            .await
            .unwrap();

        create_unique_index(&db)
            .await
            .expect("Error creating unique index");

        let last_position_counter =
            TransactionLog::get_last_transaction_id(&db, CounterType::Position).await;
        let last_price_counter =
            TransactionLog::get_last_transaction_id(&db, CounterType::Price).await;
        let last_pnl_counter = TransactionLog::get_last_transaction_id(&db, CounterType::Pnl).await;

        let counter = Counter::new(
            max_position_counter,
            max_price_counter,
            max_pnl_counter,
            last_position_counter,
            last_price_counter,
            last_pnl_counter,
        );

        TransactionLog {
            counter,
            db_name: db_name.to_owned(),
            client_holder,
        }
    }

    pub fn increment_counter(&self, counter_type: CounterType) -> u32 {
        self.counter.increment(counter_type)
    }

    pub async fn get_last_transaction_id(db: &Database, counter_type: CounterType) -> u32 {
        match counter_type {
            CounterType::Position => get_last_id::<TradePosition>(db).await,
            CounterType::Price => get_last_id::<PriceLog>(db).await,
            CounterType::Pnl => get_last_id::<PnlLog>(db).await,
        }
    }

    pub async fn get_db(&self) -> Option<Database> {
        let db = match database::get(&self.client_holder, &self.db_name).await {
            Ok(db) => Some(db),
            Err(e) => {
                log::error!("get_db: {:?}", e);
                None
            }
        };
        db
    }

    pub async fn get_all_open_positions(db: &Database) -> Vec<TradePosition> {
        let item = TradePosition::default();
        let items = match search_items(db, &item).await {
            Ok(items) => items
                .into_iter()
                .filter(|position| position.state() == State::Open)
                .collect(),
            Err(_) => {
                vec![]
            }
        };
        log::trace!("get_all_open_position: {:?}", items);
        items
    }

    pub async fn update_transaction(
        db: &Database,
        item: &TradePosition,
    ) -> Result<(), Box<dyn error::Error>> {
        update_item(db, item).await?;
        Ok(())
    }

    pub async fn update_price(db: &Database, item: PriceLog) -> Result<(), Box<dyn error::Error>> {
        update_item(db, &item).await?;
        Ok(())
    }

    pub async fn get_price_market_data(
        db: &Database,
    ) -> HashMap<String, HashMap<String, Vec<PricePoint>>> {
        let item = PriceLog::default();
        let items = match search_items(db, &item).await {
            Ok(items) => items,
            Err(e) => {
                log::warn!("get_price_market_data: {:?}", e);
                return HashMap::new();
            }
        };

        let mut result = HashMap::new();

        for price_log in items {
            result
                .entry(price_log.name)
                .or_insert_with(HashMap::new)
                .entry(price_log.token_name)
                .or_insert_with(Vec::new)
                .push(price_log.price_point);
        }

        for (_, token_map) in &mut result {
            for (_, price_points) in token_map {
                price_points.sort_by_key(|pp| pp.timestamp);
            }
        }

        result
    }

    pub async fn insert_pnl(db: &Database, item: PnlLog) -> Result<(), Box<dyn error::Error>> {
        insert_item(db, &item).await?;
        Ok(())
    }

    pub async fn get_app_state(db: &Database) -> AppState {
        let item = AppState::default();
        match search_item(db, &item).await {
            Ok(item) => item,
            Err(e) => {
                log::warn!("get_app_state: {:?}", e);
                item
            }
        }
    }

    pub async fn update_app_state(
        db: &Database,
        last_execution_time: Option<SystemTime>,
        last_equity: Option<Decimal>,
        curcuit_break: bool,
        error_time: Option<String>,
    ) -> Result<(), Box<dyn error::Error>> {
        let item = AppState::default();
        let mut item = match search_item(db, &item).await {
            Ok(prev_item) => prev_item,
            Err(_) => item,
        };

        if last_execution_time.is_some() {
            item.last_execution_time = last_execution_time;
        }

        if last_equity.is_some() {
            item.last_equity = last_equity;
        }

        if let Some(error_time) = error_time {
            item.error_time.push(error_time);
        }

        item.curcuit_break = curcuit_break;

        update_item(db, &item).await?;
        Ok(())
    }
}
