use arrayref::{array_mut_ref, array_ref};
use core::mem::size_of;
use paste::paste;

use crate::{pack, seq_t, unpack, FastLanes};

/// `BitPack` into a compile-time known bit-width.
pub trait BitPacking: FastLanes {
    /* 
    /// Packs 1024 elements into W bits each.
    /// The output is given as Self to ensure correct alignment.
    fn pack<const W: usize>(input: &[Self; 1024], output: &mut [Self; 1024 * W / Self::T])
    where
        BitPackWidth<W>: SupportedBitPackWidth<Self>;
    */

    /// Packs 1024 elements into `W` bits each, where `W` is runtime-known instead of
    /// compile-time known.
    ///
    /// # Safety
    /// The input slice must be of exactly length 1024. The output slice must be of length
    /// `1024 * W / T`, where `T` is the bit-width of Self and `W` is the packed width.
    /// These lengths are checked only with `debug_assert` (i.e., not checked on release builds).
    unsafe fn unchecked_pack(width: usize, input: &[Self], output: &mut [Self]);

    /* 
    /// Unpacks 1024 elements from `W` bits each.
    fn unpack<const W: usize>(input: &[Self; 1024 * W / Self::T], output: &mut [Self; 1024])
    where
        BitPackWidth<W>: SupportedBitPackWidth<Self>;
    */
    /// Unpacks 1024 elements from `W` bits each, where `W` is runtime-known instead of
    /// compile-time known.
    ///
    /// # Safety
    /// The input slice must be of length `1024 * W / T`, where `T` is the bit-width of Self and `W`
    /// is the packed width. The output slice must be of exactly length 1024.
    /// These lengths are checked only with `debug_assert` (i.e., not checked on release builds).
    unsafe fn unchecked_unpack(width: usize, input: &[Self], output: &mut [Self]);

    /* 
    /// Unpacks a single element at the provided index from a packed array of 1024 `W` bit elements.
    fn unpack_single<const W: usize>(packed: &[Self; 1024 * W / Self::T], index: usize) -> Self
    where
        BitPackWidth<W>: SupportedBitPackWidth<Self>;

    /// Unpacks a single element at the provided index from a packed array of 1024 `W` bit elements,
    /// where `W` is runtime-known instead of compile-time known.
    ///
    /// # Safety
    /// The input slice must be of length `1024 * W / T`, where `T` is the bit-width of Self and `W`
    /// is the packed width. The output slice must be of exactly length 1024.
    /// These lengths are checked only with `debug_assert` (i.e., not checked on release builds).
    unsafe fn unchecked_unpack_single(width: usize, input: &[Self], index: usize) -> Self;
    */
}

impl BitPacking for u8 {
    unsafe fn unchecked_pack(width: usize, input: &[Self], output: &mut [Self]) {
        let packed_len = 128 * width / size_of::<Self>();
        debug_assert_eq!(output.len(), packed_len, "Output buffer must be of size 1024 * W / T");
        debug_assert_eq!(input.len(), 1024, "Input buffer must be of size 1024");
        debug_assert!(width <= Self::T, "Width must be less than or equal to {}", Self::T);

        match width {
            1 => pack_8_1(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 1 / u8::T]),
            2 => pack_8_2(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 2 / u8::T]),
            3 => pack_8_3(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 3 / u8::T]),
            4 => pack_8_4(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 4 / u8::T]),
            5 => pack_8_5(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 5 / u8::T]),
            6 => pack_8_6(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 6 / u8::T]),
            7 => pack_8_7(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 7 / u8::T]),
            8 => pack_8_8(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 8 / u8::T]),
            _ => unreachable!("Unsupported width: {}", width)
        }
    }

    unsafe fn unchecked_unpack(width: usize, input: &[Self], output: &mut [Self]) {
        let packed_len = 128 * width / size_of::<Self>();
        debug_assert_eq!(input.len(), packed_len, "Input buffer must be of size 1024 * W / T");
        debug_assert_eq!(output.len(), 1024, "Output buffer must be of size 1024");
        debug_assert!(width <= Self::T, "Width must be less than or equal to {}", Self::T);

        match width {
            1 => unpack_8_1(array_ref![input, 0, 1024 * 1 / u8::T], array_mut_ref![output, 0, 1024]),
            2 => unpack_8_2(array_ref![input, 0, 1024 * 2 / u8::T], array_mut_ref![output, 0, 1024]),
            3 => unpack_8_3(array_ref![input, 0, 1024 * 3 / u8::T], array_mut_ref![output, 0, 1024]),
            4 => unpack_8_4(array_ref![input, 0, 1024 * 4 / u8::T], array_mut_ref![output, 0, 1024]),
            5 => unpack_8_5(array_ref![input, 0, 1024 * 5 / u8::T], array_mut_ref![output, 0, 1024]),
            6 => unpack_8_6(array_ref![input, 0, 1024 * 6 / u8::T], array_mut_ref![output, 0, 1024]),
            7 => unpack_8_7(array_ref![input, 0, 1024 * 7 / u8::T], array_mut_ref![output, 0, 1024]),
            8 => unpack_8_8(array_ref![input, 0, 1024 * 8 / u8::T], array_mut_ref![output, 0, 1024]),
            _ => unreachable!("Unsupported width: {}", width)
        }
    }
}

impl BitPacking for u16 {
    unsafe fn unchecked_pack(width: usize, input: &[Self], output: &mut [Self]) {
        let packed_len = 128 * width / size_of::<Self>();
        debug_assert_eq!(output.len(), packed_len, "Output buffer must be of size 1024 * W / T");
        debug_assert_eq!(input.len(), 1024, "Input buffer must be of size 1024");
        debug_assert!(width <= Self::T, "Width must be less than or equal to {}", Self::T);

        match width {
            1 => pack_16_1(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 1 / u16::T]),
            2 => pack_16_2(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 2 / u16::T]),
            3 => pack_16_3(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 3 / u16::T]),
            4 => pack_16_4(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 4 / u16::T]),
            5 => pack_16_5(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 5 / u16::T]),
            6 => pack_16_6(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 6 / u16::T]),
            7 => pack_16_7(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 7 / u16::T]),
            8 => pack_16_8(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 8 / u16::T]),
            9 => pack_16_9(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 9 / u16::T]),

            10 => pack_16_10(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 10 / u16::T]),
            11 => pack_16_11(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 11 / u16::T]),
            12 => pack_16_12(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 12 / u16::T]),
            13 => pack_16_13(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 13 / u16::T]),
            14 => pack_16_14(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 14 / u16::T]),
            15 => pack_16_15(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 15 / u16::T]),
            16 => pack_16_16(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 16 / u16::T]),
            _ => unreachable!("Unsupported width: {}", width)
        }
    }

    unsafe fn unchecked_unpack(width: usize, input: &[Self], output: &mut [Self]) {
        let packed_len = 128 * width / size_of::<Self>();
        debug_assert_eq!(input.len(), packed_len, "Input buffer must be of size 1024 * W / T");
        debug_assert_eq!(output.len(), 1024, "Output buffer must be of size 1024");
        debug_assert!(width <= Self::T, "Width must be less than or equal to {}", Self::T);

        match width {
            1 => unpack_16_1(array_ref![input, 0, 1024 * 1 / u16::T], array_mut_ref![output, 0, 1024]),
            2 => unpack_16_2(array_ref![input, 0, 1024 * 2 / u16::T], array_mut_ref![output, 0, 1024]),
            3 => unpack_16_3(array_ref![input, 0, 1024 * 3 / u16::T], array_mut_ref![output, 0, 1024]),
            4 => unpack_16_4(array_ref![input, 0, 1024 * 4 / u16::T], array_mut_ref![output, 0, 1024]),
            5 => unpack_16_5(array_ref![input, 0, 1024 * 5 / u16::T], array_mut_ref![output, 0, 1024]),
            6 => unpack_16_6(array_ref![input, 0, 1024 * 6 / u16::T], array_mut_ref![output, 0, 1024]),
            7 => unpack_16_7(array_ref![input, 0, 1024 * 7 / u16::T], array_mut_ref![output, 0, 1024]),
            8 => unpack_16_8(array_ref![input, 0, 1024 * 8 / u16::T], array_mut_ref![output, 0, 1024]),
            9 => unpack_16_9(array_ref![input, 0, 1024 * 9 / u16::T], array_mut_ref![output, 0, 1024]),
            10 => unpack_16_10(array_ref![input, 0, 1024 * 10 / u16::T], array_mut_ref![output, 0, 1024]),
            11 => unpack_16_11(array_ref![input, 0, 1024 * 11 / u16::T], array_mut_ref![output, 0, 1024]),
            12 => unpack_16_12(array_ref![input, 0, 1024 * 12 / u16::T], array_mut_ref![output, 0, 1024]),
            13 => unpack_16_13(array_ref![input, 0, 1024 * 13 / u16::T], array_mut_ref![output, 0, 1024]),
            14 => unpack_16_14(array_ref![input, 0, 1024 * 14 / u16::T], array_mut_ref![output, 0, 1024]),
            15 => unpack_16_15(array_ref![input, 0, 1024 * 15 / u16::T], array_mut_ref![output, 0, 1024]),
            16 => unpack_16_16(array_ref![input, 0, 1024 * 16 / u16::T], array_mut_ref![output, 0, 1024]),
            _ => unreachable!("Unsupported width: {}", width)
        }
    }
}

