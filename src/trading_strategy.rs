use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum TrendType {
    Up,
    Down,
    Any,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Serialize, Deserialize)]
pub enum TradingStrategy {
    Inago(TrendType),
    RandomInago(TrendType),
}

impl TradingStrategy {
    pub fn trend_type(&self) -> &TrendType {
        match self {
            TradingStrategy::RandomInago(t) | TradingStrategy::Inago(t) => t,
        }
    }
}

impl PartialEq for TradingStrategy {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TradingStrategy::Inago(TrendType::Any), TradingStrategy::Inago(TrendType::Up))
            | (TradingStrategy::Inago(TrendType::Up), TradingStrategy::Inago(TrendType::Any))
            | (TradingStrategy::Inago(TrendType::Any), TradingStrategy::Inago(TrendType::Down))
            | (TradingStrategy::Inago(TrendType::Down), TradingStrategy::Inago(TrendType::Any)) => {
                true
            }
            (TradingStrategy::Inago(t1), TradingStrategy::Inago(t2)) if t1 == t2 => true,

            (
                TradingStrategy::RandomInago(TrendType::Any),
                TradingStrategy::RandomInago(TrendType::Up),
            )
            | (
                TradingStrategy::RandomInago(TrendType::Up),
                TradingStrategy::RandomInago(TrendType::Any),
            )
            | (
                TradingStrategy::RandomInago(TrendType::Any),
                TradingStrategy::RandomInago(TrendType::Down),
            )
            | (
                TradingStrategy::RandomInago(TrendType::Down),
                TradingStrategy::RandomInago(TrendType::Any),
            ) => true,
            (TradingStrategy::RandomInago(t1), TradingStrategy::RandomInago(t2)) if t1 == t2 => {
                true
            }

            _ => false,
        }
    }
}
