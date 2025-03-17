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
    RandomMeanReversion(TrendType),
    Grid(TrendType),
    Inago(TrendType),
    RandomInago(TrendType),
}

impl TradingStrategy {
    pub fn trend_type(&self) -> &TrendType {
        match self {
            TradingStrategy::RandomMeanReversion(t)
            | TradingStrategy::MeanReversion(t)
            | TradingStrategy::Grid(t)
            | TradingStrategy::RandomInago(t)
            | TradingStrategy::Inago(t) => t,
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
                TradingStrategy::RandomMeanReversion(TrendType::Any),
                TradingStrategy::RandomMeanReversion(TrendType::Up),
            )
            | (
                TradingStrategy::RandomMeanReversion(TrendType::Up),
                TradingStrategy::RandomMeanReversion(TrendType::Any),
            )
            | (
                TradingStrategy::RandomMeanReversion(TrendType::Any),
                TradingStrategy::RandomMeanReversion(TrendType::Down),
            )
            | (
                TradingStrategy::RandomMeanReversion(TrendType::Down),
                TradingStrategy::RandomMeanReversion(TrendType::Any),
            ) => true,
            (
                TradingStrategy::RandomMeanReversion(t1),
                TradingStrategy::RandomMeanReversion(t2),
            ) if t1 == t2 => true,

            (TradingStrategy::Grid(TrendType::Any), TradingStrategy::Grid(TrendType::Up))
            | (TradingStrategy::Grid(TrendType::Up), TradingStrategy::Grid(TrendType::Any))
            | (TradingStrategy::Grid(TrendType::Any), TradingStrategy::Grid(TrendType::Down))
            | (TradingStrategy::Grid(TrendType::Down), TradingStrategy::Grid(TrendType::Any)) => {
                true
            }
            (TradingStrategy::Grid(t1), TradingStrategy::Grid(t2)) if t1 == t2 => true,

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