impl BitPacking for u32 {
    unsafe fn unchecked_pack(width: usize, input: &[Self], output: &mut [Self]) {
        let packed_len = 128 * width / size_of::<Self>();
        debug_assert_eq!(output.len(), packed_len, "Output buffer must be of size 1024 * W / T");
        debug_assert_eq!(input.len(), 1024, "Input buffer must be of size 1024");
        debug_assert!(width <= Self::T, "Width must be less than or equal to {}", Self::T);

        match width {
            1 => pack_32_1(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 1 / u32::T]),
            2 => pack_32_2(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 2 / u32::T]),
            3 => pack_32_3(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 3 / u32::T]),
            4 => pack_32_4(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 4 / u32::T]),
            5 => pack_32_5(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 5 / u32::T]),
            6 => pack_32_6(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 6 / u32::T]),
            7 => pack_32_7(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 7 / u32::T]),
            8 => pack_32_8(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 8 / u32::T]),
            9 => pack_32_9(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 9 / u32::T]),
            10 => pack_32_10(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 10 / u32::T]),
            11 => pack_32_11(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 11 / u32::T]),
            12 => pack_32_12(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 12 / u32::T]),
            13 => pack_32_13(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 13 / u32::T]),
            14 => pack_32_14(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 14 / u32::T]),
            15 => pack_32_15(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 15 / u32::T]),
            16 => pack_32_16(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 16 / u32::T]),
            17 => pack_32_17(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 17 / u32::T]),
            18 => pack_32_18(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 18 / u32::T]),
            19 => pack_32_19(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 19 / u32::T]),
            20 => pack_32_20(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 20 / u32::T]),
            21 => pack_32_21(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 21 / u32::T]),
            22 => pack_32_22(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 22 / u32::T]),
            23 => pack_32_23(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 23 / u32::T]),
            24 => pack_32_24(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 24 / u32::T]),
            25 => pack_32_25(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 25 / u32::T]),
            26 => pack_32_26(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 26 / u32::T]),
            27 => pack_32_27(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 27 / u32::T]),
            28 => pack_32_28(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 28 / u32::T]),
            29 => pack_32_29(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 29 / u32::T]),
            30 => pack_32_30(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 30 / u32::T]),
            31 => pack_32_31(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 31 / u32::T]),
            32 => pack_32_32(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 32 / u32::T]),
            _ => unreachable!("Unsupported width: {}", width)
        }
    }

    unsafe fn unchecked_unpack(width: usize, input: &[Self], output: &mut [Self]) {
        let packed_len = 128 * width / size_of::<Self>();
        debug_assert_eq!(input.len(), packed_len, "Input buffer must be of size 1024 * W / T");
        debug_assert_eq!(output.len(), 1024, "Output buffer must be of size 1024");
        debug_assert!(width <= Self::T, "Width must be less than or equal to {}", Self::T);

        match width {
            1 => unpack_32_1(array_ref![input, 0, 1024 * 1 / u32::T], array_mut_ref![output, 0, 1024]),
            2 => unpack_32_2(array_ref![input, 0, 1024 * 2 / u32::T], array_mut_ref![output, 0, 1024]),
            3 => unpack_32_3(array_ref![input, 0, 1024 * 3 / u32::T], array_mut_ref![output, 0, 1024]),
            4 => unpack_32_4(array_ref![input, 0, 1024 * 4 / u32::T], array_mut_ref![output, 0, 1024]),
            5 => unpack_32_5(array_ref![input, 0, 1024 * 5 / u32::T], array_mut_ref![output, 0, 1024]),
            6 => unpack_32_6(array_ref![input, 0, 1024 * 6 / u32::T], array_mut_ref![output, 0, 1024]),
            7 => unpack_32_7(array_ref![input, 0, 1024 * 7 / u32::T], array_mut_ref![output, 0, 1024]),
            8 => unpack_32_8(array_ref![input, 0, 1024 * 8 / u32::T], array_mut_ref![output, 0, 1024]),
            9 => unpack_32_9(array_ref![input, 0, 1024 * 9 / u32::T], array_mut_ref![output, 0, 1024]),
            10 => unpack_32_10(array_ref![input, 0, 1024 * 10 / u32::T], array_mut_ref![output, 0, 1024]),
            11 => unpack_32_11(array_ref![input, 0, 1024 * 11 / u32::T], array_mut_ref![output, 0, 1024]),
            12 => unpack_32_12(array_ref![input, 0, 1024 * 12 / u32::T], array_mut_ref![output, 0, 1024]),
            13 => unpack_32_13(array_ref![input, 0, 1024 * 13 / u32::T], array_mut_ref![output, 0, 1024]),
            14 => unpack_32_14(array_ref![input, 0, 1024 * 14 / u32::T], array_mut_ref![output, 0, 1024]),
            15 => unpack_32_15(array_ref![input, 0, 1024 * 15 / u32::T], array_mut_ref![output, 0, 1024]),
            16 => unpack_32_16(array_ref![input, 0, 1024 * 16 / u32::T], array_mut_ref![output, 0, 1024]),
            17 => unpack_32_17(array_ref![input, 0, 1024 * 17 / u32::T], array_mut_ref![output, 0, 1024]),
            18 => unpack_32_18(array_ref![input, 0, 1024 * 18 / u32::T], array_mut_ref![output, 0, 1024]),
            19 => unpack_32_19(array_ref![input, 0, 1024 * 19 / u32::T], array_mut_ref![output, 0, 1024]),
            20 => unpack_32_20(array_ref![input, 0, 1024 * 20 / u32::T], array_mut_ref![output, 0, 1024]),
            21 => unpack_32_21(array_ref![input, 0, 1024 * 21 / u32::T], array_mut_ref![output, 0, 1024]),
            22 => unpack_32_22(array_ref![input, 0, 1024 * 22 / u32::T], array_mut_ref![output, 0, 1024]),
            23 => unpack_32_23(array_ref![input, 0, 1024 * 23 / u32::T], array_mut_ref![output, 0, 1024]),
            24 => unpack_32_24(array_ref![input, 0, 1024 * 24 / u32::T], array_mut_ref![output, 0, 1024]),
            25 => unpack_32_25(array_ref![input, 0, 1024 * 25 / u32::T], array_mut_ref![output, 0, 1024]),
            26 => unpack_32_26(array_ref![input, 0, 1024 * 26 / u32::T], array_mut_ref![output, 0, 1024]),
            27 => unpack_32_27(array_ref![input, 0, 1024 * 27 / u32::T], array_mut_ref![output, 0, 1024]),
            28 => unpack_32_28(array_ref![input, 0, 1024 * 28 / u32::T], array_mut_ref![output, 0, 1024]),
            29 => unpack_32_29(array_ref![input, 0, 1024 * 29 / u32::T], array_mut_ref![output, 0, 1024]),
            30 => unpack_32_30(array_ref![input, 0, 1024 * 30 / u32::T], array_mut_ref![output, 0, 1024]),
            31 => unpack_32_31(array_ref![input, 0, 1024 * 31 / u32::T], array_mut_ref![output, 0, 1024]),
            32 => unpack_32_32(array_ref![input, 0, 1024 * 32 / u32::T], array_mut_ref![output, 0, 1024]),
            _ => unreachable!("Unsupported width: {}", width)
        }
    }
}

