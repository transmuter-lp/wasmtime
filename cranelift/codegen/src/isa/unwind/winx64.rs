//! Windows x64 ABI unwind information.

use alloc::vec::Vec;
use log::warn;
#[cfg(feature = "enable-serde")]
use serde_derive::{Deserialize, Serialize};

use crate::binemit::CodeOffset;
use crate::isa::unwind::UnwindInst;
use crate::result::{CodegenError, CodegenResult};

use super::Writer;

/// Maximum (inclusive) size of a "small" stack allocation
const SMALL_ALLOC_MAX_SIZE: u32 = 128;
/// Maximum (inclusive) size of a "large" stack allocation that can represented in 16-bits
const LARGE_ALLOC_16BIT_MAX_SIZE: u32 = 524280;

/// The supported unwind codes for the x64 Windows ABI.
///
/// See: <https://docs.microsoft.com/en-us/cpp/build/exception-handling-x64>
/// Only what is needed to describe the prologues generated by the Cranelift x86 ISA are represented here.
/// Note: the Cranelift x86 ISA RU enum matches the Windows unwind GPR encoding values.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub(crate) enum UnwindCode {
    PushRegister {
        instruction_offset: u8,
        reg: u8,
    },
    SaveReg {
        instruction_offset: u8,
        reg: u8,
        stack_offset: u32,
    },
    SaveXmm {
        instruction_offset: u8,
        reg: u8,
        stack_offset: u32,
    },
    StackAlloc {
        instruction_offset: u8,
        size: u32,
    },
    SetFPReg {
        instruction_offset: u8,
    },
}

impl UnwindCode {
    fn emit(&self, writer: &mut Writer) {
        enum UnwindOperation {
            PushNonvolatileRegister = 0,
            LargeStackAlloc = 1,
            SmallStackAlloc = 2,
            SetFPReg = 3,
            SaveNonVolatileRegister = 4,
            SaveNonVolatileRegisterFar = 5,
            SaveXmm128 = 8,
            SaveXmm128Far = 9,
        }

        match self {
            Self::PushRegister {
                instruction_offset,
                reg,
            } => {
                writer.write_u8(*instruction_offset);
                writer.write_u8((*reg << 4) | (UnwindOperation::PushNonvolatileRegister as u8));
            }
            Self::SaveReg {
                instruction_offset,
                reg,
                stack_offset,
            }
            | Self::SaveXmm {
                instruction_offset,
                reg,
                stack_offset,
            } => {
                let is_xmm = match self {
                    Self::SaveXmm { .. } => true,
                    _ => false,
                };
                let (op_small, op_large) = if is_xmm {
                    (UnwindOperation::SaveXmm128, UnwindOperation::SaveXmm128Far)
                } else {
                    (
                        UnwindOperation::SaveNonVolatileRegister,
                        UnwindOperation::SaveNonVolatileRegisterFar,
                    )
                };
                writer.write_u8(*instruction_offset);
                let scaled_stack_offset = stack_offset / 16;
                if scaled_stack_offset <= core::u16::MAX as u32 {
                    writer.write_u8((*reg << 4) | (op_small as u8));
                    writer.write_u16_le(scaled_stack_offset as u16);
                } else {
                    writer.write_u8((*reg << 4) | (op_large as u8));
                    writer.write_u16_le(*stack_offset as u16);
                    writer.write_u16_le((stack_offset >> 16) as u16);
                }
            }
            Self::StackAlloc {
                instruction_offset,
                size,
            } => {
                // Stack allocations on Windows must be a multiple of 8 and be at least 1 slot
                assert!(*size >= 8);
                assert!((*size % 8) == 0);

                writer.write_u8(*instruction_offset);
                if *size <= SMALL_ALLOC_MAX_SIZE {
                    writer.write_u8(
                        ((((*size - 8) / 8) as u8) << 4) | UnwindOperation::SmallStackAlloc as u8,
                    );
                } else if *size <= LARGE_ALLOC_16BIT_MAX_SIZE {
                    writer.write_u8(UnwindOperation::LargeStackAlloc as u8);
                    writer.write_u16_le((*size / 8) as u16);
                } else {
                    writer.write_u8((1 << 4) | (UnwindOperation::LargeStackAlloc as u8));
                    writer.write_u32_le(*size);
                }
            }
            Self::SetFPReg { instruction_offset } => {
                writer.write_u8(*instruction_offset);
                writer.write_u8(UnwindOperation::SetFPReg as u8);
            }
        }
    }

    fn node_count(&self) -> usize {
        match self {
            Self::StackAlloc { size, .. } => {
                if *size <= SMALL_ALLOC_MAX_SIZE {
                    1
                } else if *size <= LARGE_ALLOC_16BIT_MAX_SIZE {
                    2
                } else {
                    3
                }
            }
            Self::SaveXmm { stack_offset, .. } | Self::SaveReg { stack_offset, .. } => {
                if *stack_offset <= core::u16::MAX as u32 {
                    2
                } else {
                    3
                }
            }
            _ => 1,
        }
    }
}

pub(crate) enum MappedRegister {
    Int(u8),
    Xmm(u8),
}

