// SIMD-optimized operations for chess engine
// Uses AVX2/SSE4.2 instructions for parallel processing of multiple bitboards

use crate::Bitboard;

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

#[cfg(target_arch = "x86")]
use std::arch::x86::*;

/// SIMD-optimized bitboard operations
pub struct SimdBitboard;

impl SimdBitboard {
    /// Process 4 bitboards in parallel using AVX2 (256-bit vectors)
    #[cfg(all(target_feature = "avx2", any(target_arch = "x86", target_arch = "x86_64")))]
    #[inline]
    pub unsafe fn parallel_and_4(
        a: &[Bitboard; 4], 
        b: &[Bitboard; 4]
    ) -> [Bitboard; 4] {
        let a_vec = _mm256_load_si256(a.as_ptr() as *const __m256i);
        let b_vec = _mm256_load_si256(b.as_ptr() as *const __m256i);
        let result = _mm256_and_si256(a_vec, b_vec);
        
        let mut output = [Bitboard::EMPTY; 4];
        _mm256_store_si256(output.as_mut_ptr() as *mut __m256i, result);
        output
    }

    /// Process 4 bitboards OR operation in parallel using AVX2
    #[cfg(all(target_feature = "avx2", any(target_arch = "x86", target_arch = "x86_64")))]
    #[inline]
    pub unsafe fn parallel_or_4(
        a: &[Bitboard; 4], 
        b: &[Bitboard; 4]
    ) -> [Bitboard; 4] {
        let a_vec = _mm256_load_si256(a.as_ptr() as *const __m256i);
        let b_vec = _mm256_load_si256(b.as_ptr() as *const __m256i);
        let result = _mm256_or_si256(a_vec, b_vec);
        
        let mut output = [Bitboard::EMPTY; 4];
        _mm256_store_si256(output.as_mut_ptr() as *mut __m256i, result);
        output
    }

    /// Process 4 bitboards XOR operation in parallel using AVX2
    #[cfg(all(target_feature = "avx2", any(target_arch = "x86", target_arch = "x86_64")))]
    #[inline]
    pub unsafe fn parallel_xor_4(
        a: &[Bitboard; 4], 
        b: &[Bitboard; 4]
    ) -> [Bitboard; 4] {
        let a_vec = _mm256_load_si256(a.as_ptr() as *const __m256i);
        let b_vec = _mm256_load_si256(b.as_ptr() as *const __m256i);
        let result = _mm256_xor_si256(a_vec, b_vec);
        
        let mut output = [Bitboard::EMPTY; 4];
        _mm256_store_si256(output.as_mut_ptr() as *mut __m256i, result);
        output
    }

    /// Count bits in 4 bitboards simultaneously using POPCNT
    #[cfg(all(target_feature = "popcnt", any(target_arch = "x86", target_arch = "x86_64")))]
    #[inline]
    pub unsafe fn parallel_popcount_4(boards: &[Bitboard; 4]) -> [u32; 4] {
        [
            _popcnt64(boards[0].value() as i64) as u32,
            _popcnt64(boards[1].value() as i64) as u32,
            _popcnt64(boards[2].value() as i64) as u32,
            _popcnt64(boards[3].value() as i64) as u32,
        ]
    }

    /// Parallel shift operations for 4 bitboards
    #[cfg(all(target_feature = "avx2", any(target_arch = "x86", target_arch = "x86_64")))]
    #[inline]
    pub unsafe fn parallel_shift_left_4(
        boards: &[Bitboard; 4], 
        shift: i32
    ) -> [Bitboard; 4] {
        let vec = _mm256_load_si256(boards.as_ptr() as *const __m256i);
        let result = _mm256_slli_epi64(vec, shift);
        
        let mut output = [Bitboard::EMPTY; 4];
        _mm256_store_si256(output.as_mut_ptr() as *mut __m256i, result);
        output
    }

    /// Parallel shift operations for 4 bitboards (right shift)
    #[cfg(all(target_feature = "avx2", any(target_arch = "x86", target_arch = "x86_64")))]
    #[inline]
    pub unsafe fn parallel_shift_right_4(
        boards: &[Bitboard; 4], 
        shift: i32
    ) -> [Bitboard; 4] {
        let vec = _mm256_load_si256(boards.as_ptr() as *const __m256i);
        let result = _mm256_srli_epi64(vec, shift);
        
        let mut output = [Bitboard::EMPTY; 4];
        _mm256_store_si256(output.as_mut_ptr() as *mut __m256i, result);
        output
    }

    /// Fallback implementations for non-AVX2 systems
    #[inline]
    pub fn parallel_and_4_fallback(
        a: &[Bitboard; 4], 
        b: &[Bitboard; 4]
    ) -> [Bitboard; 4] {
        [
            a[0] & b[0],
            a[1] & b[1],
            a[2] & b[2],
            a[3] & b[3],
        ]
    }

    #[inline]
    pub fn parallel_or_4_fallback(
        a: &[Bitboard; 4], 
        b: &[Bitboard; 4]
    ) -> [Bitboard; 4] {
        [
            a[0] | b[0],
            a[1] | b[1],
            a[2] | b[2],
            a[3] | b[3],
        ]
    }