impl BitPacking for u64 {
    unsafe fn unchecked_pack(width: usize, input: &[Self], output: &mut [Self]) {
        let packed_len = 128 * width / size_of::<Self>();
        debug_assert_eq!(output.len(), packed_len, "Output buffer must be of size 1024 * W / T");
        debug_assert_eq!(input.len(), 1024, "Input buffer must be of size 1024");
        debug_assert!(width <= Self::T, "Width must be less than or equal to {}", Self::T);

        match width {
            1 => pack_64_1(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 1 / u64::T]),
            2 => pack_64_2(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 2 / u64::T]),
            3 => pack_64_3(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 3 / u64::T]),
            4 => pack_64_4(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 4 / u64::T]),
            5 => pack_64_5(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 5 / u64::T]),
            6 => pack_64_6(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 6 / u64::T]),
            7 => pack_64_7(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 7 / u64::T]),
            8 => pack_64_8(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 8 / u64::T]),
            9 => pack_64_9(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 9 / u64::T]),
            10 => pack_64_10(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 10 / u64::T]),
            11 => pack_64_11(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 11 / u64::T]),
            12 => pack_64_12(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 12 / u64::T]),
            13 => pack_64_13(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 13 / u64::T]),
            14 => pack_64_14(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 14 / u64::T]),
            15 => pack_64_15(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 15 / u64::T]),
            16 => pack_64_16(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 16 / u64::T]),
            17 => pack_64_17(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 17 / u64::T]),
            18 => pack_64_18(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 18 / u64::T]),
            19 => pack_64_19(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 19 / u64::T]),
            20 => pack_64_20(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 20 / u64::T]),
            21 => pack_64_21(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 21 / u64::T]),
            22 => pack_64_22(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 22 / u64::T]),
            23 => pack_64_23(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 23 / u64::T]),
            24 => pack_64_24(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 24 / u64::T]),
            25 => pack_64_25(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 25 / u64::T]),
            26 => pack_64_26(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 26 / u64::T]),
            27 => pack_64_27(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 27 / u64::T]),
            28 => pack_64_28(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 28 / u64::T]),
            29 => pack_64_29(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 29 / u64::T]),
            30 => pack_64_30(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 30 / u64::T]),
            31 => pack_64_31(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 31 / u64::T]),
            32 => pack_64_32(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 32 / u64::T]),
            33 => pack_64_33(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 33 / u64::T]),
            34 => pack_64_34(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 34 / u64::T]),
            35 => pack_64_35(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 35 / u64::T]),
            36 => pack_64_36(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 36 / u64::T]),
            37 => pack_64_37(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 37 / u64::T]),
            38 => pack_64_38(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 38 / u64::T]),
            39 => pack_64_39(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 39 / u64::T]),
            40 => pack_64_40(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 40 / u64::T]),
            41 => pack_64_41(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 41 / u64::T]),
            42 => pack_64_42(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 42 / u64::T]),
            43 => pack_64_43(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 43 / u64::T]),
            44 => pack_64_44(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 44 / u64::T]),
            45 => pack_64_45(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 45 / u64::T]),
            46 => pack_64_46(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 46 / u64::T]),
            47 => pack_64_47(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 47 / u64::T]),
            48 => pack_64_48(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 48 / u64::T]),
            49 => pack_64_49(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 49 / u64::T]),
            50 => pack_64_50(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 50 / u64::T]),
            51 => pack_64_51(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 51 / u64::T]),
            52 => pack_64_52(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 52 / u64::T]),
            53 => pack_64_53(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 53 / u64::T]),
            54 => pack_64_54(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 54 / u64::T]),
            55 => pack_64_55(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 55 / u64::T]),
            56 => pack_64_56(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 56 / u64::T]),
            57 => pack_64_57(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 57 / u64::T]),
            58 => pack_64_58(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 58 / u64::T]),
            59 => pack_64_59(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 59 / u64::T]),
            60 => pack_64_60(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 60 / u64::T]),
            61 => pack_64_61(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 61 / u64::T]),
            62 => pack_64_62(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 62 / u64::T]),
            63 => pack_64_63(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 63 / u64::T]),
            64 => pack_64_64(array_ref![input, 0, 1024], array_mut_ref![output, 0, 1024 * 64 / u64::T]),
            _ => unreachable!("Unsupported width: {}", width)
        }
    }

    unsafe fn unchecked_unpack(width: usize, input: &[Self], output: &mut [Self]) {
        let packed_len = 128 * width / size_of::<Self>();
        debug_assert_eq!(input.len(), packed_len, "Input buffer must be of size 1024 * W / T");
        debug_assert_eq!(output.len(), 1024, "Output buffer must be of size 1024");
        debug_assert!(width <= Self::T, "Width must be less than or equal to {}", Self::T);

        match width {
            1 => unpack_64_1(array_ref![input, 0, 1024 * 1 / u64::T], array_mut_ref![output, 0, 1024]),
            2 => unpack_64_2(array_ref![input, 0, 1024 * 2 / u64::T], array_mut_ref![output, 0, 1024]),
            3 => unpack_64_3(array_ref![input, 0, 1024 * 3 / u64::T], array_mut_ref![output, 0, 1024]),
            4 => unpack_64_4(array_ref![input, 0, 1024 * 4 / u64::T], array_mut_ref![output, 0, 1024]),
            5 => unpack_64_5(array_ref![input, 0, 1024 * 5 / u64::T], array_mut_ref![output, 0, 1024]),
            6 => unpack_64_6(array_ref![input, 0, 1024 * 6 / u64::T], array_mut_ref![output, 0, 1024]),
            7 => unpack_64_7(array_ref![input, 0, 1024 * 7 / u64::T], array_mut_ref![output, 0, 1024]),
            8 => unpack_64_8(array_ref![input, 0, 1024 * 8 / u64::T], array_mut_ref![output, 0, 1024]),
            9 => unpack_64_9(array_ref![input, 0, 1024 * 9 / u64::T], array_mut_ref![output, 0, 1024]),
            10 => unpack_64_10(array_ref![input, 0, 1024 * 10 / u64::T], array_mut_ref![output, 0, 1024]),
            11 => unpack_64_11(array_ref![input, 0, 1024 * 11 / u64::T], array_mut_ref![output, 0, 1024]),
            12 => unpack_64_12(array_ref![input, 0, 1024 * 12 / u64::T], array_mut_ref![output, 0, 1024]),
            13 => unpack_64_13(array_ref![input, 0, 1024 * 13 / u64::T], array_mut_ref![output, 0, 1024]),
            14 => unpack_64_14(array_ref![input, 0, 1024 * 14 / u64::T], array_mut_ref![output, 0, 1024]),
            15 => unpack_64_15(array_ref![input, 0, 1024 * 15 / u64::T], array_mut_ref![output, 0, 1024]),
            16 => unpack_64_16(array_ref![input, 0, 1024 * 16 / u64::T], array_mut_ref![output, 0, 1024]),
            17 => unpack_64_17(array_ref![input, 0, 1024 * 17 / u64::T], array_mut_ref![output, 0, 1024]),
            18 => unpack_64_18(array_ref![input, 0, 1024 * 18 / u64::T], array_mut_ref![output, 0, 1024]),
            19 => unpack_64_19(array_ref![input, 0, 1024 * 19 / u64::T], array_mut_ref![output, 0, 1024]),
            20 => unpack_64_20(array_ref![input, 0, 1024 * 20 / u64::T], array_mut_ref![output, 0, 1024]),
            21 => unpack_64_21(array_ref![input, 0, 1024 * 21 / u64::T], array_mut_ref![output, 0, 1024]),
            22 => unpack_64_22(array_ref![input, 0, 1024 * 22 / u64::T], array_mut_ref![output, 0, 1024]),
            23 => unpack_64_23(array_ref![input, 0, 1024 * 23 / u64::T], array_mut_ref![output, 0, 1024]),
            24 => unpack_64_24(array_ref![input, 0, 1024 * 24 / u64::T], array_mut_ref![output, 0, 1024]),
            25 => unpack_64_25(array_ref![input, 0, 1024 * 25 / u64::T], array_mut_ref![output, 0, 1024]),
            26 => unpack_64_26(array_ref![input, 0, 1024 * 26 / u64::T], array_mut_ref![output, 0, 1024]),
            27 => unpack_64_27(array_ref![input, 0, 1024 * 27 / u64::T], array_mut_ref![output, 0, 1024]),
            28 => unpack_64_28(array_ref![input, 0, 1024 * 28 / u64::T], array_mut_ref![output, 0, 1024]),
            29 => unpack_64_29(array_ref![input, 0, 1024 * 29 / u64::T], array_mut_ref![output, 0, 1024]),
            30 => unpack_64_30(array_ref![input, 0, 1024 * 30 / u64::T], array_mut_ref![output, 0, 1024]),
            31 => unpack_64_31(array_ref![input, 0, 1024 * 31 / u64::T], array_mut_ref![output, 0, 1024]),
            32 => unpack_64_32(array_ref![input, 0, 1024 * 32 / u64::T], array_mut_ref![output, 0, 1024]),
            33 => unpack_64_33(array_ref![input, 0, 1024 * 33 / u64::T], array_mut_ref![output, 0, 1024]),
            34 => unpack_64_34(array_ref![input, 0, 1024 * 34 / u64::T], array_mut_ref![output, 0, 1024]),
            35 => unpack_64_35(array_ref![input, 0, 1024 * 35 / u64::T], array_mut_ref![output, 0, 1024]),
            36 => unpack_64_36(array_ref![input, 0, 1024 * 36 / u64::T], array_mut_ref![output, 0, 1024]),
            37 => unpack_64_37(array_ref![input, 0, 1024 * 37 / u64::T], array_mut_ref![output, 0, 1024]),
            38 => unpack_64_38(array_ref![input, 0, 1024 * 38 / u64::T], array_mut_ref![output, 0, 1024]),
            39 => unpack_64_39(array_ref![input, 0, 1024 * 39 / u64::T], array_mut_ref![output, 0, 1024]),
            40 => unpack_64_40(array_ref![input, 0, 1024 * 40 / u64::T], array_mut_ref![output, 0, 1024]),
            41 => unpack_64_41(array_ref![input, 0, 1024 * 41 / u64::T], array_mut_ref![output, 0, 1024]),
            42 => unpack_64_42(array_ref![input, 0, 1024 * 42 / u64::T], array_mut_ref![output, 0, 1024]),
            43 => unpack_64_43(array_ref![input, 0, 1024 * 43 / u64::T], array_mut_ref![output, 0, 1024]),
            44 => unpack_64_44(array_ref![input, 0, 1024 * 44 / u64::T], array_mut_ref![output, 0, 1024]),
            45 => unpack_64_45(array_ref![input, 0, 1024 * 45 / u64::T], array_mut_ref![output, 0, 1024]),
            46 => unpack_64_46(array_ref![input, 0, 1024 * 46 / u64::T], array_mut_ref![output, 0, 1024]),
            47 => unpack_64_47(array_ref![input, 0, 1024 * 47 / u64::T], array_mut_ref![output, 0, 1024]),
            48 => unpack_64_48(array_ref![input, 0, 1024 * 48 / u64::T], array_mut_ref![output, 0, 1024]),
            49 => unpack_64_49(array_ref![input, 0, 1024 * 49 / u64::T], array_mut_ref![output, 0, 1024]),
            50 => unpack_64_50(array_ref![input, 0, 1024 * 50 / u64::T], array_mut_ref![output, 0, 1024]),
            51 => unpack_64_51(array_ref![input, 0, 1024 * 51 / u64::T], array_mut_ref![output, 0, 1024]),
            52 => unpack_64_52(array_ref![input, 0, 1024 * 52 / u64::T], array_mut_ref![output, 0, 1024]),
            53 => unpack_64_53(array_ref![input, 0, 1024 * 53 / u64::T], array_mut_ref![output, 0, 1024]),
            54 => unpack_64_54(array_ref![input, 0, 1024 * 54 / u64::T], array_mut_ref![output, 0, 1024]),
            55 => unpack_64_55(array_ref![input, 0, 1024 * 55 / u64::T], array_mut_ref![output, 0, 1024]),
            56 => unpack_64_56(array_ref![input, 0, 1024 * 56 / u64::T], array_mut_ref![output, 0, 1024]),
            57 => unpack_64_57(array_ref![input, 0, 1024 * 57 / u64::T], array_mut_ref![output, 0, 1024]),
            58 => unpack_64_58(array_ref![input, 0, 1024 * 58 / u64::T], array_mut_ref![output, 0, 1024]),
            59 => unpack_64_59(array_ref![input, 0, 1024 * 59 / u64::T], array_mut_ref![output, 0, 1024]),
            60 => unpack_64_60(array_ref![input, 0, 1024 * 60 / u64::T], array_mut_ref![output, 0, 1024]),
            61 => unpack_64_61(array_ref![input, 0, 1024 * 61 / u64::T], array_mut_ref![output, 0, 1024]),
            62 => unpack_64_62(array_ref![input, 0, 1024 * 62 / u64::T], array_mut_ref![output, 0, 1024]),
            63 => unpack_64_63(array_ref![input, 0, 1024 * 63 / u64::T], array_mut_ref![output, 0, 1024]),
            64 => unpack_64_64(array_ref![input, 0, 1024 * 64 / u64::T], array_mut_ref![output, 0, 1024]),
            _ => unreachable!("Unsupported width: {}", width)
        }
    }
}

macro_rules! unpack_8 {
    ($name:ident, $bits:expr) => {
        fn $name(input: &[u8; 1024 * $bits / u8::T], output: &mut [u8; 1024]) {
            for lane in 0..u8::LANES {
                unpack!(u8, $bits, input, lane, |$idx, $elem| {
                    output[$idx] = $elem;
                });
            }
        }
    };
}

// Now generate the functions using the macro
unpack_8!(unpack_8_1, 1);
unpack_8!(unpack_8_2, 2);
unpack_8!(unpack_8_3, 3);
unpack_8!(unpack_8_4, 4);
unpack_8!(unpack_8_5, 5);
unpack_8!(unpack_8_6, 6);
unpack_8!(unpack_8_7, 7);
unpack_8!(unpack_8_8, 8);
macro_rules! pack_8 {
    ($name:ident, $bits:expr) => {
        fn $name(input: &[u8; 1024], output: &mut [u8; 1024 * $bits / u8::T]) {
            for lane in 0..u8::LANES {
                pack!(u8, $bits, output, lane, |$idx| {
                    input[$idx]
                });
            }
        }
    };
}

// Now generate the functions using the macro
pack_8!(pack_8_1, 1);
pack_8!(pack_8_2, 2);
pack_8!(pack_8_3, 3);
pack_8!(pack_8_4, 4);
pack_8!(pack_8_5, 5);
pack_8!(pack_8_6, 6);
pack_8!(pack_8_7, 7);
pack_8!(pack_8_8, 8);

/* 
fn unpack_8_1(input: &[u8; 1024 * 1 / u8::T], output: &mut [u8; 1024]) {
    for lane in 0..u8::LANES {
        unpack!(u8, 1, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_8_2(input: &[u8; 1024 * 2 / u8::T], output: &mut [u8; 1024]) {
    for lane in 0..u8::LANES {
        unpack!(u8, 2, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_8_3(input: &[u8; 1024 * 3 / u8::T], output: &mut [u8; 1024]) {
    for lane in 0..u8::LANES {
        unpack!(u8, 3, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_8_4(input: &[u8; 1024 * 4 / u8::T], output: &mut [u8; 1024]) {
    for lane in 0..u8::LANES {
        unpack!(u8, 4, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_8_5(input: &[u8; 1024 * 5 / u8::T], output: &mut [u8; 1024]) {
    for lane in 0..u8::LANES {
        unpack!(u8, 5, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_8_6(input: &[u8; 1024 * 6 / u8::T], output: &mut [u8; 1024]) {
    for lane in 0..u8::LANES {
        unpack!(u8, 6, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_8_7(input: &[u8; 1024 * 7 / u8::T], output: &mut [u8; 1024]) {
    for lane in 0..u8::LANES {
        unpack!(u8, 7, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_8_8(input: &[u8; 1024 * 8 / u8::T], output: &mut [u8; 1024]) {
    for lane in 0..u8::LANES {
        unpack!(u8, 8, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}
*/

