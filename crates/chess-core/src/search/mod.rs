pub mod engine;
pub mod parallel;

pub use engine::{SearchConfig, SearchEngine, SearchResult};
pub use parallel::{
    ParallelConfig, ParallelEvaluator, ParallelMoveGenerator, ParallelSearchEngine,
};
