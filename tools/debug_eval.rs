#[cfg(test)]
mod debug_eval {
    use crate::{GameState, Evaluator};

    #[test]
    fn debug_evaluation_components() {
        let evaluator = Evaluator::new();
        let game_state = GameState::new();

        let material = evaluator.material_balance(&game_state);
        let positional = evaluator.positional_score(&game_state);
        let mobility = evaluator.mobility_score(&game_state);
        let king_safety = evaluator.king_safety_score(&game_state);
        let pawn_structure = evaluator.pawn_structure_score(&game_state);

        println!("Material: {}", material);
        println!("Positional: {}", positional);
        println!("Mobility: {}", mobility);
        println!("King safety: {}", king_safety);
        println!("Pawn structure: {}", pawn_structure);
        
        let total = material + positional + mobility + king_safety + pawn_structure;
        println!("Total: {}", total);

        assert!(false); // Force print output
    }
}