/* 
fn pack_8_1(input: &[u8; 1024], output: &mut [u8; 1024 * 1 / u8::T]) {
    for lane in 0..u8::LANES {
        pack!(u8, 1, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_8_2(input: &[u8; 1024], output: &mut [u8; 1024 * 2 / u8::T]) {
    for lane in 0..u8::LANES {
        pack!(u8, 2, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_8_3(input: &[u8; 1024], output: &mut [u8; 1024 * 3 / u8::T]) {
    for lane in 0..u8::LANES {
        pack!(u8, 3, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_8_4(input: &[u8; 1024], output: &mut [u8; 1024 * 4 / u8::T]) {
    for lane in 0..u8::LANES {
        pack!(u8, 4, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_8_5(input: &[u8; 1024], output: &mut [u8; 1024 * 5 / u8::T]) {
    for lane in 0..u8::LANES {
        pack!(u8, 5, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_8_6(input: &[u8; 1024], output: &mut [u8; 1024 * 6 / u8::T]) {
    for lane in 0..u8::LANES {
        pack!(u8, 6, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_8_7(input: &[u8; 1024], output: &mut [u8; 1024 * 7 / u8::T]) {
    for lane in 0..u8::LANES {
        pack!(u8, 7, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_8_8(input: &[u8; 1024], output: &mut [u8; 1024 * 8 / u8::T]) {
    for lane in 0..u8::LANES {
        pack!(u8, 8, output, lane, |$idx| {
            input[$idx]
        });
    }
}
*/

macro_rules! unpack_16 {
    ($name:ident, $bits:expr) => {
        fn $name(input: &[u16; 1024 * $bits / u16::T], output: &mut [u16; 1024]) {
            for lane in 0..u16::LANES {
                unpack!(u16, $bits, input, lane, |$idx, $elem| {
                    output[$idx] = $elem;
                });
            }
        }
    };
}

// Now generate the functions using the macro
unpack_16!(unpack_16_1, 1);
unpack_16!(unpack_16_2, 2);
unpack_16!(unpack_16_3, 3);
unpack_16!(unpack_16_4, 4);
unpack_16!(unpack_16_5, 5);
unpack_16!(unpack_16_6, 6);
unpack_16!(unpack_16_7, 7);
unpack_16!(unpack_16_8, 8);
unpack_16!(unpack_16_9, 9);
unpack_16!(unpack_16_10, 10);
unpack_16!(unpack_16_11, 11);
unpack_16!(unpack_16_12, 12);
unpack_16!(unpack_16_13, 13);
unpack_16!(unpack_16_14, 14);
unpack_16!(unpack_16_15, 15);
unpack_16!(unpack_16_16, 16);

/* 
fn unpack_16_1(input: &[u16; 1024 * 1 / u16::T], output: &mut [u16; 1024]) {
    for lane in 0..u16::LANES {
        unpack!(u16, 1, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_16_2(input: &[u16; 1024 * 2 / u16::T], output: &mut [u16; 1024]) {
    for lane in 0..u16::LANES {
        unpack!(u16, 2, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_16_3(input: &[u16; 1024 * 3 / u16::T], output: &mut [u16; 1024]) {
    for lane in 0..u16::LANES {
        unpack!(u16, 3, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_16_4(input: &[u16; 1024 * 4 / u16::T], output: &mut [u16; 1024]) {
    for lane in 0..u16::LANES {
        unpack!(u16, 4, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_16_5(input: &[u16; 1024 * 5 / u16::T], output: &mut [u16; 1024]) {
    for lane in 0..u16::LANES {
        unpack!(u16, 5, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_16_6(input: &[u16; 1024 * 6 / u16::T], output: &mut [u16; 1024]) {
    for lane in 0..u16::LANES {
        unpack!(u16, 6, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_16_7(input: &[u16; 1024 * 7 / u16::T], output: &mut [u16; 1024]) {
    for lane in 0..u16::LANES {
        unpack!(u16, 7, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_16_8(input: &[u16; 1024 * 8 / u16::T], output: &mut [u16; 1024]) {
    for lane in 0..u16::LANES {
        unpack!(u16, 8, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_16_9(input: &[u16; 1024 * 9 / u16::T], output: &mut [u16; 1024]) {
    for lane in 0..u16::LANES {
        unpack!(u16, 9, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_16_10(input: &[u16; 1024 * 10 / u16::T], output: &mut [u16; 1024]) {
    for lane in 0..u16::LANES {
        unpack!(u16, 10, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_16_11(input: &[u16; 1024 * 11 / u16::T], output: &mut [u16; 1024]) {
    for lane in 0..u16::LANES {
        unpack!(u16, 11, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_16_12(input: &[u16; 1024 * 12 / u16::T], output: &mut [u16; 1024]) {
    for lane in 0..u16::LANES {
        unpack!(u16, 12, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_16_13(input: &[u16; 1024 * 13 / u16::T], output: &mut [u16; 1024]) {
    for lane in 0..u16::LANES {
        unpack!(u16, 13, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_16_14(input: &[u16; 1024 * 14 / u16::T], output: &mut [u16; 1024]) {
    for lane in 0..u16::LANES {
        unpack!(u16, 14, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_16_15(input: &[u16; 1024 * 15 / u16::T], output: &mut [u16; 1024]) {
    for lane in 0..u16::LANES {
        unpack!(u16, 15, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_16_16(input: &[u16; 1024 * 16 / u16::T], output: &mut [u16; 1024]) {
    for lane in 0..u16::LANES {
        unpack!(u16, 16, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}
*/
macro_rules! pack_16 {
    ($name:ident, $bits:expr) => {
        fn $name(input: &[u16; 1024], output: &mut [u16; 1024 * $bits / u16::T]) {
            for lane in 0..u16::LANES {
                pack!(u16, $bits, output, lane, |$idx| {
                    input[$idx]
                });
            }
        }
    };
}

// Now use the macro to define all the functions
pack_16!(pack_16_1, 1);
pack_16!(pack_16_2, 2);
pack_16!(pack_16_3, 3);
pack_16!(pack_16_4, 4);
pack_16!(pack_16_5, 5);
pack_16!(pack_16_6, 6);
pack_16!(pack_16_7, 7);
pack_16!(pack_16_8, 8);
pack_16!(pack_16_9, 9);
pack_16!(pack_16_10, 10);
pack_16!(pack_16_11, 11);
pack_16!(pack_16_12, 12);
pack_16!(pack_16_13, 13);
pack_16!(pack_16_14, 14);
pack_16!(pack_16_15, 15);
pack_16!(pack_16_16, 16);

/* 
fn pack_16_1(input: &[u16; 1024], output: &mut [u16; 1024 * 1 / u16::T]) {
    for lane in 0..u16::LANES {
        pack!(u16, 1, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_16_2(input: &[u16; 1024], output: &mut [u16; 1024 * 2 / u16::T]) {
    for lane in 0..u16::LANES {
        pack!(u16, 2, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_16_3(input: &[u16; 1024], output: &mut [u16; 1024 * 3 / u16::T]) {
    for lane in 0..u16::LANES {
        pack!(u16, 3, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_16_4(input: &[u16; 1024], output: &mut [u16; 1024 * 4 / u16::T]) {
    for lane in 0..u16::LANES {
        pack!(u16, 4, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_16_5(input: &[u16; 1024], output: &mut [u16; 1024 * 5 / u16::T]) {
    for lane in 0..u16::LANES {
        pack!(u16, 5, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_16_6(input: &[u16; 1024], output: &mut [u16; 1024 * 6 / u16::T]) {
    for lane in 0..u16::LANES {
        pack!(u16, 6, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_16_7(input: &[u16; 1024], output: &mut [u16; 1024 * 7 / u16::T]) {
    for lane in 0..u16::LANES {
        pack!(u16, 7, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_16_8(input: &[u16; 1024], output: &mut [u16; 1024 * 8 / u16::T]) {
    for lane in 0..u16::LANES {
        pack!(u16, 8, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_16_9(input: &[u16; 1024], output: &mut [u16; 1024 * 9 / u16::T]) {
    for lane in 0..u16::LANES {
        pack!(u16, 9, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_16_10(input: &[u16; 1024], output: &mut [u16; 1024 * 10 / u16::T]) {
    for lane in 0..u16::LANES {
        pack!(u16, 10, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_16_11(input: &[u16; 1024], output: &mut [u16; 1024 * 11 / u16::T]) {
    for lane in 0..u16::LANES {
        pack!(u16, 11, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_16_12(input: &[u16; 1024], output: &mut [u16; 1024 * 12 / u16::T]) {
    for lane in 0..u16::LANES {
        pack!(u16, 12, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_16_13(input: &[u16; 1024], output: &mut [u16; 1024 * 13 / u16::T]) {
    for lane in 0..u16::LANES {
        pack!(u16, 13, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_16_14(input: &[u16; 1024], output: &mut [u16; 1024 * 14 / u16::T]) {
    for lane in 0..u16::LANES {
        pack!(u16, 14, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_16_15(input: &[u16; 1024], output: &mut [u16; 1024 * 15 / u16::T]) {
    for lane in 0..u16::LANES {
        pack!(u16, 15, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_16_16(input: &[u16; 1024], output: &mut [u16; 1024 * 16 / u16::T]) {
    for lane in 0..u16::LANES {
        pack!(u16, 16, output, lane, |$idx| {
            input[$idx]
        });
    }
}
*/
macro_rules! unpack_32 {
    ($name:ident, $bit_width:expr) => {
        fn $name(input: &[u32; 1024 * $bit_width / u32::T], output: &mut [u32; 1024]) {
            for lane in 0..u32::LANES {
                unpack!(u32, $bit_width, input, lane, |$idx, $elem| {
                    output[$idx] = $elem
                });
            }
        }
    };
}

// Using the macro to create the unpack functions
unpack_32!(unpack_32_1, 1);
unpack_32!(unpack_32_2, 2);
unpack_32!(unpack_32_3, 3);
unpack_32!(unpack_32_4, 4);
unpack_32!(unpack_32_5, 5);
unpack_32!(unpack_32_6, 6);
unpack_32!(unpack_32_7, 7);
unpack_32!(unpack_32_8, 8);
unpack_32!(unpack_32_9, 9);
unpack_32!(unpack_32_10, 10);
unpack_32!(unpack_32_11, 11);
unpack_32!(unpack_32_12, 12);
unpack_32!(unpack_32_13, 13);
unpack_32!(unpack_32_14, 14);
unpack_32!(unpack_32_15, 15);
unpack_32!(unpack_32_16, 16);
unpack_32!(unpack_32_17, 17);
unpack_32!(unpack_32_18, 18);
unpack_32!(unpack_32_19, 19);
unpack_32!(unpack_32_20, 20);
unpack_32!(unpack_32_21, 21);
unpack_32!(unpack_32_22, 22);
unpack_32!(unpack_32_23, 23);
unpack_32!(unpack_32_24, 24);
unpack_32!(unpack_32_25, 25);
unpack_32!(unpack_32_26, 26);
unpack_32!(unpack_32_27, 27);
unpack_32!(unpack_32_28, 28);
unpack_32!(unpack_32_29, 29);
unpack_32!(unpack_32_30, 30);
unpack_32!(unpack_32_31, 31);
unpack_32!(unpack_32_32, 32);

