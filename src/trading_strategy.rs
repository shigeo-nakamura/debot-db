use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum TrendType {
    Up,
    Down,
    Any,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Serialize, Deserialize)]
pub enum TradingStrategy {
    MarketMake(TrendType),
    Inago(TrendType),
    MeanReversion(TrendType),
    RandomInago(TrendType),
    RandomMeanReversion(TrendType),
    RandomMarketMake(TrendType),
    Hybrid,
    Rebalance,
}

impl TradingStrategy {
    pub fn trend_type(&self) -> &TrendType {
        match self {
            TradingStrategy::Inago(t)
            | TradingStrategy::MarketMake(t)
            | TradingStrategy::MeanReversion(t)
            | TradingStrategy::RandomInago(t)
            | TradingStrategy::RandomMarketMake(t)
            | TradingStrategy::RandomMeanReversion(t) => t,
            TradingStrategy::Hybrid | TradingStrategy::Rebalance => &TrendType::Any,
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
            (TradingStrategy::MarketMake(TrendType::Any), TradingStrategy::MarketMake(_))
            | (TradingStrategy::MarketMake(_), TradingStrategy::MarketMake(TrendType::Any)) => true,
            (TradingStrategy::MarketMake(t1), TradingStrategy::MarketMake(t2)) if t1 == t2 => true,

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
            (
                TradingStrategy::RandomMarketMake(TrendType::Any),
                TradingStrategy::RandomMarketMake(_),
            )
            | (
                TradingStrategy::RandomMarketMake(_),
                TradingStrategy::RandomMarketMake(TrendType::Any),
            ) => true,
            (TradingStrategy::RandomMarketMake(t1), TradingStrategy::RandomMarketMake(t2))
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