/// Maps UnwindInfo register to Windows x64 unwind data.
pub(crate) trait RegisterMapper<Reg> {
    /// Maps a Reg to a Windows unwind register number.
    fn map(reg: Reg) -> MappedRegister;
}

/// Represents Windows x64 unwind information.
///
/// For information about Windows x64 unwind info, see:
/// <https://docs.microsoft.com/en-us/cpp/build/exception-handling-x64>
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub struct UnwindInfo {
    pub(crate) flags: u8,
    pub(crate) prologue_size: u8,
    pub(crate) frame_register: Option<u8>,
    pub(crate) frame_register_offset: u8,
    pub(crate) unwind_codes: Vec<UnwindCode>,
}

impl UnwindInfo {
    /// Gets the emit size of the unwind information, in bytes.
    pub fn emit_size(&self) -> usize {
        let node_count = self.node_count();

        // Calculation of the size requires no SEH handler or chained info
        assert!(self.flags == 0);

        // Size of fixed part of UNWIND_INFO is 4 bytes
        // Then comes the UNWIND_CODE nodes (2 bytes each)
        // Then comes 2 bytes of padding for the unwind codes if necessary
        // Next would come the SEH data, but we assert above that the function doesn't have SEH data

        4 + (node_count * 2) + if (node_count & 1) == 1 { 2 } else { 0 }
    }

    /// Emits the unwind information into the given mutable byte slice.
    ///
    /// This function will panic if the slice is not at least `emit_size` in length.
    pub fn emit(&self, buf: &mut [u8]) {
        const UNWIND_INFO_VERSION: u8 = 1;

        let node_count = self.node_count();
        assert!(node_count <= 256);

        let mut writer = Writer::new(buf);

        writer.write_u8((self.flags << 3) | UNWIND_INFO_VERSION);
        writer.write_u8(self.prologue_size);
        writer.write_u8(node_count as u8);

        if let Some(reg) = self.frame_register {
            writer.write_u8((self.frame_register_offset << 4) | reg);
        } else {
            writer.write_u8(0);
        }

        // Unwind codes are written in reverse order (prologue offset descending)
        for code in self.unwind_codes.iter().rev() {
            code.emit(&mut writer);
        }

        // To keep a 32-bit alignment, emit 2 bytes of padding if there's an odd number of 16-bit nodes
        if (node_count & 1) == 1 {
            writer.write_u16_le(0);
        }

        // Ensure the correct number of bytes was emitted
        assert_eq!(writer.offset, self.emit_size());
    }

    fn node_count(&self) -> usize {
        self.unwind_codes
            .iter()
            .fold(0, |nodes, c| nodes + c.node_count())
    }
}

const UNWIND_RBP_REG: u8 = 5;

pub(crate) fn create_unwind_info_from_insts<MR: RegisterMapper<crate::machinst::Reg>>(
    insts: &[(CodeOffset, UnwindInst)],
) -> CodegenResult<UnwindInfo> {
    let mut unwind_codes = vec![];
    let mut frame_register_offset = 0;
    let mut max_unwind_offset = 0;
    for &(instruction_offset, ref inst) in insts {
        let instruction_offset = ensure_unwind_offset(instruction_offset)?;
        match inst {
            &UnwindInst::PushFrameRegs { .. } => {
                unwind_codes.push(UnwindCode::PushRegister {
                    instruction_offset,
                    reg: UNWIND_RBP_REG,
                });
            }
            &UnwindInst::DefineNewFrame {
                offset_downward_to_clobbers,
                ..
            } => {
                frame_register_offset = ensure_unwind_offset(offset_downward_to_clobbers)?;
                unwind_codes.push(UnwindCode::SetFPReg { instruction_offset });
            }
            &UnwindInst::StackAlloc { size } => {
                unwind_codes.push(UnwindCode::StackAlloc {
                    instruction_offset,
                    size,
                });
            }
            &UnwindInst::SaveReg {
                clobber_offset,
                reg,
            } => match MR::map(reg.into()) {
                MappedRegister::Int(reg) => {
                    unwind_codes.push(UnwindCode::SaveReg {
                        instruction_offset,
                        reg,
                        stack_offset: clobber_offset,
                    });
                }
                MappedRegister::Xmm(reg) => {
                    unwind_codes.push(UnwindCode::SaveXmm {
                        instruction_offset,
                        reg,
                        stack_offset: clobber_offset,
                    });
                }
            },
            &UnwindInst::RegStackOffset { .. } => {
                unreachable!("only supported with DWARF");
            }
            &UnwindInst::Aarch64SetPointerAuth { .. } => {
                unreachable!("no aarch64 on x64");
            }
        }
        max_unwind_offset = instruction_offset;
    }

    Ok(UnwindInfo {
        flags: 0,
        prologue_size: max_unwind_offset,
        frame_register: Some(UNWIND_RBP_REG),
        frame_register_offset,
        unwind_codes,
    })
}

fn ensure_unwind_offset(offset: u32) -> CodegenResult<u8> {
    if offset > 255 {
        warn!("function prologues cannot exceed 255 bytes in size for Windows x64");
        return Err(CodegenError::CodeTooLarge);
    }
    Ok(offset as u8)
}