/* 
fn unpack_32_1(input: &[u32; 1024 * 1 / u32::T], output: &mut [u32; 1024]) {
    for lane in 0..u32::LANES {
        unpack!(u32, 1, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_32_2(input: &[u32; 1024 * 2 / u32::T], output: &mut [u32; 1024]) {
    for lane in 0..u32::LANES {
        unpack!(u32, 2, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_32_3(input: &[u32; 1024 * 3 / u32::T], output: &mut [u32; 1024]) {
    for lane in 0..u32::LANES {
        unpack!(u32, 3, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_32_4(input: &[u32; 1024 * 4 / u32::T], output: &mut [u32; 1024]) {
    for lane in 0..u32::LANES {
        unpack!(u32, 4, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_32_5(input: &[u32; 1024 * 5 / u32::T], output: &mut [u32; 1024]) {
    for lane in 0..u32::LANES {
        unpack!(u32, 5, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_32_6(input: &[u32; 1024 * 6 / u32::T], output: &mut [u32; 1024]) {
    for lane in 0..u32::LANES {
        unpack!(u32, 6, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_32_7(input: &[u32; 1024 * 7 / u32::T], output: &mut [u32; 1024]) {
    for lane in 0..u32::LANES {
        unpack!(u32, 7, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_32_8(input: &[u32; 1024 * 8 / u32::T], output: &mut [u32; 1024]) {
    for lane in 0..u32::LANES {
        unpack!(u32, 8, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_32_9(input: &[u32; 1024 * 9 / u32::T], output: &mut [u32; 1024]) {
    for lane in 0..u32::LANES {
        unpack!(u32, 9, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_32_10(input: &[u32; 1024 * 10 / u32::T], output: &mut [u32; 1024]) {
    for lane in 0..u32::LANES {
        unpack!(u32, 10, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_32_11(input: &[u32; 1024 * 11 / u32::T], output: &mut [u32; 1024]) {
    for lane in 0..u32::LANES {
        unpack!(u32, 11, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_32_12(input: &[u32; 1024 * 12 / u32::T], output: &mut [u32; 1024]) {
    for lane in 0..u32::LANES {
        unpack!(u32, 12, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_32_13(input: &[u32; 1024 * 13 / u32::T], output: &mut [u32; 1024]) {
    for lane in 0..u32::LANES {
        unpack!(u32, 13, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_32_14(input: &[u32; 1024 * 14 / u32::T], output: &mut [u32; 1024]) {
    for lane in 0..u32::LANES {
        unpack!(u32, 14, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_32_15(input: &[u32; 1024 * 15 / u32::T], output: &mut [u32; 1024]) {
    for lane in 0..u32::LANES {
        unpack!(u32, 15, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_32_16(input: &[u32; 1024 * 16 / u32::T], output: &mut [u32; 1024]) {
    for lane in 0..u32::LANES {
        unpack!(u32, 16, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_32_17(input: &[u32; 1024 * 17 / u32::T], output: &mut [u32; 1024]) {
    for lane in 0..u32::LANES {
        unpack!(u32, 17, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_32_18(input: &[u32; 1024 * 18 / u32::T], output: &mut [u32; 1024]) {
    for lane in 0..u32::LANES {
        unpack!(u32, 18, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_32_19(input: &[u32; 1024 * 19 / u32::T], output: &mut [u32; 1024]) {
    for lane in 0..u32::LANES {
        unpack!(u32, 19, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_32_20(input: &[u32; 1024 * 20 / u32::T], output: &mut [u32; 1024]) {
    for lane in 0..u32::LANES {
        unpack!(u32, 20, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_32_21(input: &[u32; 1024 * 21 / u32::T], output: &mut [u32; 1024]) {
    for lane in 0..u32::LANES {
        unpack!(u32, 21, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_32_22(input: &[u32; 1024 * 22 / u32::T], output: &mut [u32; 1024]) {
    for lane in 0..u32::LANES {
        unpack!(u32, 22, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_32_23(input: &[u32; 1024 * 23 / u32::T], output: &mut [u32; 1024]) {
    for lane in 0..u32::LANES {
        unpack!(u32, 23, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_32_24(input: &[u32; 1024 * 24 / u32::T], output: &mut [u32; 1024]) {
    for lane in 0..u32::LANES {
        unpack!(u32, 24, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_32_25(input: &[u32; 1024 * 25 / u32::T], output: &mut [u32; 1024]) {
    for lane in 0..u32::LANES {
        unpack!(u32, 25, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_32_26(input: &[u32; 1024 * 26 / u32::T], output: &mut [u32; 1024]) {
    for lane in 0..u32::LANES {
        unpack!(u32, 26, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_32_27(input: &[u32; 1024 * 27 / u32::T], output: &mut [u32; 1024]) {
    for lane in 0..u32::LANES {
        unpack!(u32, 27, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_32_28(input: &[u32; 1024 * 28 / u32::T], output: &mut [u32; 1024]) {
    for lane in 0..u32::LANES {
        unpack!(u32, 28, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_32_29(input: &[u32; 1024 * 29 / u32::T], output: &mut [u32; 1024]) {
    for lane in 0..u32::LANES {
        unpack!(u32, 29, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_32_30(input: &[u32; 1024 * 30 / u32::T], output: &mut [u32; 1024]) {
    for lane in 0..u32::LANES {
        unpack!(u32, 30, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_32_31(input: &[u32; 1024 * 31 / u32::T], output: &mut [u32; 1024]) {
    for lane in 0..u32::LANES {
        unpack!(u32, 31, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_32_32(input: &[u32; 1024 * 32 / u32::T], output: &mut [u32; 1024]) {
    for lane in 0..u32::LANES {
        unpack!(u32, 32, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}
*/

macro_rules! pack_32 {
    ($name:ident, $bits:expr) => {
        fn $name(input: &[u32; 1024], output: &mut [u32; 1024 * $bits / u32::BITS as usize]) {
            for lane in 0..u32::LANES {
                pack!(u32, $bits, output, lane, |$idx| {
                    input[$idx]
                });
            }
        }
    };
}

pack_32!(pack_32_1, 1);
pack_32!(pack_32_2, 2);
pack_32!(pack_32_3, 3);
pack_32!(pack_32_4, 4);
pack_32!(pack_32_5, 5);
pack_32!(pack_32_6, 6);
pack_32!(pack_32_7, 7);
pack_32!(pack_32_8, 8);
pack_32!(pack_32_9, 9);
pack_32!(pack_32_10, 10);
pack_32!(pack_32_11, 11);
pack_32!(pack_32_12, 12);
pack_32!(pack_32_13, 13);
pack_32!(pack_32_14, 14);
pack_32!(pack_32_15, 15);
pack_32!(pack_32_16, 16);
pack_32!(pack_32_17, 17);
pack_32!(pack_32_18, 18);
pack_32!(pack_32_19, 19);
pack_32!(pack_32_20, 20);
pack_32!(pack_32_21, 21);
pack_32!(pack_32_22, 22);
pack_32!(pack_32_23, 23);
pack_32!(pack_32_24, 24);
pack_32!(pack_32_25, 25);
pack_32!(pack_32_26, 26);
pack_32!(pack_32_27, 27);
pack_32!(pack_32_28, 28);
pack_32!(pack_32_29, 29);
pack_32!(pack_32_30, 30);
pack_32!(pack_32_31, 31);
pack_32!(pack_32_32, 32);

/* 
fn pack_32_1(input: &[u32; 1024], output: &mut [u32; 1024 * 1 / u32::T]) {
    for lane in 0..u32::LANES {
        pack!(u32, 1, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_32_2(input: &[u32; 1024], output: &mut [u32; 1024 * 2 / u32::T]) {
    for lane in 0..u32::LANES {
        pack!(u32, 2, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_32_3(input: &[u32; 1024], output: &mut [u32; 1024 * 3 / u32::T]) {
    for lane in 0..u32::LANES {
        pack!(u32, 3, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_32_4(input: &[u32; 1024], output: &mut [u32; 1024 * 4 / u32::T]) {
    for lane in 0..u32::LANES {
        pack!(u32, 4, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_32_5(input: &[u32; 1024], output: &mut [u32; 1024 * 5 / u32::T]) {
    for lane in 0..u32::LANES {
        pack!(u32, 5, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_32_6(input: &[u32; 1024], output: &mut [u32; 1024 * 6 / u32::T]) {
    for lane in 0..u32::LANES {
        pack!(u32, 6, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_32_7(input: &[u32; 1024], output: &mut [u32; 1024 * 7 / u32::T]) {
    for lane in 0..u32::LANES {
        pack!(u32, 7, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_32_8(input: &[u32; 1024], output: &mut [u32; 1024 * 8 / u32::T]) {
    for lane in 0..u32::LANES {
        pack!(u32, 8, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_32_9(input: &[u32; 1024], output: &mut [u32; 1024 * 9 / u32::T]) {
    for lane in 0..u32::LANES {
        pack!(u32, 9, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_32_10(input: &[u32; 1024], output: &mut [u32; 1024 * 10 / u32::T]) {
    for lane in 0..u32::LANES {
        pack!(u32, 10, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_32_11(input: &[u32; 1024], output: &mut [u32; 1024 * 11 / u32::T]) {
    for lane in 0..u32::LANES {
        pack!(u32, 11, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_32_12(input: &[u32; 1024], output: &mut [u32; 1024 * 12 / u32::T]) {
    for lane in 0..u32::LANES {
        pack!(u32, 12, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_32_13(input: &[u32; 1024], output: &mut [u32; 1024 * 13 / u32::T]) {
    for lane in 0..u32::LANES {
        pack!(u32, 13, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_32_14(input: &[u32; 1024], output: &mut [u32; 1024 * 14 / u32::T]) {
    for lane in 0..u32::LANES {
        pack!(u32, 14, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_32_15(input: &[u32; 1024], output: &mut [u32; 1024 * 15 / u32::T]) {
    for lane in 0..u32::LANES {
        pack!(u32, 15, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_32_16(input: &[u32; 1024], output: &mut [u32; 1024 * 16 / u32::T]) {
    for lane in 0..u32::LANES {
        pack!(u32, 16, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_32_17(input: &[u32; 1024], output: &mut [u32; 1024 * 17 / u32::T]) {
    for lane in 0..u32::LANES {
        pack!(u32, 17, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_32_18(input: &[u32; 1024], output: &mut [u32; 1024 * 18 / u32::T]) {
    for lane in 0..u32::LANES {
        pack!(u32, 18, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_32_19(input: &[u32; 1024], output: &mut [u32; 1024 * 19 / u32::T]) {
    for lane in 0..u32::LANES {
        pack!(u32, 19, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_32_20(input: &[u32; 1024], output: &mut [u32; 1024 * 20 / u32::T]) {
    for lane in 0..u32::LANES {
        pack!(u32, 20, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_32_21(input: &[u32; 1024], output: &mut [u32; 1024 * 21 / u32::T]) {
    for lane in 0..u32::LANES {
        pack!(u32, 21, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_32_22(input: &[u32; 1024], output: &mut [u32; 1024 * 22 / u32::T]) {
    for lane in 0..u32::LANES {
        pack!(u32, 22, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_32_23(input: &[u32; 1024], output: &mut [u32; 1024 * 23 / u32::T]) {
    for lane in 0..u32::LANES {
        pack!(u32, 23, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_32_24(input: &[u32; 1024], output: &mut [u32; 1024 * 24 / u32::T]) {
    for lane in 0..u32::LANES {
        pack!(u32, 24, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_32_25(input: &[u32; 1024], output: &mut [u32; 1024 * 25 / u32::T]) {
    for lane in 0..u32::LANES {
        pack!(u32, 25, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_32_26(input: &[u32; 1024], output: &mut [u32; 1024 * 26 / u32::T]) {
    for lane in 0..u32::LANES {
        pack!(u32, 26, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_32_27(input: &[u32; 1024], output: &mut [u32; 1024 * 27 / u32::T]) {
    for lane in 0..u32::LANES {
        pack!(u32, 27, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_32_28(input: &[u32; 1024], output: &mut [u32; 1024 * 28 / u32::T]) {
    for lane in 0..u32::LANES {
        pack!(u32, 28, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_32_29(input: &[u32; 1024], output: &mut [u32; 1024 * 29 / u32::T]) {
    for lane in 0..u32::LANES {
        pack!(u32, 29, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_32_30(input: &[u32; 1024], output: &mut [u32; 1024 * 30 / u32::T]) {
    for lane in 0..u32::LANES {
        pack!(u32, 30, output, lane, |$idx| {
            input[$idx]
        });
    }
}
fn pack_32_31(input: &[u32; 1024], output: &mut [u32; 1024 * 31 / u32::T]) {
    for lane in 0..u32::LANES {
        pack!(u32, 31, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_32_32(input: &[u32; 1024], output: &mut [u32; 1024 * 32 / u32::T]) {
    for lane in 0..u32::LANES {
        pack!(u32, 32, output, lane, |$idx| {
            input[$idx]
        });
    }
}
*/

