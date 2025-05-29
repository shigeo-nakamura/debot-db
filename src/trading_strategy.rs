use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum TrendType {
    Up,
    Down,
    Any,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Serialize, Deserialize)]
pub enum TradingStrategy {
    MarketMake,
    Inago(TrendType),
    MeanReversion(TrendType),
    RandomMarketMake,
    RandomInago(TrendType),
    RandomMeanReversion(TrendType),
    Hybrid,
    Rebalance,
}

impl TradingStrategy {
    pub fn trend_type(&self) -> &TrendType {
        match self {
            TradingStrategy::Inago(t)
            | TradingStrategy::MeanReversion(t)
            | TradingStrategy::RandomInago(t)
            | TradingStrategy::RandomMeanReversion(t) => t,
            TradingStrategy::Hybrid
            | TradingStrategy::Rebalance
            | TradingStrategy::MarketMake
            | TradingStrategy::RandomMarketMake => &TrendType::Any,
        }
    }
}

impl PartialEq for TradingStrategy {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            // Hybrid matches specific trends of Inago and MeanReversion
            (TradingStrategy::Hybrid, TradingStrategy::Inago(TrendType::Up))
            | (TradingStrategy::Hybrid, TradingStrategy::Inago(TrendType::Down))
            | (TradingStrategy::Hybrid, TradingStrategy::MeanReversion(TrendType::Up))
            | (TradingStrategy::Hybrid, TradingStrategy::MeanReversion(TrendType::Down))
            | (TradingStrategy::Inago(TrendType::Up), TradingStrategy::Hybrid)
            | (TradingStrategy::Inago(TrendType::Down), TradingStrategy::Hybrid)
            | (TradingStrategy::MeanReversion(TrendType::Up), TradingStrategy::Hybrid)
            | (TradingStrategy::MeanReversion(TrendType::Down), TradingStrategy::Hybrid)
            | (TradingStrategy::Hybrid, TradingStrategy::Hybrid) => true,

            // Rebalance equals itself only
            (TradingStrategy::Rebalance, TradingStrategy::Rebalance) => true,

            // MarketMake
            (TradingStrategy::MarketMake, TradingStrategy::MarketMake) => true,

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

            // RandomMarketMake
            (TradingStrategy::RandomMarketMake, TradingStrategy::RandomMarketMake) => true,

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
