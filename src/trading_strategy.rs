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
    GridEntry(TrendType),
    FlashCrash,
    RandomInago(TrendType),
    RandomInagoReversion(TrendType),
    RandomGridEntry(TrendType),
}

impl TradingStrategy {
    pub fn trend_type(&self) -> &TrendType {
        match self {
            TradingStrategy::Inago(t)
            | TradingStrategy::InagoReversion(t)
            | TradingStrategy::GridEntry(t)
            | TradingStrategy::RandomInago(t)
            | TradingStrategy::RandomInagoReversion(t)
            | TradingStrategy::RandomGridEntry(t) => t,
            TradingStrategy::FlashCrash => &TrendType::Up,
        }
    }
}

impl PartialEq for TradingStrategy {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            // Inago
            (TradingStrategy::Inago(TrendType::Any), TradingStrategy::Inago(_))
            | (TradingStrategy::Inago(_), TradingStrategy::Inago(TrendType::Any)) => true,
            (TradingStrategy::Inago(t1), TradingStrategy::Inago(t2)) if t1 == t2 => true,

            // InagoReversion
            (
                TradingStrategy::InagoReversion(TrendType::Any),
                TradingStrategy::InagoReversion(_),
            )
            | (
                TradingStrategy::InagoReversion(_),
                TradingStrategy::InagoReversion(TrendType::Any),
            ) => true,
            (TradingStrategy::InagoReversion(t1), TradingStrategy::InagoReversion(t2))
                if t1 == t2 =>
            {
                true
            }

            // GridEntry
            (TradingStrategy::GridEntry(TrendType::Any), TradingStrategy::GridEntry(_))
            | (TradingStrategy::GridEntry(_), TradingStrategy::GridEntry(TrendType::Any)) => true,
            (TradingStrategy::GridEntry(t1), TradingStrategy::GridEntry(t2)) if t1 == t2 => true,

            // RandomInago
            (TradingStrategy::RandomInago(TrendType::Any), TradingStrategy::RandomInago(_))
            | (TradingStrategy::RandomInago(_), TradingStrategy::RandomInago(TrendType::Any)) => {
                true
            }
            (TradingStrategy::RandomInago(t1), TradingStrategy::RandomInago(t2)) if t1 == t2 => {
                true
            }

            // RandomInagoReversion
            (
                TradingStrategy::RandomInagoReversion(TrendType::Any),
                TradingStrategy::RandomInagoReversion(_),
            )
            | (
                TradingStrategy::RandomInagoReversion(_),
                TradingStrategy::RandomInagoReversion(TrendType::Any),
            ) => true,
            (
                TradingStrategy::RandomInagoReversion(t1),
                TradingStrategy::RandomInagoReversion(t2),
            ) if t1 == t2 => true,

            // RandomGridEntry
            (
                TradingStrategy::RandomGridEntry(TrendType::Any),
                TradingStrategy::RandomGridEntry(_),
            )
            | (
                TradingStrategy::RandomGridEntry(_),
                TradingStrategy::RandomGridEntry(TrendType::Any),
            ) => true,
            (TradingStrategy::RandomGridEntry(t1), TradingStrategy::RandomGridEntry(t2))
                if t1 == t2 =>
            {
                true
            }

            // FlashCrash
            (TradingStrategy::FlashCrash, TradingStrategy::FlashCrash) => true,

            _ => false,
        }
    }
}
