pub mod engine;
pub mod parallel;

pub use engine::{SearchEngine, SearchConfig, SearchResult};
pub use parallel::{ParallelConfig, ParallelSearchEngine, ParallelMoveGenerator, ParallelEvaluator};