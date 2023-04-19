use crate::{CPU_MEMORY_CHANNELS, INSTRUCTION_ELEMENTS};
use core::borrow::{Borrow, BorrowMut};
use core::mem::{size_of, transmute};
use valida_memory::MEMORY_CELL_BYTES;
use valida_util::indices_arr;

pub struct CpuCols<T> {
    /// The program counter.
    pub pc: T,

    /// The instruction that was read, i.e. `program_code[pc]`.
    pub instruction: [T; INSTRUCTION_ELEMENTS],

    /// Buffers for the two memory reads and single write
    pub memory_read_1: [T; MEMORY_CELL_BYTES],
    pub memory_read_2: [T; MEMORY_CELL_BYTES],
    pub memory_write: [T; MEMORY_CELL_BYTES],

    /// Flags indicating what type of operation is being performed this cycle.
    pub opcode_flags: OpcodeFlagCols<T>,

    /// Channels to the memory bus.
    pub mem_channels: [MemoryChannelCols<T>; CPU_MEMORY_CHANNELS],

    /// Channel to the shared coprocessor bus
    pub coprocessor_channel: CoprocessorChannelCols<T>,
}

pub struct OpcodeFlagCols<T> {
    pub is_imm32: T,
    pub is_bus_op: T,
}

pub struct MemoryChannelCols<T> {
    pub used: T,
    pub addr: T,
    pub value: [T; MEMORY_CELL_BYTES],
}

pub struct CoprocessorChannelCols<T> {
    pub opcode: T,
    pub value: T,
}

// `u8` is guaranteed to have a `size_of` of 1.
pub const NUM_CPU_COLUMNS: usize = size_of::<CpuCols<u8>>();

pub const CPU_COL_INDICES: CpuCols<usize> = make_col_map();

const fn make_col_map() -> CpuCols<usize> {
    let indices_arr = indices_arr::<NUM_CPU_COLUMNS>();
    unsafe { transmute::<[usize; NUM_CPU_COLUMNS], CpuCols<usize>>(indices_arr) }
}

impl<T> Borrow<CpuCols<T>> for [T] {
    fn borrow(&self) -> &CpuCols<T> {
        // TODO: Double check if this is correct & consider making asserts debug-only.
        let (prefix, shorts, _suffix) = unsafe { self.align_to::<CpuCols<T>>() };
        assert!(prefix.is_empty(), "Data was not aligned");
        assert_eq!(shorts.len(), 1);
        &shorts[0]
    }
}

impl<T> BorrowMut<CpuCols<T>> for [T] {
    fn borrow_mut(&mut self) -> &mut CpuCols<T> {
        // TODO: Double check if this is correct & consider making asserts debug-only.
        let (prefix, shorts, _suffix) = unsafe { self.align_to_mut::<CpuCols<T>>() };
        assert!(prefix.is_empty(), "Data was not aligned");
        assert_eq!(shorts.len(), 1);
        &mut shorts[0]
    }
}