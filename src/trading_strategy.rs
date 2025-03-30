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
    InagoReversion(TrendType),
    RandomInago(TrendType),
    RandomInagoReversion(TrendType),
    RandomTrendFollow(TrendType),
}

impl TradingStrategy {
    pub fn trend_type(&self) -> &TrendType {
        match self {
            TradingStrategy::RandomInago(t)
            | TradingStrategy::InagoReversion(t)
            | TradingStrategy::RandomInagoReversion(t)
            | TradingStrategy::Inago(t)
            | TradingStrategy::RandomTrendFollow(t) => t,
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
                TradingStrategy::InagoReversion(TrendType::Any),
                TradingStrategy::InagoReversion(TrendType::Up),
            )
            | (
                TradingStrategy::InagoReversion(TrendType::Up),
                TradingStrategy::InagoReversion(TrendType::Any),
            )
            | (
                TradingStrategy::InagoReversion(TrendType::Any),
                TradingStrategy::InagoReversion(TrendType::Down),
            )
            | (
                TradingStrategy::InagoReversion(TrendType::Down),
                TradingStrategy::InagoReversion(TrendType::Any),
            ) => true,
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

            (
                TradingStrategy::RandomInagoReversion(TrendType::Any),
                TradingStrategy::RandomInagoReversion(TrendType::Up),
            )
            | (
                TradingStrategy::RandomInagoReversion(TrendType::Up),
                TradingStrategy::RandomInagoReversion(TrendType::Any),
            )
            | (
                TradingStrategy::RandomInagoReversion(TrendType::Any),
                TradingStrategy::RandomInagoReversion(TrendType::Down),
            )
            | (
                TradingStrategy::RandomInagoReversion(TrendType::Down),
                TradingStrategy::RandomInagoReversion(TrendType::Any),
            ) => true,
            (
                TradingStrategy::RandomInagoReversion(t1),
                TradingStrategy::RandomInagoReversion(t2),
            ) if t1 == t2 => true,

            (
                TradingStrategy::RandomTrendFollow(TrendType::Any),
                TradingStrategy::RandomTrendFollow(TrendType::Up),
            )
            | (
                TradingStrategy::RandomTrendFollow(TrendType::Up),
                TradingStrategy::RandomTrendFollow(TrendType::Any),
            )
            | (
                TradingStrategy::RandomTrendFollow(TrendType::Any),
                TradingStrategy::RandomTrendFollow(TrendType::Down),
            )
            | (
                TradingStrategy::RandomTrendFollow(TrendType::Down),
                TradingStrategy::RandomTrendFollow(TrendType::Any),
            ) => true,
            (TradingStrategy::RandomTrendFollow(t1), TradingStrategy::RandomTrendFollow(t2))
                if t1 == t2 =>
            {
                true
            }

            _ => false,
        }
    }
}