macro_rules! generate_unpack_64 {
    ($($n:expr),*) => {
        $(
            paste::item! {
                fn [<unpack_64_ $n>](input: &[u64; 1024 * $n / u64::T], output: &mut [u64; 1024]) {
                    for lane in 0..u64::LANES {
                        unpack!(u64, $n, input, lane, |$idx, $elem| {
                            output[$idx] = $elem
                        });
                    }
                }
            }
        )*
    };
}

// Generate unpack functions for 1 to 64
generate_unpack_64!(
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
    17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32,
    33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48,
    49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64
);

macro_rules! generate_pack_64 {
    ($($n:expr),*) => {
        $(
            paste::item! {
                fn [<pack_64_ $n>](input: &[u64; 1024], output: &mut [u64; 1024 * $n / u64::T]) {
                    for lane in 0..u64::LANES {
                        pack!(u64, $n, output, lane, |$idx| {
                            input[$idx]
                        });
                    }
                }
            }
        )*
    };
}

// Generate pack functions for 1 to 64
generate_pack_64!(
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
    17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32,
    33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48,
    49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64
);

/* 
fn unpack_64_1(input: &[u64; 1024 * 1 / u64::T], output: &mut [u64; 1024]) {
    for lane in 0..u64::LANES {
        unpack!(u64, 1, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_64_2(input: &[u64; 1024 * 2 / u64::T], output: &mut [u64; 1024]) {
    for lane in 0..u64::LANES {
        unpack!(u64, 2, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_64_3(input: &[u64; 1024 * 3 / u64::T], output: &mut [u64; 1024]) {
    for lane in 0..u64::LANES {
        unpack!(u64, 3, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_64_4(input: &[u64; 1024 * 4 / u64::T], output: &mut [u64; 1024]) {
    for lane in 0..u64::LANES {
        unpack!(u64, 4, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_64_5(input: &[u64; 1024 * 5 / u64::T], output: &mut [u64; 1024]) {
    for lane in 0..u64::LANES {
        unpack!(u64, 5, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_64_6(input: &[u64; 1024 * 6 / u64::T], output: &mut [u64; 1024]) {
    for lane in 0..u64::LANES {
        unpack!(u64, 6, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_64_7(input: &[u64; 1024 * 7 / u64::T], output: &mut [u64; 1024]) {
    for lane in 0..u64::LANES {
        unpack!(u64, 7, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_64_8(input: &[u64; 1024 * 8 / u64::T], output: &mut [u64; 1024]) {
    for lane in 0..u64::LANES {
        unpack!(u64, 8, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_64_9(input: &[u64; 1024 * 9 / u64::T], output: &mut [u64; 1024]) {
    for lane in 0..u64::LANES {
        unpack!(u64, 9, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_64_10(input: &[u64; 1024 * 10 / u64::T], output: &mut [u64; 1024]) {
    for lane in 0..u64::LANES {
        unpack!(u64, 10, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_64_11(input: &[u64; 1024 * 11 / u64::T], output: &mut [u64; 1024]) {
    for lane in 0..u64::LANES {
        unpack!(u64, 11, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_64_12(input: &[u64; 1024 * 12 / u64::T], output: &mut [u64; 1024]) {
    for lane in 0..u64::LANES {
        unpack!(u64, 12, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_64_13(input: &[u64; 1024 * 13 / u64::T], output: &mut [u64; 1024]) {
    for lane in 0..u64::LANES {
        unpack!(u64, 13, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_64_14(input: &[u64; 1024 * 14 / u64::T], output: &mut [u64; 1024]) {
    for lane in 0..u64::LANES {
        unpack!(u64, 14, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_64_15(input: &[u64; 1024 * 15 / u64::T], output: &mut [u64; 1024]) {
    for lane in 0..u64::LANES {
        unpack!(u64, 15, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_64_16(input: &[u64; 1024 * 16 / u64::T], output: &mut [u64; 1024]) {
    for lane in 0..u64::LANES {
        unpack!(u64, 16, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_64_17(input: &[u64; 1024 * 17 / u64::T], output: &mut [u64; 1024]) {
    for lane in 0..u64::LANES {
        unpack!(u64, 17, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_64_18(input: &[u64; 1024 * 18 / u64::T], output: &mut [u64; 1024]) {
    for lane in 0..u64::LANES {
        unpack!(u64, 18, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_64_19(input: &[u64; 1024 * 19 / u64::T], output: &mut [u64; 1024]) {
    for lane in 0..u64::LANES {
        unpack!(u64, 19, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_64_20(input: &[u64; 1024 * 20 / u64::T], output: &mut [u64; 1024]) {
    for lane in 0..u64::LANES {
        unpack!(u64, 20, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_64_21(input: &[u64; 1024 * 21 / u64::T], output: &mut [u64; 1024]) {
    for lane in 0..u64::LANES {
        unpack!(u64, 21, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_64_22(input: &[u64; 1024 * 22 / u64::T], output: &mut [u64; 1024]) {
    for lane in 0..u64::LANES {
        unpack!(u64, 22, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_64_23(input: &[u64; 1024 * 23 / u64::T], output: &mut [u64; 1024]) {
    for lane in 0..u64::LANES {
        unpack!(u64, 23, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_64_24(input: &[u64; 1024 * 24 / u64::T], output: &mut [u64; 1024]) {
    for lane in 0..u64::LANES {
        unpack!(u64, 24, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_64_25(input: &[u64; 1024 * 25 / u64::T], output: &mut [u64; 1024]) {
    for lane in 0..u64::LANES {
        unpack!(u64, 25, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_64_26(input: &[u64; 1024 * 26 / u64::T], output: &mut [u64; 1024]) {
    for lane in 0..u64::LANES {
        unpack!(u64, 26, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_64_27(input: &[u64; 1024 * 27 / u64::T], output: &mut [u64; 1024]) {
    for lane in 0..u64::LANES {
        unpack!(u64, 27, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_64_28(input: &[u64; 1024 * 28 / u64::T], output: &mut [u64; 1024]) {
    for lane in 0..u64::LANES {
        unpack!(u64, 28, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_64_29(input: &[u64; 1024 * 29 / u64::T], output: &mut [u64; 1024]) {
    for lane in 0..u64::LANES {
        unpack!(u64, 29, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_64_30(input: &[u64; 1024 * 30 / u64::T], output: &mut [u64; 1024]) {
    for lane in 0..u64::LANES {
        unpack!(u64, 30, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_64_31(input: &[u64; 1024 * 31 / u64::T], output: &mut [u64; 1024]) {
    for lane in 0..u64::LANES {
        unpack!(u64, 31, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_64_32(input: &[u64; 1024 * 32 / u64::T], output: &mut [u64; 1024]) {
    for lane in 0..u64::LANES {
        unpack!(u64, 32, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_64_33(input: &[u64; 1024 * 33 / u64::T], output: &mut [u64; 1024]) {
    for lane in 0..u64::LANES {
        unpack!(u64, 33, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_64_34(input: &[u64; 1024 * 34 / u64::T], output: &mut [u64; 1024]) {
    for lane in 0..u64::LANES {
        unpack!(u64, 34, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_64_35(input: &[u64; 1024 * 35 / u64::T], output: &mut [u64; 1024]) {
    for lane in 0..u64::LANES {
        unpack!(u64, 35, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_64_36(input: &[u64; 1024 * 36 / u64::T], output: &mut [u64; 1024]) {
    for lane in 0..u64::LANES {
        unpack!(u64, 36, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_64_37(input: &[u64; 1024 * 37 / u64::T], output: &mut [u64; 1024]) {
    for lane in 0..u64::LANES {
        unpack!(u64, 37, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_64_38(input: &[u64; 1024 * 38 / u64::T], output: &mut [u64; 1024]) {
    for lane in 0..u64::LANES {
        unpack!(u64, 38, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_64_39(input: &[u64; 1024 * 39 / u64::T], output: &mut [u64; 1024]) {
    for lane in 0..u64::LANES {
        unpack!(u64, 39, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_64_40(input: &[u64; 1024 * 40 / u64::T], output: &mut [u64; 1024]) {
    for lane in 0..u64::LANES {
        unpack!(u64, 40, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_64_41(input: &[u64; 1024 * 41 / u64::T], output: &mut [u64; 1024]) {
    for lane in 0..u64::LANES {
        unpack!(u64, 41, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_64_42(input: &[u64; 1024 * 42 / u64::T], output: &mut [u64; 1024]) {
    for lane in 0..u64::LANES {
        unpack!(u64, 42, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_64_43(input: &[u64; 1024 * 43 / u64::T], output: &mut [u64; 1024]) {
    for lane in 0..u64::LANES {
        unpack!(u64, 43, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_64_44(input: &[u64; 1024 * 44 / u64::T], output: &mut [u64; 1024]) {
    for lane in 0..u64::LANES {
        unpack!(u64, 44, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_64_45(input: &[u64; 1024 * 45 / u64::T], output: &mut [u64; 1024]) {
    for lane in 0..u64::LANES {
        unpack!(u64, 45, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_64_46(input: &[u64; 1024 * 46 / u64::T], output: &mut [u64; 1024]) {
    for lane in 0..u64::LANES {
        unpack!(u64, 46, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_64_47(input: &[u64; 1024 * 47 / u64::T], output: &mut [u64; 1024]) {
    for lane in 0..u64::LANES {
        unpack!(u64, 47, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_64_48(input: &[u64; 1024 * 48 / u64::T], output: &mut [u64; 1024]) {
    for lane in 0..u64::LANES {
        unpack!(u64, 48, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_64_49(input: &[u64; 1024 * 49 / u64::T], output: &mut [u64; 1024]) {
    for lane in 0..u64::LANES {
        unpack!(u64, 49, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_64_50(input: &[u64; 1024 * 50 / u64::T], output: &mut [u64; 1024]) {
    for lane in 0..u64::LANES {
        unpack!(u64, 50, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_64_51(input: &[u64; 1024 * 51 / u64::T], output: &mut [u64; 1024]) {
    for lane in 0..u64::LANES {
        unpack!(u64, 51, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_64_52(input: &[u64; 1024 * 52 / u64::T], output: &mut [u64; 1024]) {
    for lane in 0..u64::LANES {
        unpack!(u64, 52, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_64_53(input: &[u64; 1024 * 53 / u64::T], output: &mut [u64; 1024]) {
    for lane in 0..u64::LANES {
        unpack!(u64, 53, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_64_54(input: &[u64; 1024 * 54 / u64::T], output: &mut [u64; 1024]) {
    for lane in 0..u64::LANES {
        unpack!(u64, 54, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_64_55(input: &[u64; 1024 * 55 / u64::T], output: &mut [u64; 1024]) {
    for lane in 0..u64::LANES {
        unpack!(u64, 55, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_64_56(input: &[u64; 1024 * 56 / u64::T], output: &mut [u64; 1024]) {
    for lane in 0..u64::LANES {
        unpack!(u64, 56, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_64_57(input: &[u64; 1024 * 57 / u64::T], output: &mut [u64; 1024]) {
    for lane in 0..u64::LANES {
        unpack!(u64, 57, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_64_58(input: &[u64; 1024 * 58 / u64::T], output: &mut [u64; 1024]) {
    for lane in 0..u64::LANES {
        unpack!(u64, 58, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_64_59(input: &[u64; 1024 * 59 / u64::T], output: &mut [u64; 1024]) {
    for lane in 0..u64::LANES {
        unpack!(u64, 59, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_64_60(input: &[u64; 1024 * 60 / u64::T], output: &mut [u64; 1024]) {
    for lane in 0..u64::LANES {
        unpack!(u64, 60, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_64_61(input: &[u64; 1024 * 61 / u64::T], output: &mut [u64; 1024]) {
    for lane in 0..u64::LANES {
        unpack!(u64, 61, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_64_62(input: &[u64; 1024 * 62 / u64::T], output: &mut [u64; 1024]) {
    for lane in 0..u64::LANES {
        unpack!(u64, 62, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_64_63(input: &[u64; 1024 * 63 / u64::T], output: &mut [u64; 1024]) {
    for lane in 0..u64::LANES {
        unpack!(u64, 63, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}

fn unpack_64_64(input: &[u64; 1024 * 64 / u64::T], output: &mut [u64; 1024]) {
    for lane in 0..u64::LANES {
        unpack!(u64, 64, input, lane, |$idx, $elem| {
            output[$idx] = $elem
        });
    }
}
*/

