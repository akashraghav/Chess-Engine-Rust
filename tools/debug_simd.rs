use chess_core::utils::{OptimizedBitboard, SimdBitboard};
use chess_core::board::Bitboard;

fn main() {
    let a = [
        Bitboard::new(0xFF00000000000000),
        Bitboard::new(0x00FF000000000000),
        Bitboard::new(0x0000FF0000000000),
        Bitboard::new(0x000000FF00000000),
    ];
    let b = [
        Bitboard::new(0xF0F0000000000000),
        Bitboard::new(0x0F0F000000000000),
        Bitboard::new(0x00F0F00000000000),
        Bitboard::new(0x000F0F0000000000),
    ];

    println!("Input a: {:?}", a.iter().map(|bb| format!("0x{:016X}", bb.value())).collect::<Vec<_>>());
    println!("Input b: {:?}", b.iter().map(|bb| format!("0x{:016X}", bb.value())).collect::<Vec<_>>());

    // Test fallback directly
    let fallback_result = SimdBitboard::parallel_and_4_fallback(&a, &b);
    println!("Fallback result: {:?}", fallback_result.iter().map(|bb| format!("0x{:016X}", bb.value())).collect::<Vec<_>>());

    // Test OptimizedBitboard
    let optimized_result = OptimizedBitboard::batch_and_4(&a, &b);
    println!("Optimized result: {:?}", optimized_result.iter().map(|bb| format!("0x{:016X}", bb.value())).collect::<Vec<_>>());

    // Test manual AND operations
    let manual_result = [
        a[0] & b[0],
        a[1] & b[1], 
        a[2] & b[2],
        a[3] & b[3],
    ];
    println!("Manual result: {:?}", manual_result.iter().map(|bb| format!("0x{:016X}", bb.value())).collect::<Vec<_>>());
}