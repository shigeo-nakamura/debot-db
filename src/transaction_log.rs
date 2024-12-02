// transaction_log.rs

use bson::doc;
use bson::Bson;
use bson::Document;
use debot_utils::get_local_time;
use debot_utils::HasId;
use mongodb::Collection;
use mongodb::{
    options::{ClientOptions, Tls, TlsOptions},
    Database,
};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use shared_mongodb::{database, ClientHolder};
use std::collections::HashMap;
use std::error;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::sync::Arc;
use std::time::SystemTime;
use tokio::sync::Mutex;

use crate::delete_item_all;
use crate::SearchMode;
use crate::TradingStrategy;
use crate::{
    create_unique_index, insert_item, search_item, search_items, update_item, Counter, CounterType,
    Entity,
};

async fn get_last_id<T: Default + Entity + HasId>(db: &Database) -> u32 {
    let item = T::default();
    match search_items(db, &item, crate::SearchMode::Descending, Some(1), None).await {
        Ok(mut items) => items.pop().and_then(|item| item.id()).unwrap_or(0),
        Err(e) => {
            log::info!("get_last_id: {:?}", e);
            0
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum SampleTerm {
    TradingTerm,
    ShortTerm,
    LongTerm,
}

impl SampleTerm {
    pub fn to_numeric(&self) -> Decimal {
        match self {
            SampleTerm::TradingTerm => Decimal::new(1, 0),
            SampleTerm::ShortTerm => Decimal::new(2, 0),
            SampleTerm::LongTerm => Decimal::new(3, 0),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FundConfig {
    pub token: String,
    pub trading_strategy: TradingStrategy,
    pub amount_per_strategy: Decimal,
    pub risk_reward: Decimal,
    pub take_profit_ratio: Option<Decimal>,
    pub atr_spread: Option<Decimal>,
    pub atr_term: SampleTerm,
    pub open_hours: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppState {
    pub id: u32,
    pub last_execution_time: Option<SystemTime>,
    pub last_equity: Option<Decimal>,
    pub ave_dd: Option<Decimal>,
    pub max_dd: Option<Decimal>,
    pub cumulative_return: Decimal,
    pub cumulative_dd: Decimal,
    pub score: Option<Decimal>,
    pub score_2: Option<Decimal>,
    pub score_3: Option<Decimal>,
    pub curcuit_break: bool,
    pub error_time: Vec<String>,
    pub max_invested_amount: Decimal,
    pub fund_configs: Option<Vec<FundConfig>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            id: 1,
            last_execution_time: None,
            last_equity: None,
            ave_dd: None,
            max_dd: None,
            cumulative_return: Decimal::ZERO,
            cumulative_dd: Decimal::ZERO,
            score: None,
            score_2: None,
            score_3: None,
            curcuit_break: false,
            error_time: vec![],
            max_invested_amount: Decimal::ZERO,
            fund_configs: Some(vec![]),
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

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PricePoint {
    pub timestamp: i64,
    pub timestamp_str: String,
    pub price: Decimal,
    pub volume: Option<Decimal>,
    pub num_trades: Option<u64>,
    pub funding_rate: Option<Decimal>,
    pub open_interest: Option<Decimal>,
    pub oracle_price: Option<Decimal>,
}

impl PricePoint {
    pub fn new(
        price: Decimal,
        timestamp: Option<i64>,
        volume: Option<Decimal>,
        num_trades: Option<u64>,
        funding_rate: Option<Decimal>,
        open_interest: Option<Decimal>,
        oracle_price: Option<Decimal>,
    ) -> Self {
        let (local_timestamp, timestamp_str) = get_local_time();
        let timestamp = timestamp.unwrap_or(local_timestamp);
        Self {
            timestamp,
            timestamp_str,
            price,
            volume,
            num_trades,
            funding_rate,
            open_interest,
            oracle_price,
        }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum CandlePattern {
    #[default]
    None,
    Hammer,
    InvertedHammer,
    BullishEngulfing,
    BearishEngulfing,
    Doji,
    Marubozu,
    MorningStar,
    EveningStar,
    ThreeWhiteSoldiers,
    ThreeBlackCrows,
    PiercingPattern,
    DarkCloudCover,
    Harami,
    HaramiCross,
    SpinningTop,
}

impl CandlePattern {
    pub fn to_one_hot(&self) -> [Decimal; 16] {
        let mut one_hot = [Decimal::ZERO; 16];

        match self {
            CandlePattern::None => one_hot[0] = Decimal::ONE,
            CandlePattern::Hammer => one_hot[1] = Decimal::ONE,
            CandlePattern::InvertedHammer => one_hot[2] = Decimal::ONE,
            CandlePattern::BullishEngulfing => one_hot[3] = Decimal::ONE,
            CandlePattern::BearishEngulfing => one_hot[4] = Decimal::ONE,
            CandlePattern::Doji => one_hot[5] = Decimal::ONE,
            CandlePattern::Marubozu => one_hot[6] = Decimal::ONE,
            CandlePattern::MorningStar => one_hot[7] = Decimal::ONE,
            CandlePattern::EveningStar => one_hot[8] = Decimal::ONE,
            CandlePattern::ThreeWhiteSoldiers => one_hot[9] = Decimal::ONE,
            CandlePattern::ThreeBlackCrows => one_hot[10] = Decimal::ONE,
            CandlePattern::PiercingPattern => one_hot[11] = Decimal::ONE,
            CandlePattern::DarkCloudCover => one_hot[12] = Decimal::ONE,
            CandlePattern::Harami => one_hot[13] = Decimal::ONE,
            CandlePattern::HaramiCross => one_hot[14] = Decimal::ONE,
            CandlePattern::SpinningTop => one_hot[15] = Decimal::ONE,
        }

        one_hot
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct DebugLog {
    pub input_1: Decimal,
    pub input_2: Decimal,
    pub input_3: Decimal,
    pub input_4: Decimal,
    pub input_5: Decimal,
    pub input_6: Decimal,
    pub input_7: Decimal,
    pub input_8: Decimal,
    pub input_9: Decimal,
    pub input_10: Decimal,
    pub input_11: Decimal,
    pub input_12: Decimal,
    pub input_13: Decimal,
    pub input_14: Decimal,
    pub input_15: Decimal,
    pub input_16: Decimal,
    pub input_17: Decimal,
    pub input_18: Decimal,
    pub input_19: Decimal,
    pub input_20: Decimal,
    pub input_21: Decimal,
    pub input_22: Decimal,
    pub input_23: Decimal,
    pub input_24: Decimal,
    pub input_25: Decimal,
    pub input_26: Decimal,
    pub input_27: Decimal,
    pub input_28: Decimal,
    pub input_29: Decimal,
    pub input_30: CandlePattern,
    pub input_31: CandlePattern,
    pub input_32: CandlePattern,
    pub input_33: CandlePattern,
    pub input_34: CandlePattern,
    pub input_35: CandlePattern,
    pub input_36: CandlePattern,
    pub input_37: CandlePattern,
    pub input_38: CandlePattern,
    pub input_39: CandlePattern,
    pub output_1: Decimal,
    pub output_2: Decimal,
    pub output_3: Option<Decimal>,
    pub output_4: Option<Decimal>,
    pub output_5: Option<Decimal>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct PositionLog {
    pub id: Option<u32>,
    pub fund_name: String,
    pub order_id: String,
    pub ordered_price: Decimal,
    pub state: String,
    pub token_name: String,
    pub open_time_str: String,
    pub open_timestamp: i64,
    pub close_time_str: String,
    pub average_open_price: Decimal,
    pub position_type: String,
    pub close_price: Decimal,
    pub asset_in_usd: Decimal,
    pub pnl: Decimal,
    pub fee: Decimal,
    pub debug: DebugLog,
}

#[derive(Serialize, Deserialize)]
pub struct SerializableModel {
    pub model: Vec<u8>,
}

impl HasId for PositionLog {
    fn id(&self) -> Option<u32> {
        self.id
    }
}

pub struct TransactionLog {
    counter: Counter,
    db_r_name: String,
    db_w_name: String,
    client_holder: Arc<Mutex<ClientHolder>>,
}

impl TransactionLog {
    pub async fn new(
        max_position_counter: Option<u32>,
        max_price_counter: Option<u32>,
        max_pnl_counter: Option<u32>,
        mongodb_uri: &str,
        db_r_name: &str,
        db_w_name: &str,
        back_test: bool,
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

        let db = shared_mongodb::database::get(&client_holder, &db_w_name)
            .await
            .unwrap();

        create_unique_index(&db)
            .await
            .expect("Error creating unique index");

        if back_test {
            if let Err(e) = Self::delete_all_positions(&db).await {
                panic!("delete_all_positions failed: {:?}", e);
            }
            if let Err(e) = Self::delete_app_state(&db).await {
                panic!("delete_app_state failed: {:?}", e);
            }
        }

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

        log::warn!(
            "position = {}/{:?}, price = {}/{:?}, pnl = {}/{:?}",
            last_position_counter,
            max_position_counter,
            last_price_counter,
            max_price_counter,
            last_pnl_counter,
            max_pnl_counter,
        );

        TransactionLog {
            counter,
            db_r_name: db_r_name.to_owned(),
            db_w_name: db_w_name.to_owned(),
            client_holder,
        }
    }

    pub fn increment_counter(&self, counter_type: CounterType) -> u32 {
        self.counter.increment(counter_type)
    }

    pub async fn get_last_transaction_id(db: &Database, counter_type: CounterType) -> u32 {
        match counter_type {
            CounterType::Position => get_last_id::<PositionLog>(db).await,
            CounterType::Price => get_last_id::<PriceLog>(db).await,
            CounterType::Pnl => get_last_id::<PnlLog>(db).await,
        }
    }

    pub async fn get_w_db(&self) -> Option<Database> {
        self.get_db(false).await
    }

    pub async fn get_r_db(&self) -> Option<Database> {
        self.get_db(true).await
    }

    async fn get_db(&self, read: bool) -> Option<Database> {
        let db_name = if read {
            &self.db_r_name
        } else {
            &self.db_w_name
        };
        let db = match database::get(&self.client_holder, db_name).await {
            Ok(db) => Some(db),
            Err(e) => {
                log::error!("get_db: {:?}", e);
                None
            }
        };
        db
    }

    pub async fn update_transaction(
        db: &Database,
        item: &PositionLog,
    ) -> Result<(), Box<dyn error::Error>> {
        update_item(db, item).await?;
        Ok(())
    }

    pub async fn update_price(db: &Database, item: PriceLog) -> Result<(), Box<dyn error::Error>> {
        update_item(db, &item).await?;
        Ok(())
    }

    pub async fn copy_price(db_r: &Database, db_w: &Database, limit: Option<u32>) {
        let item = PriceLog::default();
        let items = {
            match search_items(db_r, &item, SearchMode::Ascending, limit, None).await {
                Ok(items) => items,
                Err(e) => {
                    log::error!("get price: {:?}", e);
                    return;
                }
            }
        };
        log::debug!("get prices: num = {}", items.len());

        for item in &items {
            match insert_item(db_w, item).await {
                Ok(_) => {}
                Err(e) => {
                    log::error!("write price: {:?}", e);
                    return;
                }
            }
        }
    }

    pub async fn get_price_market_data(
        db: &Database,
        limit: Option<u32>,
        id: Option<u32>,
        is_ascend: bool,
    ) -> HashMap<String, HashMap<String, Vec<PricePoint>>> {
        let search_mode = if is_ascend {
            SearchMode::Ascending
        } else {
            SearchMode::Descending
        };
        let item = PriceLog::default();
        let items = if id.is_some() {
            match search_item(db, &item, id).await {
                Ok(items) => {
                    let mut item_vec = Vec::new();
                    item_vec.push(items);
                    item_vec
                }
                Err(e) => {
                    log::warn!("get_price_market_data: {:?}", e);
                    return HashMap::new();
                }
            }
        } else if limit.is_some() {
            match search_items(db, &item, search_mode, limit, None).await {
                Ok(items) => items,
                Err(e) => {
                    log::warn!("get_price_market_data: {:?}", e);
                    return HashMap::new();
                }
            }
        } else {
            match search_items(db, &item, search_mode, None, None).await {
                Ok(items) => items,
                Err(e) => {
                    log::warn!("get_price_market_data: {:?}", e);
                    return HashMap::new();
                }
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

    pub async fn get_all_positions(db: &Database) -> Vec<PositionLog> {
        let item = PositionLog::default();
        let items = match search_items(db, &item, crate::SearchMode::Ascending, None, None).await {
            Ok(items) => items.into_iter().collect(),
            Err(e) => {
                log::error!("get_all_position: {:?}", e);
                vec![]
            }
        };
        log::trace!("get_all_position: {:?}", items);
        items
    }

    async fn delete_all_positions(db: &Database) -> Result<(), Box<dyn error::Error>> {
        let item = PositionLog::default();
        delete_item_all(db, &item).await
    }

    pub async fn insert_pnl(db: &Database, item: PnlLog) -> Result<(), Box<dyn error::Error>> {
        insert_item(db, &item).await?;
        Ok(())
    }

    pub async fn get_app_state(db: &Database) -> AppState {
        let item = AppState::default();
        match search_item(db, &item, Some(1)).await {
            Ok(item) => item,
            Err(e) => {
                log::warn!("get_app_state: {:?}", e);
                item
            }
        }
    }

    async fn delete_app_state(db: &Database) -> Result<(), Box<dyn error::Error>> {
        let item = AppState::default();
        delete_item_all(db, &item).await
    }

    pub async fn update_app_state(
        db: &Database,
        last_execution_time: Option<SystemTime>,
        last_equity: Option<Decimal>,
        ave_dd: Option<Decimal>,
        max_dd: Option<Decimal>,
        cumulative_return: Option<Decimal>,
        cumulative_dd: Option<Decimal>,
        score: Option<Decimal>,
        score_2: Option<Decimal>,
        score_3: Option<Decimal>,
        curcuit_break: bool,
        error_time: Option<String>,
        max_invested_amount: Option<Decimal>,
        fund_configs: Option<Vec<FundConfig>>,
    ) -> Result<(), Box<dyn error::Error>> {
        let item = AppState::default();
        let mut item = match search_item(db, &item, Some(1)).await {
            Ok(prev_item) => prev_item,
            Err(_) => item,
        };

        if last_execution_time.is_some() {
            item.last_execution_time = last_execution_time;
        }

        if last_equity.is_some() {
            item.last_equity = last_equity;
        }

        if ave_dd.is_some() {
            item.ave_dd = ave_dd;
        }

        if let Some(max_dd_val) = max_dd {
            if item
                .max_dd
                .map_or(true, |item_max_dd| max_dd_val > item_max_dd)
            {
                item.max_dd = Some(max_dd_val);
            }
        }

        if cumulative_return.is_some() {
            item.cumulative_return += cumulative_return.unwrap();
        }

        if cumulative_dd.is_some() {
            item.cumulative_dd += cumulative_dd.unwrap();
        }

        if score.is_some() {
            item.score = score;
        }

        if score_2.is_some() {
            item.score_2 = score_2;
        }

        if score_3.is_some() {
            item.score_3 = score_3;
        }

        item.curcuit_break = curcuit_break;

        if let Some(error_time) = error_time {
            item.error_time.push(error_time);
        }

        if let Some(max_invested_amount) = max_invested_amount {
            item.max_invested_amount = max_invested_amount;
        }

        if let Some(fund_configs) = fund_configs {
            item.fund_configs = Some(fund_configs);
        }

        update_item(db, &item).await?;
        Ok(())
    }
}

#[derive(Clone)]
pub struct ModelParams {
    db_name: String,
    client_holder: Arc<Mutex<ClientHolder>>,
    collection_name: String,
    save_to_db: bool,
    file_path: Option<String>,
}

impl ModelParams {
    pub async fn new(
        mongodb_uri: &str,
        db_name: &str,
        save_to_db: bool,
        file_path: Option<String>,
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

        ModelParams {
            db_name: db_name.to_owned(),
            client_holder,
            collection_name: "model_params".to_owned(),
            save_to_db,
            file_path,
        }
    }

    async fn get_db(&self) -> Option<Database> {
        let db = match database::get(&self.client_holder, &self.db_name).await {
            Ok(db) => Some(db),
            Err(e) => {
                log::error!("get_db: {:?}", e);
                None
            }
        };
        db
    }

    pub async fn save_model(
        &self,
        key: &str,
        model: &SerializableModel,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if self.save_to_db {
            self.save_model_to_db(key, model).await
        } else {
            self.save_model_to_file(key, model).await
        }
    }

    pub async fn load_model(
        &self,
        key: &str,
    ) -> Result<SerializableModel, Box<dyn std::error::Error>> {
        if self.save_to_db {
            self.load_model_from_db(key).await
        } else {
            self.load_model_from_file(key).await
        }
    }

    async fn save_model_to_db(
        &self,
        key: &str,
        model: &SerializableModel,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let db = self.get_db().await.ok_or("no db")?;
        let collection: Collection<Document> = db.collection(&self.collection_name);
        let serialized_model = bincode::serialize(model)?;

        let document = doc! {
            "key": key,
            "model": Bson::Binary(mongodb::bson::Binary {
                subtype: mongodb::bson::spec::BinarySubtype::Generic,
                bytes: serialized_model
            })
        };

        collection
            .update_one(
                doc! { "key": key },
                doc! { "$set": document },
                mongodb::options::UpdateOptions::builder()
                    .upsert(true)
                    .build(),
            )
            .await?;
        Ok(())
    }

    async fn load_model_from_db(
        &self,
        key: &str,
    ) -> Result<SerializableModel, Box<dyn std::error::Error>> {
        let db = self.get_db().await.ok_or("no db")?;
        let collection: Collection<Document> = db.collection(&self.collection_name);

        let filter = doc! { "key": key };
        let document = collection
            .find_one(filter, None)
            .await?
            .ok_or("No model found in the collection")?;

        if let Some(Bson::Binary(model_bytes)) = document.get("model") {
            let model: SerializableModel = bincode::deserialize(&model_bytes.bytes)?;
            Ok(model)
        } else {
            Err("Invalid data format".into())
        }
    }

    async fn save_model_to_file(
        &self,
        key: &str,
        model: &SerializableModel,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let serialized_model = bincode::serialize(model)?;
        let file_name = format!("{}.bin", key);

        let file_path = if let Some(ref dir) = self.file_path {
            Path::new(dir).join(file_name)
        } else {
            Path::new(&file_name).to_path_buf()
        };

        let mut file = File::create(&file_path)?;
        file.write_all(&serialized_model)?;
        Ok(())
    }

    async fn load_model_from_file(
        &self,
        key: &str,
    ) -> Result<SerializableModel, Box<dyn std::error::Error>> {
        let file_name = format!("{}.bin", key);

        let file_path = if let Some(ref dir) = self.file_path {
            Path::new(dir).join(file_name)
        } else {
            Path::new(&file_name).to_path_buf()
        };

        let mut file = File::open(&file_path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        let model: SerializableModel = bincode::deserialize(&buffer)?;
        Ok(model)
    }
}