/* 
fn pack_64_1(input: &[u64; 1024], output: &mut [u64; 1024 * 1 / u64::T]) {
    for lane in 0..u64::LANES {
        pack!(u64, 1, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_64_2(input: &[u64; 1024], output: &mut [u64; 1024 * 2 / u64::T]) {
    for lane in 0..u64::LANES {
        pack!(u64, 2, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_64_3(input: &[u64; 1024], output: &mut [u64; 1024 * 3 / u64::T]) {
    for lane in 0..u64::LANES {
        pack!(u64, 3, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_64_4(input: &[u64; 1024], output: &mut [u64; 1024 * 4 / u64::T]) {
    for lane in 0..u64::LANES {
        pack!(u64, 4, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_64_5(input: &[u64; 1024], output: &mut [u64; 1024 * 5 / u64::T]) {
    for lane in 0..u64::LANES {
        pack!(u64, 5, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_64_6(input: &[u64; 1024], output: &mut [u64; 1024 * 6 / u64::T]) {
    for lane in 0..u64::LANES {
        pack!(u64, 6, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_64_7(input: &[u64; 1024], output: &mut [u64; 1024 * 7 / u64::T]) {
    for lane in 0..u64::LANES {
        pack!(u64, 7, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_64_8(input: &[u64; 1024], output: &mut [u64; 1024 * 8 / u64::T]) {
    for lane in 0..u64::LANES {
        pack!(u64, 8, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_64_9(input: &[u64; 1024], output: &mut [u64; 1024 * 9 / u64::T]) {
    for lane in 0..u64::LANES {
        pack!(u64, 9, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_64_10(input: &[u64; 1024], output: &mut [u64; 1024 * 10 / u64::T]) {
    for lane in 0..u64::LANES {
        pack!(u64, 10, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_64_11(input: &[u64; 1024], output: &mut [u64; 1024 * 11 / u64::T]) {
    for lane in 0..u64::LANES {
        pack!(u64, 11, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_64_12(input: &[u64; 1024], output: &mut [u64; 1024 * 12 / u64::T]) {
    for lane in 0..u64::LANES {
        pack!(u64, 12, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_64_13(input: &[u64; 1024], output: &mut [u64; 1024 * 13 / u64::T]) {
    for lane in 0..u64::LANES {
        pack!(u64, 13, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_64_14(input: &[u64; 1024], output: &mut [u64; 1024 * 14 / u64::T]) {
    for lane in 0..u64::LANES {
        pack!(u64, 14, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_64_15(input: &[u64; 1024], output: &mut [u64; 1024 * 15 / u64::T]) {
    for lane in 0..u64::LANES {
        pack!(u64, 15, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_64_16(input: &[u64; 1024], output: &mut [u64; 1024 * 16 / u64::T]) {
    for lane in 0..u64::LANES {
        pack!(u64, 16, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_64_17(input: &[u64; 1024], output: &mut [u64; 1024 * 17 / u64::T]) {
    for lane in 0..u64::LANES {
        pack!(u64, 17, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_64_18(input: &[u64; 1024], output: &mut [u64; 1024 * 18 / u64::T]) {
    for lane in 0..u64::LANES {
        pack!(u64, 18, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_64_19(input: &[u64; 1024], output: &mut [u64; 1024 * 19 / u64::T]) {
    for lane in 0..u64::LANES {
        pack!(u64, 19, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_64_20(input: &[u64; 1024], output: &mut [u64; 1024 * 20 / u64::T]) {
    for lane in 0..u64::LANES {
        pack!(u64, 20, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_64_21(input: &[u64; 1024], output: &mut [u64; 1024 * 21 / u64::T]) {
    for lane in 0..u64::LANES {
        pack!(u64, 21, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_64_22(input: &[u64; 1024], output: &mut [u64; 1024 * 22 / u64::T]) {
    for lane in 0..u64::LANES {
        pack!(u64, 22, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_64_23(input: &[u64; 1024], output: &mut [u64; 1024 * 23 / u64::T]) {
    for lane in 0..u64::LANES {
        pack!(u64, 23, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_64_24(input: &[u64; 1024], output: &mut [u64; 1024 * 24 / u64::T]) {
    for lane in 0..u64::LANES {
        pack!(u64, 24, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_64_25(input: &[u64; 1024], output: &mut [u64; 1024 * 25 / u64::T]) {
    for lane in 0..u64::LANES {
        pack!(u64, 25, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_64_26(input: &[u64; 1024], output: &mut [u64; 1024 * 26 / u64::T]) {
    for lane in 0..u64::LANES {
        pack!(u64, 26, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_64_27(input: &[u64; 1024], output: &mut [u64; 1024 * 27 / u64::T]) {
    for lane in 0..u64::LANES {
        pack!(u64, 27, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_64_28(input: &[u64; 1024], output: &mut [u64; 1024 * 28 / u64::T]) {
    for lane in 0..u64::LANES {
        pack!(u64, 28, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_64_29(input: &[u64; 1024], output: &mut [u64; 1024 * 29 / u64::T]) {
    for lane in 0..u64::LANES {
        pack!(u64, 29, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_64_30(input: &[u64; 1024], output: &mut [u64; 1024 * 30 / u64::T]) {
    for lane in 0..u64::LANES {
        pack!(u64, 30, output, lane, |$idx| {
            input[$idx]
        });
    }
}
fn pack_64_31(input: &[u64; 1024], output: &mut [u64; 1024 * 31 / u64::T]) {
    for lane in 0..u64::LANES {
        pack!(u64, 31, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_64_32(input: &[u64; 1024], output: &mut [u64; 1024 * 32 / u64::T]) {
    for lane in 0..u64::LANES {
        pack!(u64, 32, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_64_33(input: &[u64; 1024], output: &mut [u64; 1024 * 33 / u64::T]) {
    for lane in 0..u64::LANES {
        pack!(u64, 33, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_64_34(input: &[u64; 1024], output: &mut [u64; 1024 * 34 / u64::T]) {
    for lane in 0..u64::LANES {
        pack!(u64, 34, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_64_35(input: &[u64; 1024], output: &mut [u64; 1024 * 35 / u64::T]) {
    for lane in 0..u64::LANES {
        pack!(u64, 35, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_64_36(input: &[u64; 1024], output: &mut [u64; 1024 * 36 / u64::T]) {
    for lane in 0..u64::LANES {
        pack!(u64, 36, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_64_37(input: &[u64; 1024], output: &mut [u64; 1024 * 37 / u64::T]) {
    for lane in 0..u64::LANES {
        pack!(u64, 37, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_64_38(input: &[u64; 1024], output: &mut [u64; 1024 * 38 / u64::T]) {
    for lane in 0..u64::LANES {
        pack!(u64, 38, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_64_39(input: &[u64; 1024], output: &mut [u64; 1024 * 39 / u64::T]) {
    for lane in 0..u64::LANES {
        pack!(u64, 39, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_64_40(input: &[u64; 1024], output: &mut [u64; 1024 * 40 / u64::T]) {
    for lane in 0..u64::LANES {
        pack!(u64, 40, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_64_41(input: &[u64; 1024], output: &mut [u64; 1024 * 41 / u64::T]) {
    for lane in 0..u64::LANES {
        pack!(u64, 41, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_64_42(input: &[u64; 1024], output: &mut [u64; 1024 * 42 / u64::T]) {
    for lane in 0..u64::LANES {
        pack!(u64, 42, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_64_43(input: &[u64; 1024], output: &mut [u64; 1024 * 43 / u64::T]) {
    for lane in 0..u64::LANES {
        pack!(u64, 43, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_64_44(input: &[u64; 1024], output: &mut [u64; 1024 * 44 / u64::T]) {
    for lane in 0..u64::LANES {
        pack!(u64, 44, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_64_45(input: &[u64; 1024], output: &mut [u64; 1024 * 45 / u64::T]) {
    for lane in 0..u64::LANES {
        pack!(u64, 45, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_64_46(input: &[u64; 1024], output: &mut [u64; 1024 * 46 / u64::T]) {
    for lane in 0..u64::LANES {
        pack!(u64, 46, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_64_47(input: &[u64; 1024], output: &mut [u64; 1024 * 47 / u64::T]) {
    for lane in 0..u64::LANES {
        pack!(u64, 47, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_64_48(input: &[u64; 1024], output: &mut [u64; 1024 * 48 / u64::T]) {
    for lane in 0..u64::LANES {
        pack!(u64, 48, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_64_49(input: &[u64; 1024], output: &mut [u64; 1024 * 49 / u64::T]) {
    for lane in 0..u64::LANES {
        pack!(u64, 49, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_64_50(input: &[u64; 1024], output: &mut [u64; 1024 * 50 / u64::T]) {
    for lane in 0..u64::LANES {
        pack!(u64, 50, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_64_51(input: &[u64; 1024], output: &mut [u64; 1024 * 51 / u64::T]) {
    for lane in 0..u64::LANES {
        pack!(u64, 51, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_64_52(input: &[u64; 1024], output: &mut [u64; 1024 * 52 / u64::T]) {
    for lane in 0..u64::LANES {
        pack!(u64, 52, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_64_53(input: &[u64; 1024], output: &mut [u64; 1024 * 53 / u64::T]) {
    for lane in 0..u64::LANES {
        pack!(u64, 53, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_64_54(input: &[u64; 1024], output: &mut [u64; 1024 * 54 / u64::T]) {
    for lane in 0..u64::LANES {
        pack!(u64, 54, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_64_55(input: &[u64; 1024], output: &mut [u64; 1024 * 55 / u64::T]) {
    for lane in 0..u64::LANES {
        pack!(u64, 55, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_64_56(input: &[u64; 1024], output: &mut [u64; 1024 * 56 / u64::T]) {
    for lane in 0..u64::LANES {
        pack!(u64, 56, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_64_57(input: &[u64; 1024], output: &mut [u64; 1024 * 57 / u64::T]) {
    for lane in 0..u64::LANES {
        pack!(u64, 57, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_64_58(input: &[u64; 1024], output: &mut [u64; 1024 * 58 / u64::T]) {
    for lane in 0..u64::LANES {
        pack!(u64, 58, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_64_59(input: &[u64; 1024], output: &mut [u64; 1024 * 59 / u64::T]) {
    for lane in 0..u64::LANES {
        pack!(u64, 59, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_64_60(input: &[u64; 1024], output: &mut [u64; 1024 * 60 / u64::T]) {
    for lane in 0..u64::LANES {
        pack!(u64, 60, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_64_61(input: &[u64; 1024], output: &mut [u64; 1024 * 61 / u64::T]) {
    for lane in 0..u64::LANES {
        pack!(u64, 61, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_64_62(input: &[u64; 1024], output: &mut [u64; 1024 * 62 / u64::T]) {
    for lane in 0..u64::LANES {
        pack!(u64, 62, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_64_63(input: &[u64; 1024], output: &mut [u64; 1024 * 63 / u64::T]) {
    for lane in 0..u64::LANES {
        pack!(u64, 63, output, lane, |$idx| {
            input[$idx]
        });
    }
}

fn pack_64_64(input: &[u64; 1024], output: &mut [u64; 1024 * 64 / u64::T]) {
    for lane in 0..u64::LANES {
        pack!(u64, 64, output, lane, |$idx| {
            input[$idx]
        });
    }
}
*/
/* 
macro_rules! impl_packing {
    ($T:ty) => {
        paste! {
            impl BitPacking for $T {
                fn pack<const W: usize>(
                    input: &[Self; 1024],
                    output: &mut [Self; 1024 * W / Self::T],
                ) where BitPackWidth<W>: SupportedBitPackWidth<Self> {
                    for lane in 0..Self::LANES {
                        pack!($T, W, output, lane, |$idx| {
                            input[$idx]
                        });
                    }
                }

                unsafe fn unchecked_pack(width: usize, input: &[Self], output: &mut [Self]) {
                    let packed_len = 128 * width / size_of::<Self>();
                    debug_assert_eq!(output.len(), packed_len, "Output buffer must be of size 1024 * W / T");
                    debug_assert_eq!(input.len(), 1024, "Input buffer must be of size 1024");
                    debug_assert!(width <= Self::T, "Width must be less than or equal to {}", Self::T);

                    seq_t!(W in $T {
                        match width {
                            #(W => Self::pack::<W>(
                                array_ref![input, 0, 1024],
                                array_mut_ref![output, 0, 1024 * W / <$T>::T],
                            ),)*
                            // seq_t has exclusive upper bound
                            Self::T => Self::pack::<{ Self::T }>(
                                array_ref![input, 0, 1024],
                                array_mut_ref![output, 0, 1024],
                            ),
                            _ => unreachable!("Unsupported width: {}", width)
                        }
                    })
                }

                fn unpack<const W: usize>(
                    input: &[Self; 1024 * W / Self::T],
                    output: &mut [Self; 1024],
                ) where BitPackWidth<W>: SupportedBitPackWidth<Self> {
                    for lane in 0..Self::LANES {
                        unpack!($T, W, input, lane, |$idx, $elem| {
                            output[$idx] = $elem
                        });
                    }
                }

                unsafe fn unchecked_unpack(width: usize, input: &[Self], output: &mut [Self]) {
                    let packed_len = 128 * width / size_of::<Self>();
                    debug_assert_eq!(input.len(), packed_len, "Input buffer must be of size 1024 * W / T");
                    debug_assert_eq!(output.len(), 1024, "Output buffer must be of size 1024");
                    debug_assert!(width <= Self::T, "Width must be less than or equal to {}", Self::T);

                    seq_t!(W in $T {
                        match width {
                            #(W => Self::unpack::<W>(
                                array_ref![input, 0, 1024 * W / <$T>::T],
                                array_mut_ref![output, 0, 1024],
                            ),)*
                            // seq_t has exclusive upper bound
                            Self::T => Self::unpack::<{ Self::T }>(
                                array_ref![input, 0, 1024],
                                array_mut_ref![output, 0, 1024],
                            ),
                            _ => unreachable!("Unsupported width: {}", width)
                        }
                    })
                }

                /// Unpacks a single element at the provided index from a packed array of 1024 `W` bit elements.
                fn unpack_single<const W: usize>(packed: &[Self; 1024 * W / Self::T], index: usize) -> Self
                where
                    BitPackWidth<W>: SupportedBitPackWidth<Self>,
                {
                    if W == 0 {
                        // Special case for W=0, we just need to zero the output.
                        return 0 as $T;
                    }

                    // We can think of the input array as effectively a row-major, left-to-right
                    // 2-D array of with `Self::LANES` columns and `Self::T` rows.
                    //
                    // Meanwhile, we can think of the packed array as either:
                    //      1. `Self::T` rows of W-bit elements, with `Self::LANES` columns
                    //      2. `W` rows of `Self::T`-bit words, with `Self::LANES` columns
                    //
                    // Bitpacking involves a transposition of the input array ordering, such that
                    // decompression can be fused efficiently with encodings like delta and RLE.
                    //
                    // First step, we need to get the lane and row for interpretation #1 above.
                    assert!(index < 1024, "Index must be less than 1024, got {}", index);
                    let (lane, row): (usize, usize) = {
                        const LANES: [u8; 1024] = lanes_by_index::<$T>();
                        const ROWS: [u8; 1024] = rows_by_index::<$T>();
                        (LANES[index] as usize, ROWS[index] as usize)
                    };

                    if W == <$T>::T {
                        // Special case for W==T, we can just read the value directly
                        return packed[<$T>::LANES * row + lane];
                    }

                    let mask: $T = (1 << (W % <$T>::T)) - 1;
                    let start_bit = row * W;
                    let start_word = start_bit / <$T>::T;
                    let lo_shift = start_bit % <$T>::T;
                    let remaining_bits = <$T>::T - lo_shift;

                    let lo = packed[<$T>::LANES * start_word + lane] >> lo_shift;
                    return if remaining_bits >= W {
                        // in this case we will mask out all bits of hi word
                        lo & mask
                    } else {
                        // guaranteed that lo_shift > 0 and thus remaining_bits < T
                        let hi = packed[<$T>::LANES * (start_word + 1) + lane] << remaining_bits;
                        (lo | hi) & mask
                    };
                }

                unsafe fn unchecked_unpack_single(width: usize, packed: &[Self], index: usize) -> Self {
                    const T: usize = <$T>::T;

                    let packed_len = 128 * width / size_of::<Self>();
                    debug_assert_eq!(packed.len(), packed_len, "Input buffer must be of size {}", packed_len);
                    debug_assert!(width <= Self::T, "Width must be less than or equal to {}", Self::T);

                    seq_t!(W in $T {
                        match width {
                            #(W => {
                                return <$T>::unpack_single::<W>(array_ref![packed, 0, 1024 * W / T], index);
                            },)*
                            // seq_t has exclusive upper bound
                            T => {
                                return <$T>::unpack_single::<T>(array_ref![packed, 0, 1024], index);
                            },
                            _ => unreachable!("Unsupported width: {}", width)
                        }
                    })
                }
            }
        }
    };
}
*/