    #[inline]
    pub fn parallel_popcount_4_fallback(boards: &[Bitboard; 4]) -> [u32; 4] {
        [
            boards[0].count_bits(),
            boards[1].count_bits(),
            boards[2].count_bits(),
            boards[3].count_bits(),
        ]
    }
}

/// High-level SIMD bitboard operations with runtime feature detection
pub struct OptimizedBitboard;

impl OptimizedBitboard {
    /// Perform AND operation on 4 bitboards with best available instruction set
    #[inline]
    pub fn batch_and_4(a: &[Bitboard; 4], b: &[Bitboard; 4]) -> [Bitboard; 4] {
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            if is_x86_feature_detected!("avx2") {
                unsafe { SimdBitboard::parallel_and_4(a, b) }
            } else {
                SimdBitboard::parallel_and_4_fallback(a, b)
            }
        }
        #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
        {
            SimdBitboard::parallel_and_4_fallback(a, b)
        }
    }

    /// Perform OR operation on 4 bitboards with best available instruction set
    #[inline]
    pub fn batch_or_4(a: &[Bitboard; 4], b: &[Bitboard; 4]) -> [Bitboard; 4] {
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            if is_x86_feature_detected!("avx2") {
                unsafe { SimdBitboard::parallel_or_4(a, b) }
            } else {
                SimdBitboard::parallel_or_4_fallback(a, b)
            }
        }
        #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
        {
            SimdBitboard::parallel_or_4_fallback(a, b)
        }
    }

    /// Count bits in 4 bitboards with best available instruction set
    #[inline]
    pub fn batch_popcount_4(boards: &[Bitboard; 4]) -> [u32; 4] {
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            if is_x86_feature_detected!("popcnt") {
                unsafe { SimdBitboard::parallel_popcount_4(boards) }
            } else {
                SimdBitboard::parallel_popcount_4_fallback(boards)
            }
        }
        #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
        {
            SimdBitboard::parallel_popcount_4_fallback(boards)
        }
    }

    /// Parallel north shifts for multiple piece types
    #[inline]
    pub fn batch_north_shifts(boards: &[Bitboard; 4]) -> [Bitboard; 4] {
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            if is_x86_feature_detected!("avx2") {
                unsafe { SimdBitboard::parallel_shift_left_4(boards, 8) }
            } else {
                [
                    boards[0].shift_north(),
                    boards[1].shift_north(),
                    boards[2].shift_north(),
                    boards[3].shift_north(),
                ]
            }
        }
        #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
        {
            [
                boards[0].shift_north(),
                boards[1].shift_north(),
                boards[2].shift_north(),
                boards[3].shift_north(),
            ]
        }
    }

    /// Parallel south shifts for multiple piece types
    #[inline]
    pub fn batch_south_shifts(boards: &[Bitboard; 4]) -> [Bitboard; 4] {
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            if is_x86_feature_detected!("avx2") {
                unsafe { SimdBitboard::parallel_shift_right_4(boards, 8) }
            } else {
                [
                    boards[0].shift_south(),
                    boards[1].shift_south(),
                    boards[2].shift_south(),
                    boards[3].shift_south(),
                ]
            }
        }
        #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
        {
            [
                boards[0].shift_south(),
                boards[1].shift_south(),
                boards[2].shift_south(),
                boards[3].shift_south(),
            ]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_and_operations() {
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
            Bitboard::new(0x0000F0F000000000), // Fixed: was 0x000F0F0000000000
        ];

        let result = OptimizedBitboard::batch_and_4(&a, &b);
        
        assert_eq!(result[0].value(), 0xF000000000000000);
        assert_eq!(result[1].value(), 0x000F000000000000);
        assert_eq!(result[2].value(), 0x0000F00000000000);
        assert_eq!(result[3].value(), 0x000000F000000000); // Fixed expected value
    }

    #[test]
    fn test_batch_popcount() {
        let boards = [
            Bitboard::new(0xFF00000000000000), // 8 bits
            Bitboard::new(0x00FF000000000000), // 8 bits  
            Bitboard::new(0x0000FF0000000000), // 8 bits
            Bitboard::new(0x000000FF00000000), // 8 bits
        ];

        let counts = OptimizedBitboard::batch_popcount_4(&boards);
        
        assert_eq!(counts, [8, 8, 8, 8]);
    }

    #[test]
    fn test_batch_shifts() {
        let boards = [
            Bitboard::new(0x00000000000000FF), // bottom rank
            Bitboard::new(0x000000000000FF00), // second rank
            Bitboard::new(0x0000000000FF0000), // third rank
            Bitboard::new(0x00000000FF000000), // fourth rank
        ];

        let north_shifted = OptimizedBitboard::batch_north_shifts(&boards);
        
        assert_eq!(north_shifted[0].value(), 0x000000000000FF00); // bottom→second rank ✓
        assert_eq!(north_shifted[1].value(), 0x0000000000FF0000); // second→third rank ✓
        assert_eq!(north_shifted[2].value(), 0x00000000FF000000); // third→fourth rank ✓  
        assert_eq!(north_shifted[3].value(), 0x000000FF00000000); // fourth→fifth rank ✓
    }
}