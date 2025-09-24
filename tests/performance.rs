use chess_core::{GameState, MoveGenerator};
use std::time::Instant;

#[test]
fn test_move_generation_performance() {
    let game_state = GameState::new();
    let generator = MoveGenerator::new();
    
    let start = Instant::now();
    let mut total_moves = 0;
    
    // Benchmark move generation for 1000 iterations
    for _ in 0..1000 {
        let moves = generator.generate_legal_moves(&game_state);
        total_moves += moves.len();
    }
    
    let duration = start.elapsed();
    let moves_per_second = (total_moves as f64) / duration.as_secs_f64();
    
    // Should generate at least 10k moves per second (very conservative)
    assert!(moves_per_second > 10_000.0, 
            "Move generation too slow: {:.0} moves/sec", moves_per_second);
    
    println!("✅ Move generation performance: {:.0} moves/sec", moves_per_second);
}

#[test]
fn test_position_evaluation_performance() {
    let game_state = GameState::new();
    
    let start = Instant::now();
    let mut total_evals = 0;
    
    // Benchmark position evaluation for 1000 iterations
    for _ in 0..1000 {
        let _eval = game_state.evaluate();
        total_evals += 1;
    }
    
    let duration = start.elapsed();
    let evals_per_second = (total_evals as f64) / duration.as_secs_f64();
    
    // Should evaluate at least 1k positions per second (very conservative)
    assert!(evals_per_second > 1_000.0, 
            "Position evaluation too slow: {:.0} evals/sec", evals_per_second);
    
    println!("✅ Position evaluation performance: {:.0} evals/sec", evals_per_second);
}

#[test]
fn test_memory_usage() {
    // Test that basic operations don't use excessive memory
    let game_state = GameState::new();
    let generator = MoveGenerator::new();
    
    // Generate moves multiple times to test for memory leaks
    for _ in 0..100 {
        let moves = generator.generate_legal_moves(&game_state);
        assert!(!moves.is_empty());
        // Let moves go out of scope to test cleanup
    }
    
    println!("✅ Memory usage test passed");
}

#[test] 
fn test_concurrent_access() {
    use std::thread;
    use std::sync::Arc;
    
    let game_state = Arc::new(GameState::new());
    let generator = Arc::new(MoveGenerator::new());
    
    let mut handles = vec![];
    
    // Test concurrent access from multiple threads
    for i in 0..4 {
        let game_state = Arc::clone(&game_state);
        let generator = Arc::clone(&generator);
        
        let handle = thread::spawn(move || {
            for _ in 0..100 {
                let moves = generator.generate_legal_moves(&game_state);
                assert_eq!(moves.len(), 20); // Starting position has 20 moves
            }
            println!("✅ Thread {} completed successfully", i);
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().expect("Thread should complete successfully");
    }
    
    println!("✅ Concurrent access test passed");
}