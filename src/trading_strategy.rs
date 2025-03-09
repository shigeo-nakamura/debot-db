use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum TrendType {
    Up,
    Down,
    Any,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Serialize, Deserialize)]
pub enum TradingStrategy {
    RandomWalk(TrendType),
    MeanReversion(TrendType),
    RandomFlashCrashDetector(TrendType),
    FlashCrashDetector(TrendType),
}

impl TradingStrategy {
    pub fn trend_type(&self) -> &TrendType {
        match self {
            TradingStrategy::RandomWalk(t)
            | TradingStrategy::MeanReversion(t)
            | TradingStrategy::RandomFlashCrashDetector(t)
            | TradingStrategy::FlashCrashDetector(t) => t,
        }
    }
}

impl PartialEq for TradingStrategy {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                TradingStrategy::RandomWalk(TrendType::Any),
                TradingStrategy::RandomWalk(TrendType::Up),
            )
            | (
                TradingStrategy::RandomWalk(TrendType::Up),
                TradingStrategy::RandomWalk(TrendType::Any),
            )
            | (
                TradingStrategy::RandomWalk(TrendType::Any),
                TradingStrategy::RandomWalk(TrendType::Down),
            )
            | (
                TradingStrategy::RandomWalk(TrendType::Down),
                TradingStrategy::RandomWalk(TrendType::Any),
            ) => true,
            (TradingStrategy::RandomWalk(t1), TradingStrategy::RandomWalk(t2)) if t1 == t2 => true,

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
                TradingStrategy::RandomFlashCrashDetector(TrendType::Any),
                TradingStrategy::RandomFlashCrashDetector(TrendType::Up),
            )
            | (
                TradingStrategy::RandomFlashCrashDetector(TrendType::Up),
                TradingStrategy::RandomFlashCrashDetector(TrendType::Any),
            )
            | (
                TradingStrategy::RandomFlashCrashDetector(TrendType::Any),
                TradingStrategy::RandomFlashCrashDetector(TrendType::Down),
            )
            | (
                TradingStrategy::RandomFlashCrashDetector(TrendType::Down),
                TradingStrategy::RandomFlashCrashDetector(TrendType::Any),
            ) => true,
            (
                TradingStrategy::RandomFlashCrashDetector(t1),
                TradingStrategy::RandomFlashCrashDetector(t2),
            ) if t1 == t2 => true,

            (
                TradingStrategy::FlashCrashDetector(TrendType::Any),
                TradingStrategy::FlashCrashDetector(TrendType::Up),
            )
            | (
                TradingStrategy::FlashCrashDetector(TrendType::Up),
                TradingStrategy::FlashCrashDetector(TrendType::Any),
            )
            | (
                TradingStrategy::FlashCrashDetector(TrendType::Any),
                TradingStrategy::FlashCrashDetector(TrendType::Down),
            )
            | (
                TradingStrategy::FlashCrashDetector(TrendType::Down),
                TradingStrategy::FlashCrashDetector(TrendType::Any),
            ) => true,
            (TradingStrategy::FlashCrashDetector(t1), TradingStrategy::FlashCrashDetector(t2))
                if t1 == t2 =>
            {
                true
            }

            _ => false,
        }
    }
}
