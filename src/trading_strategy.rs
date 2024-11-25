use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum TrendType {
    Up,
    Down,
    Any,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Serialize, Deserialize)]
pub enum TradingStrategy {
    RandomWalk(TrendType),
    RandomForest(TrendType),
}

impl TradingStrategy {
    pub fn trend_type(&self) -> &TrendType {
        match self {
            TradingStrategy::RandomWalk(t) | TradingStrategy::RandomForest(t) => t,
        }
    }
}

impl PartialEq for TradingStrategy {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                TradingStrategy::RandomWalk(TrendType::Any),
                TradingStrategy::RandomWalk(TrendType::Up),
            ) => true,
            (
                TradingStrategy::RandomWalk(TrendType::Any),
                TradingStrategy::RandomWalk(TrendType::Down),
            ) => true,
            (
                TradingStrategy::RandomWalk(TrendType::Up),
                TradingStrategy::RandomWalk(TrendType::Any),
            ) => true,
            (
                TradingStrategy::RandomWalk(TrendType::Down),
                TradingStrategy::RandomWalk(TrendType::Any),
            ) => true,
            (TradingStrategy::RandomWalk(t1), TradingStrategy::RandomWalk(t2)) if t1 == t2 => true,

            (
                TradingStrategy::RandomForest(TrendType::Any),
                TradingStrategy::RandomForest(TrendType::Up),
            ) => true,
            (
                TradingStrategy::RandomForest(TrendType::Any),
                TradingStrategy::RandomForest(TrendType::Down),
            ) => true,
            (
                TradingStrategy::RandomForest(TrendType::Up),
                TradingStrategy::RandomForest(TrendType::Any),
            ) => true,
            (
                TradingStrategy::RandomForest(TrendType::Down),
                TradingStrategy::RandomForest(TrendType::Any),
            ) => true,
            (TradingStrategy::RandomForest(t1), TradingStrategy::RandomForest(t2)) if t1 == t2 => {
                true
            }

            _ => false,
        }
    }
}
