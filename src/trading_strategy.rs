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
    MeanReversion(TrendType),
    FlashCrash,
    RandomInago(TrendType),
    RandomInagoReversion(TrendType),
    RandomGridEntry(TrendType),
    RandomMeanReversion(TrendType),
    RandomFlashCrash,
}

impl TradingStrategy {
    pub fn trend_type(&self) -> &TrendType {
        match self {
            TradingStrategy::Inago(t)
            | TradingStrategy::InagoReversion(t)
            | TradingStrategy::GridEntry(t)
            | TradingStrategy::MeanReversion(t)
            | TradingStrategy::RandomInago(t)
            | TradingStrategy::RandomInagoReversion(t)
            | TradingStrategy::RandomGridEntry(t) => t,
            TradingStrategy::RandomMeanReversion(t) => t,
            TradingStrategy::RandomFlashCrash | TradingStrategy::FlashCrash => &TrendType::Up,
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

            // MeanReversion
            (TradingStrategy::MeanReversion(TrendType::Any), TradingStrategy::MeanReversion(_))
            | (TradingStrategy::MeanReversion(_), TradingStrategy::MeanReversion(TrendType::Any)) => {
                true
            }
            (TradingStrategy::MeanReversion(t1), TradingStrategy::MeanReversion(t2))
                if t1 == t2 =>
            {
                true
            }

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

            // RandomMeanReversion
            (
                TradingStrategy::RandomMeanReversion(TrendType::Any),
                TradingStrategy::RandomMeanReversion(_),
            )
            | (
                TradingStrategy::RandomMeanReversion(_),
                TradingStrategy::RandomMeanReversion(TrendType::Any),
            ) => true,
            (
                TradingStrategy::RandomMeanReversion(t1),
                TradingStrategy::RandomMeanReversion(t2),
            ) if t1 == t2 => true,

            // RandomFlashCrash
            (TradingStrategy::RandomFlashCrash, TradingStrategy::RandomFlashCrash) => true,

            // FlashCrash
            (TradingStrategy::FlashCrash, TradingStrategy::FlashCrash) => true,

            _ => false,
        }
    }
}
