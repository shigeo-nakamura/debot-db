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
    RandomGridEntry(TrendType),
}

impl TradingStrategy {
    pub fn trend_type(&self) -> &TrendType {
        match self {
            TradingStrategy::RandomInago(t)
            | TradingStrategy::InagoReversion(t)
            | TradingStrategy::RandomInagoReversion(t)
            | TradingStrategy::Inago(t)
            | TradingStrategy::RandomGridEntry(t) => t,
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
                TradingStrategy::RandomGridEntry(TrendType::Any),
                TradingStrategy::RandomGridEntry(TrendType::Up),
            )
            | (
                TradingStrategy::RandomGridEntry(TrendType::Up),
                TradingStrategy::RandomGridEntry(TrendType::Any),
            )
            | (
                TradingStrategy::RandomGridEntry(TrendType::Any),
                TradingStrategy::RandomGridEntry(TrendType::Down),
            )
            | (
                TradingStrategy::RandomGridEntry(TrendType::Down),
                TradingStrategy::RandomGridEntry(TrendType::Any),
            ) => true,
            (TradingStrategy::RandomGridEntry(t1), TradingStrategy::RandomGridEntry(t2))
                if t1 == t2 =>
            {
                true
            }

            _ => false,
        }
    }
}