/* 
// helper function executed at compile-time to speed up unpack_single at runtime
const fn lanes_by_index<T: FastLanes>() -> [u8; 1024] {
    let mut lanes = [0u8; 1024];
    const_for!(i in 0..1024 => {
        lanes[i] = (i % T::LANES) as u8;
    });
    lanes
}

// helper function executed at compile-time to speed up unpack_single at runtime
const fn rows_by_index<T: FastLanes>() -> [u8; 1024] {
    let mut rows = [0u8; 1024];
    const_for!(i in 0..1024 => {
        // This is the inverse of the `index` function from the pack/unpack macros:
        //     fn index(row: usize, lane: usize) -> usize {
        //         let o = row / 8;
        //         let s = row % 8;
        //         (FL_ORDER[o] * 16) + (s * 128) + lane
        //     }
        let lane = i % T::LANES;
        let s = i / 128; // because `(FL_ORDER[o] * 16) + lane` is always < 128
        let fl_order = (i - s * 128 - lane) / 16; // value of FL_ORDER[o]
        let o = FL_ORDER[fl_order]; // because this transposition is invertible!
        rows[i] = (o * 8 + s) as u8;
    });
    rows
}

*/
#[cfg(test)]
mod test {
    use core::array;
    use super::*;

    #[test]
    fn test_unchecked_pack() {
        let input = array::from_fn(|i| i as u32);
        let mut packed = [0; 320];
        unsafe { BitPacking::unchecked_pack(10, &input, &mut packed) };
        let mut output = [0; 1024];
        unsafe { BitPacking::unchecked_unpack(10, &packed, &mut output) };
        assert_eq!(input, output);
    }
}
