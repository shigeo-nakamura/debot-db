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
    MeanReversion(TrendType),
    RandomInago(TrendType),
    RandomMeanReversion(TrendType),
}

impl TradingStrategy {
    pub fn trend_type(&self) -> &TrendType {
        match self {
            TradingStrategy::Inago(t)
            | TradingStrategy::MeanReversion(t)
            | TradingStrategy::RandomInago(t)
            | TradingStrategy::RandomMeanReversion(t) => t,
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

            _ => false,
        }
    }
}
