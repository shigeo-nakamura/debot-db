use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum TrendType {
    Up,
    Down,
    Any,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Serialize, Deserialize)]
pub enum TradingStrategy {
    MeanReversion(TrendType),
    UntrainedMeanReversion(TrendType),
    GridTrade(TrendType),
}

impl TradingStrategy {
    pub fn trend_type(&self) -> &TrendType {
        match self {
            TradingStrategy::UntrainedMeanReversion(t)
            | TradingStrategy::MeanReversion(t)
            | TradingStrategy::GridTrade(t) => t,
        }
    }
}

impl PartialEq for TradingStrategy {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                TradingStrategy::MeanReversion(TrendType::Any),
                TradingStrategy::MeanReversion(TrendType::Up),
            )
            | (
                TradingStrategy::MeanReversion(TrendType::Up),
                TradingStrategy::MeanReversion(TrendType::Any),
            )
            | (
                TradingStrategy::MeanReversion(TrendType::Any),
                TradingStrategy::MeanReversion(TrendType::Down),
            )
            | (
                TradingStrategy::MeanReversion(TrendType::Down),
                TradingStrategy::MeanReversion(TrendType::Any),
            ) => true,
            (TradingStrategy::MeanReversion(t1), TradingStrategy::MeanReversion(t2))
                if t1 == t2 =>
            {
                true
            }

            (
                TradingStrategy::UntrainedMeanReversion(TrendType::Any),
                TradingStrategy::UntrainedMeanReversion(TrendType::Up),
            )
            | (
                TradingStrategy::UntrainedMeanReversion(TrendType::Up),
                TradingStrategy::UntrainedMeanReversion(TrendType::Any),
            )
            | (
                TradingStrategy::UntrainedMeanReversion(TrendType::Any),
                TradingStrategy::UntrainedMeanReversion(TrendType::Down),
            )
            | (
                TradingStrategy::UntrainedMeanReversion(TrendType::Down),
                TradingStrategy::UntrainedMeanReversion(TrendType::Any),
            ) => true,
            (
                TradingStrategy::UntrainedMeanReversion(t1),
                TradingStrategy::UntrainedMeanReversion(t2),
            ) if t1 == t2 => true,

            (
                TradingStrategy::GridTrade(TrendType::Any),
                TradingStrategy::GridTrade(TrendType::Up),
            )
            | (
                TradingStrategy::GridTrade(TrendType::Up),
                TradingStrategy::GridTrade(TrendType::Any),
            )
            | (
                TradingStrategy::GridTrade(TrendType::Any),
                TradingStrategy::GridTrade(TrendType::Down),
            )
            | (
                TradingStrategy::GridTrade(TrendType::Down),
                TradingStrategy::GridTrade(TrendType::Any),
            ) => true,
            (TradingStrategy::GridTrade(t1), TradingStrategy::GridTrade(t2)) if t1 == t2 => true,

            _ => false,
        }
    }
}
