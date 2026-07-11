/// Manual references in this crate, both figure and page numbers, are with respect to the document
/// `DDI0406C_d_armv7ar_arm.pdf`
/// `sha256: 294668ae6480133b32d85e9567cc77c5eb0e1232decdf42cac7ab480e884f6e0`

//#[cfg(feature="use-serde")]
//use serde::{Serialize, Deserialize};

use core::fmt::{self, Display, Formatter};

use yaxpeax_arch::{Arch, AddressDiff, Decoder, LengthedInstruction, Reader, ReadError};
#[allow(deprecated)]
use yaxpeax_arch::{NoColors, ShowContextual};

mod thumb;
// #[cfg(feature = "fmt")]
mod display;

#[cfg(all(feature="alloc", feature="fmt"))]
pub use display::InstructionTextBuffer;

// opcode, s, w, cond
/// a struct for the combined display of an opcode and possible suffixes.
///
/// this includes the opcode, its optional `.s` suffix, optional `.w` suffix, and condition code,
/// if any.
pub struct ConditionedOpcode(pub Opcode, pub bool, pub bool, pub ConditionCode);

impl Display for ConditionedOpcode {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}{}{}{}", self.0, if self.1 { "s" } else { "" }, if self.2 { ".w" } else { "" }, self.3)
    }
}

/// a context impl to display `arm` instructions with no additional context (no symbol name
/// information, offset names, etc). this impl results in `some_instruction.contextualize(...)`
/// displaying an instruction the same way its `Display` impl would.
pub struct NoContext;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[allow(missing_docs)]
#[cfg_attr(feature = "non-exhaustive-enums", non_exhaustive)]
pub enum Opcode {
    Invalid,
    /*
     * These two don't really have direct encodings, but are for the specific instances
     * where the semantics of the original instruction are the same as push (specifically
     * ldm/stm/mov that write to the stack and increment/decrement appropriately
     */
    POP,
    PUSH,

    B,
    BL,
    BLX,
    BX,
    BXJ,
    AND,
    EOR,
    SUB,
    RSB,
    ADD,
    ADC,
    SBC,
    RSC,
    TST,
    TEQ,
    CMP,
    CMN,
    ORR,
    MOV,
    BIC,
    MVN,
    LSL,
    LSR,
    ASR,
    RRX,
    ROR,
    ADR,
    MSR,
    MRS,
    CLZ,
    LDREXH,
    STREXH,
    LDREXB,
    STREXB,
    LDREXD,
    STREXD,
    LDREX,
    STREX,
    LDM(bool, bool, bool, bool),
    STM(bool, bool, bool, bool),
    LDR,
    STR,
    LDRH,
    STRH,
    LDRB,
    STRB,
    LDRSH,
    LDRSHT,
    LDRSB,
    LDRSBT,
    STRD,
    LDRD,
    LDC(u8, bool),
    LDCL(u8, bool),
    STC(u8, bool),
    STCL(u8, bool),
    MCRR(u8, u8, bool),
    MRRC(u8, u8, bool),
    /// > MCR (Move to Coprocessor from ARM Register)
    ///
    /// fields here are `coproc`, `opcode_1`, `opcode_2`, and a bool indicating if the original
    /// encoding was `MCR` or `MCR2` (`true` means `MCR2`).
    MCR(u8, u8, u8, bool),
    /// > MRC (Move to ARM Register from Coprocessor)
    ///
    /// fields here are `coproc`, `opcode_1`, `opcode_2`, and a bool indicating if the original
    /// encoding was `MRC` or `MRC2` (`true` means `MRC2`).
    MRC(u8, u8, u8, bool),
    CDP(u8, u8, u8, bool),
    SRS(bool, bool),
    RFE(bool, bool),
    LDRT,
    STRT,
    LDRHT,
    STRHT,
    LDRBT,
    STRBT,
    SWP,
    SWPB,
    MUL,
    MLA,
    UMAAL,
    MLS,
    UMULL,
    UMLAL,
    SMULL,
    SMUL(bool, bool),
    SMLA(bool, bool),
    SMLAL,
    SMLAL_halfword(bool, bool),
    SMAL(bool, bool),
    SMLAW(bool),
    ERET,
    BKPT,
    HVC,
    SMC,
    MOVT,
    QDSUB,
    QDADD,
    QSUB,
    QADD,

    TBB,
    TBH,
    UDF,
    SVC,
    WFE,
    WFI,
    SEV,
    CSDB,
    YIELD,
    HINT,
    NOP,
    LEAVEX,
    ENTERX,
    CLREX,
    DSB,
    DMB,
    ISB,
    SXTH,
    UXTH,
    SXTB16,
    UXTB16,
    SXTB,
    UXTB,
    SXTAH,
    UXTAH,
    SXTAB16,
    UXTAB16,
    SXTAB,
    UXTAB,
    CBZ,
    CBNZ,
    SETEND,
    CPS(bool),
    CPS_modeonly,
    REV,
    REV16,
    REVSH,
    IT,

    PKHTB,
    PKHBT,
    ORN,
    SSAT,
    SSAT16,
    SBFX,
    USAT,
    USAT16,
    UBFX,
    BFI,
    BFC,
    DBG,
    PLD,
    PLI,
    RBIT,
    SEL,

    SADD16,
    QADD16,
    SHADD16,
    SASX,
    QASX,
    SHASX,
    SSAX,
    QSAX,
    SHSAX,
    SSUB16,
    QSUB16,
    SHSUB16,
    SADD8,
    QADD8,
    SHADD8,
    SSUB8,
    QSUB8,
    SHSUB8,
    UADD16,
    UQADD16,
    UHADD16,
    UASX,
    UQASX,
    UHASX,
    USAX,
    UQSAX,
    UHSAX,
    USUB16,
    UQSUB16,
    UHSUB16,
    UADD8,
    UQADD8,
    UHADD8,
    USUB8,
    UQSUB8,
    UHSUB8,

    SMLSD,
    SMMLA,
    SMMLS,
    USADA8,
    USAD8,
    SMLAD,
    SMUSD,
    SMMUL,
    SMULW(bool),
    SMUAD,
    SDIV,
    UDIV,
    SMLALD(bool),
    SMLSLD(bool),

    // VFP (floating-point) instructions. the `.f32`/`.f64` data-type suffix is derived at display
    // time from the width of the register operands, except where noted.
    VMLA,
    VMLS,
    VNMLA,
    VNMLS,
    VMUL,
    VNMUL,
    VADD,
    VSUB,
    VDIV,
    VFMA,
    VFMS,
    VFNMA,
    VFNMS,
    VABS,
    VNEG,
    VSQRT,
    /// `vmov` in all its forms (register, immediate, core<->scalar transfers); the concrete form is
    /// distinguished by the operands.
    VMOV,
    /// `vcmp` (`false`) or `vcmpe` (`true`, signalling on NaN).
    VCMP(bool),
    VMRS,
    VMSR,
    VLDR,
    VSTR,
    VPUSH,
    VPOP,
    /// `vldm` with the increment-after (`true`) or decrement-before (`false`) addressing mode.
    VLDM(bool),
    /// `vstm` with the increment-after (`true`) or decrement-before (`false`) addressing mode.
    VSTM(bool),
    /// deprecated `fldmiax`/`fldmdbx` (the "X", extra-word, load-multiple form).
    VLDMX(bool),
    /// deprecated `fstmiax`/`fstmdbx` (the "X", extra-word, store-multiple form).
    VSTMX(bool),
    /// `vdup` broadcasting an ARM core register across a SIMD register; the datatype gives the
    /// element size.
    VDUP(SIMDDataType),
    /// `vmov` transferring a single lane between an ARM core register and a SIMD register; the
    /// datatype gives the element size and (for lane-to-core) signedness.
    VMOVLane(SIMDDataType),
    /// `vcvt`/`vcvtr` between floating-point and integer, and between f32 and f64. the destination
    /// and source data types are carried explicitly; `round` selects `vcvtr` (round per FPSCR)
    /// over `vcvt` (round toward zero) for the float-to-integer forms.
    VCVT { to: VfpType, from: VfpType, round: bool },

    /// NEON structure load `vld1`-`vld4`; the `u8` is the structure count (1-4).
    VLDN(u8, SIMDDataType),
    /// NEON structure store `vst1`-`vst4`; the `u8` is the structure count (1-4).
    VSTN(u8, SIMDDataType),
}

/// a VFP scalar data type, used to render the `.<ty>` suffixes of `vcvt`.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum VfpType {
    F16,
    F32,
    F64,
    S16,
    U16,
    S32,
    U32,
}

/// an Advanced-SIMD/NEON data-type suffix (`.8`, `.s16`, `.u32`, `.i16`, `.f32`, `.p8`, ...). the
/// `u8` in each variant is the element size in bits.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum SIMDDataType {
    /// a size with no signedness, e.g. `.8`, `.32`.
    Any(u8),
    /// a signed integer, e.g. `.s16`.
    Signed(u8),
    /// an unsigned integer, e.g. `.u16`.
    Unsigned(u8),
    /// an unspecified integer, e.g. `.i16`.
    Int(u8),
    /// a floating-point type, e.g. `.f32`.
    Float(u8),
    /// a polynomial type, e.g. `.p8`.
    Poly(u8),
}

static DATA_PROCESSING_OPCODES: [Opcode; 16] = [
    Opcode::AND,
    Opcode::EOR,
    Opcode::SUB,
    Opcode::RSB,
    Opcode::ADD,
    Opcode::ADC,
    Opcode::SBC,
    Opcode::RSC,
    Opcode::TST,
    Opcode::TEQ,
    Opcode::CMP,
    Opcode::CMN,
    Opcode::ORR,
    Opcode::MOV,
    Opcode::BIC,
    Opcode::MVN
];

/// a struct describiing a shifted register operand. this is primarily interesting in that it can
/// be translated to a `RegShiftStyle` for further interpretation.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(transparent)]
pub struct RegShift {
    data: u16
}

impl RegShift {
    /// convert an instruction's `RegShift` operand into something more appropriate for
    /// programmatic use.
    pub fn into_shift(&self) -> RegShiftStyle {
        if self.data & 0b10000 == 0 {
            RegShiftStyle::RegImm(RegImmShift { data: self.data })
        } else {
            RegShiftStyle::RegReg(RegRegShift { data: self.data })
        }
    }

    /// Does this shift actually do anything?
    fn is_noop(&self) -> bool {
        match self.into_shift() {
            RegShiftStyle::RegImm(shift) => shift.is_noop(),
            // We can't know statically that this is a no-op, because the final result is based on
            // register data. Therefore, consider it a live shift.
            RegShiftStyle::RegReg(_) => false,
        }
    }

    /// don't use this. it's for armv7 testing only.
    #[doc(hidden)]
    pub fn from_raw(data: u16) -> Self {
        RegShift { data }
    }
}

/// an enum describing one of two ways a shifted register operand may be shifted.
pub enum RegShiftStyle {
    /// a register shifted by an immediate.
    RegImm(RegImmShift),
    /// a register shifted by a register.
    RegReg(RegRegShift),
}

/// a register shifted by a register.
#[repr(transparent)]
pub struct RegRegShift {
    data: u16
}

/// the way a shift operation is carried out.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ShiftStyle {
    /// left-shift the value, filling in zeroes.
    LSL = 0,
    /// right-shift the value, filling in zeroes.
    LSR = 1,
    /// arithmetic shift right, filling with the top bit of the value (sign-extending).
    ASR = 2,
    /// rotate-right, filling with bits shifted out of the value.
    ROR = 3,
}

impl Display for ShiftStyle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use core::fmt::Write;
        let name = self.name();
        f.write_char(name[0] as char)?;
        f.write_char(name[1] as char)?;
        f.write_char(name[2] as char)?;
        Ok(())
    }
}

impl ShiftStyle {
    fn from(bits: u8) -> ShiftStyle {
        match bits {
            0b00 => ShiftStyle::LSL,
            0b01 => ShiftStyle::LSR,
            0b10 => ShiftStyle::ASR,
            0b11 => ShiftStyle::ROR,
            _ => unreachable!("bad ShiftStyle index")
        }
    }

    fn name(&self) -> &'static [u8; 3] {
        match self {
            ShiftStyle::LSL => &[b'l', b's', b'l'],
            ShiftStyle::LSR => &[b'l', b's', b'r'],
            ShiftStyle::ASR => &[b'a', b's', b'r'],
            ShiftStyle::ROR => &[b'r', b'o', b'r'],
        }
    }
}

impl RegRegShift {
    /// the general-purpose register, an amount to shift the shiftee.
    pub fn shifter(&self) -> Reg {
        Reg::from_u8((self.data >> 8) as u8 & 0b1111)
    }
    /// the way in which this register is shifted.
    pub fn stype(&self) -> ShiftStyle {
        ShiftStyle::from((self.data >> 5) as u8 & 0b11)
    }
    /// the general-purpose register to be shifted.
    pub fn shiftee(&self) -> Reg {
        Reg::from_u8(self.data as u8 & 0b1111)
    }
}

/// a register shifted by an immediate.
#[repr(transparent)]
pub struct RegImmShift {
    data: u16
}

impl RegImmShift {
    /// the immediate this register is shifted by.
    pub fn imm(&self) -> u8 {
        (self.data >> 7) as u8 & 0b11111
    }
    /// the way in which this register is shifted.
    pub fn stype(&self) -> ShiftStyle {
        ShiftStyle::from((self.data >> 5) as u8 & 0b11)
    }
    /// the general-purpose register to be shifted.
    pub fn shiftee(&self) -> Reg {
        Reg::from_u8(self.data as u8 & 0b1111)
    }

    /// Does this shift actually do anything?
    fn is_noop(&self) -> bool {
        self.stype() == ShiftStyle::LSL && self.imm() == 0
    }
}

/// a struct describing an `arm` register.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct Reg {
    bits: u8
}

impl Reg {
    #[allow(non_snake_case)]
    fn from_sysm(R: bool, M: u8) -> Option<Operand> {
/*
 * Is one of:
 * • <Rm>_<mode>, encoded with R==0.
 * • ELR_hyp, encoded with R==0.
 * • SPSR_<mode>, encoded with R==1.
 * For a full description of the encoding of this field, see Encoding and use of Banked register
 * transfer
 * instructions on page B9-1959.
 * */
        if R == false {
            [
                Some(Operand::BankedReg(Bank::Usr, Reg::from_u8(8))),
                Some(Operand::BankedReg(Bank::Usr, Reg::from_u8(9))),
                Some(Operand::BankedReg(Bank::Usr, Reg::from_u8(10))),
                Some(Operand::BankedReg(Bank::Usr, Reg::from_u8(11))),
                Some(Operand::BankedReg(Bank::Usr, Reg::from_u8(12))),
                Some(Operand::BankedReg(Bank::Usr, Reg::from_u8(13))),
                Some(Operand::BankedReg(Bank::Usr, Reg::from_u8(14))),
                None,
                Some(Operand::BankedReg(Bank::Fiq, Reg::from_u8(8))),
                Some(Operand::BankedReg(Bank::Fiq, Reg::from_u8(9))),
                Some(Operand::BankedReg(Bank::Fiq, Reg::from_u8(10))),
                Some(Operand::BankedReg(Bank::Fiq, Reg::from_u8(11))),
                Some(Operand::BankedReg(Bank::Fiq, Reg::from_u8(12))),
                Some(Operand::BankedReg(Bank::Fiq, Reg::from_u8(13))),
                Some(Operand::BankedReg(Bank::Fiq, Reg::from_u8(14))),
                None,
                // for the alternate modes (irq/svc/abt/und/mon/hyp) the even SYSm selects R14
                // (LR) and the odd selects R13 (SP) - the reverse of the usr/fiq block above.
                // (ARM ARM Table B9-2.) SYSm 0b11110 is ELR_hyp, rendered `lr_hyp` here and
                // reconciled in the differential harness.
                Some(Operand::BankedReg(Bank::Irq, Reg::from_u8(14))),
                Some(Operand::BankedReg(Bank::Irq, Reg::from_u8(13))),
                Some(Operand::BankedReg(Bank::Svc, Reg::from_u8(14))),
                Some(Operand::BankedReg(Bank::Svc, Reg::from_u8(13))),
                Some(Operand::BankedReg(Bank::Abt, Reg::from_u8(14))),
                Some(Operand::BankedReg(Bank::Abt, Reg::from_u8(13))),
                Some(Operand::BankedReg(Bank::Und, Reg::from_u8(14))),
                Some(Operand::BankedReg(Bank::Und, Reg::from_u8(13))),
                None,
                None,
                None,
                None,
                Some(Operand::BankedReg(Bank::Mon, Reg::from_u8(14))),
                Some(Operand::BankedReg(Bank::Mon, Reg::from_u8(13))),
                Some(Operand::BankedReg(Bank::Hyp, Reg::from_u8(14))),
                Some(Operand::BankedReg(Bank::Hyp, Reg::from_u8(13))),
            ][M as usize]
        } else {
            if M == 0b01110 {
                Some(Operand::BankedSPSR(Bank::Fiq))
            } else if M == 0b10000 {
                Some(Operand::BankedSPSR(Bank::Irq))
            } else if M == 0b10010 {
                Some(Operand::BankedSPSR(Bank::Svc))
            } else if M == 0b10100 {
                Some(Operand::BankedSPSR(Bank::Abt))
            } else if M == 0b10110 {
                Some(Operand::BankedSPSR(Bank::Und))
            } else if M == 0b11100 {
                Some(Operand::BankedSPSR(Bank::Mon))
            } else if M == 0b11110 {
                Some(Operand::BankedSPSR(Bank::Hyp))
            } else {
                None
            }
        }
    }

    /// create a new `Reg` with the specified number.
    ///
    /// panics if `bits` is out of range (16 or above).
    #[inline]
    pub fn from_u8(bits: u8) -> Reg {
        if bits > 0b1111 {
            panic!("register number out of range");
        }

        Reg { bits }
    }

    /// get the number of this register. the returned value will be between 0 and 15.
    pub fn number(&self) -> u8 {
        self.bits
    }
}

/// a control register.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct CReg {
    bits: u8
}

impl Display for CReg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "c{}", self.bits)
    }
}

impl CReg {
    /// create a new `CReg` with the specified number.
    ///
    /// panics if `bits` is out of range (16 or above).
    #[inline]
    pub fn from_u8(bits: u8) -> CReg {
        if bits > 0b1111 {
            panic!("register number out of range");
        }

        CReg { bits }
    }

    /// get the number of this register. the returned value will be between 0 and 15.
    pub fn number(&self) -> u8 {
        self.bits
    }
}

/// the width of a VFP/Advanced-SIMD (NEON) register operand. `S` is a 32-bit single-precision
/// register (`s0`-`s31`), `D` a 64-bit doubleword register (`d0`-`d31`), and `Q` a 128-bit
/// quadword register (`q0`-`q15`). the three views alias the same underlying register file.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum SIMDSizeCode {
    S,
    D,
    Q,
}

/// a VFP/Advanced-SIMD register: a width (`S`/`D`/`Q`) and a register number. `S` numbers range
/// 0-31, `D` 0-31, and `Q` 0-15.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct SIMDReg {
    /// the width of this register (single/double/quad).
    pub size: SIMDSizeCode,
    /// the register number.
    pub num: u8,
}

impl SIMDReg {
    #[inline]
    fn new(size: SIMDSizeCode, num: u8) -> SIMDReg {
        SIMDReg { size, num }
    }
}

/// decode the NEON structure load/store space (`1111 0100 ...`). currently handles the `vld1`/
/// `vst1` (single structure to all/one, and multiple) contiguous "multiple" forms; the strided
/// `vld2`/`vld3`/`vld4` and single-lane/all-lanes forms are deferred (`InvalidOpcode`).
#[allow(non_snake_case)]
pub(crate) fn decode_neon_loadstore(word: u32) -> Result<(Opcode, [Operand; 4]), DecodeError> {
    let D = ((word >> 22) & 1) as u8;
    let L = (word >> 21) & 1 != 0; // 1 -> load
    let Rn = ((word >> 16) & 0b1111) as u8;
    let Vd = ((word >> 12) & 0b1111) as u8;
    let Rm = (word & 0b1111) as u8;

    let single = (word >> 23) & 1 != 0;
    if single {
        // single-element (to-one-lane / to-all-lanes) forms are not yet decoded.
        return Err(DecodeError::InvalidOpcode);
    }
    // the "multiple" forms require bit20 == 0; other values here are memory hints (pld/pli/...).
    if (word >> 20) & 1 != 0 {
        return Err(DecodeError::Incomplete);
    }

    // "multiple" forms: type field (bits 11:8) selects the structure kind and register count.
    let ty = (word >> 8) & 0b1111;
    let size = match (word >> 6) & 0b11 {
        0 => 8u8,
        1 => 16,
        2 => 32,
        _ => 64,
    };
    // only the contiguous vld1/vst1 forms (1-4 registers) are handled here.
    let count = match ty {
        0b0111 => 1u8,
        0b1010 => 2,
        0b0110 => 3,
        0b0010 => 4,
        _ => return Err(DecodeError::InvalidOpcode),
    };

    let first_num = (D << 4) | Vd;
    // a list running past d31 is unpredictable; reject it (capstone renders a bogus register name).
    if first_num as u16 + count as u16 > 32 {
        return Err(DecodeError::InvalidOperand);
    }
    let first = SIMDReg::new(SIMDSizeCode::D, first_num);
    let list = Operand::SIMDRegList(first, count);

    // alignment: aa (bits 5:4). 00 -> unaligned, else 64 << (aa-1).
    let align = match (word >> 4) & 0b11 {
        0 => 0u16,
        1 => 64,
        2 => 128,
        _ => 256,
    };

    // addressing mode from Rm: 15 -> plain, 13 -> writeback (`!`), else post-indexed by Rm.
    let deref = if Rm == 0b1111 {
        Operand::SIMDDeref { base: Reg::from_u8(Rn), align, wback: false, post: None }
    } else if Rm == 0b1101 {
        Operand::SIMDDeref { base: Reg::from_u8(Rn), align, wback: true, post: None }
    } else {
        Operand::SIMDDeref { base: Reg::from_u8(Rn), align, wback: false, post: Some(Reg::from_u8(Rm)) }
    };

    let dt = SIMDDataType::Any(size);
    let opcode = if L { Opcode::VLDN(1, dt) } else { Opcode::VSTN(1, dt) };
    let _ = count;
    Ok((opcode, [list, deref, Operand::Nothing, Operand::Nothing]))
}

/// build a VFP register from the `Vx`/`x` field split used throughout the VFP encodings. for a
/// single-precision (`sz == false`) register the number is `(Vx << 1) | x`; for double precision
/// (`sz == true`) it is `(x << 4) | Vx`.
#[inline]
fn vfp_reg(sz: bool, vx: u8, x: u8) -> SIMDReg {
    if sz {
        SIMDReg::new(SIMDSizeCode::D, (x << 4) | vx)
    } else {
        SIMDReg::new(SIMDSizeCode::S, (vx << 1) | x)
    }
}

/// decode the VFP (floating-point) coprocessor space (coproc `101x`). returns the opcode and
/// operands, or a `DecodeError` if the encoding is not a (Cortex-A9 / VFPv3) VFP instruction.
///
/// this covers the `cccc 1110 ...` data-processing and register-transfer encodings and the
/// `cccc 110x ...` extension-register load/store encodings. `word` is the full 32-bit instruction.
#[allow(non_snake_case)]
fn decode_vfp(word: u32) -> Result<(Opcode, [Operand; 4]), DecodeError> {
    let sz = (word >> 8) & 1 != 0; // 0 -> f32 (single), 1 -> f64 (double)
    let D = ((word >> 22) & 1) as u8;
    let N = ((word >> 7) & 1) as u8;
    let M = ((word >> 5) & 1) as u8;
    let Vd = ((word >> 12) & 0b1111) as u8;
    let Vn = ((word >> 16) & 0b1111) as u8;
    let Vm = (word & 0b1111) as u8;

    let op23 = (word >> 23) & 1;

    if (word >> 24) & 0b1111 == 0b1110 {
        // `cccc 1110 ...` data-processing / register transfer (bits 27:24 == 0b1110).
        let opc1 = ((word >> 20) & 0b1111) as u8; // includes the D bit at position 2
        let op6 = ((word >> 6) & 1) as u8;

        // the register-transfer forms have bit4 == 1; data-processing has bit4 == 0.
        if (word >> 4) & 1 == 1 {
            return decode_vfp_transfer(word);
        }

        // three-register (and the `1x11` "other") data-processing.
        // opc1 bits (23,21,20) select the group; op6 selects within a pair.
        let sel = (((word >> 23) & 1) << 2) | (((word >> 21) & 1) << 1) | ((word >> 20) & 1);
        let dd = vfp_reg(sz, Vd, D);
        let dn = vfp_reg(sz, Vn, N);
        let dm = vfp_reg(sz, Vm, M);
        let three = |op: Opcode| {
            Ok((op, [
                Operand::SIMDReg(dd),
                Operand::SIMDReg(dn),
                Operand::SIMDReg(dm),
                Operand::Nothing,
            ]))
        };
        let _ = opc1;
        match (sel, op6) {
            (0b000, 0) => return three(Opcode::VMLA),
            (0b000, 1) => return three(Opcode::VMLS),
            (0b001, 0) => return three(Opcode::VNMLS),
            (0b001, 1) => return three(Opcode::VNMLA),
            (0b010, 0) => return three(Opcode::VMUL),
            (0b010, 1) => return three(Opcode::VNMUL),
            (0b011, 0) => return three(Opcode::VADD),
            (0b011, 1) => return three(Opcode::VSUB),
            (0b100, 0) => return three(Opcode::VDIV),
            (0b101, 0) => return three(Opcode::VFNMS),
            (0b101, 1) => return three(Opcode::VFNMA),
            (0b110, 0) => return three(Opcode::VFMA),
            (0b110, 1) => return three(Opcode::VFMS),
            (0b111, _) => {
                // "other" data-processing: opc2 = Vn field selects the operation.
                return decode_vfp_other(word);
            }
            _ => return Err(DecodeError::InvalidOpcode),
        }
    }

    if ((word >> 25) & 0b111) == 0b110 {
        // `cccc 110x ...` extension-register load/store (coproc 101x).
        return decode_vfp_loadstore(word);
    }

    let _ = (op23, N);
    Err(DecodeError::InvalidOpcode)
}

/// the VFP extension-register load/store group (`cccc 110x ...`): VLDR, VSTR, VLDM, VSTM, VPUSH,
/// VPOP. also routes the 64-bit core<->register transfers to the transfer decoder.
#[allow(non_snake_case)]
fn decode_vfp_loadstore(word: u32) -> Result<(Opcode, [Operand; 4]), DecodeError> {
    let sz = (word >> 8) & 1 != 0; // 0 -> single (a2), 1 -> double (a1)
    let P = (word >> 24) & 1 != 0;
    let U = (word >> 23) & 1 != 0;
    let D = ((word >> 22) & 1) as u8;
    let W = (word >> 21) & 1 != 0;
    let L = (word >> 20) & 1 != 0;
    let Rn = ((word >> 16) & 0b1111) as u8;
    let Vd = ((word >> 12) & 0b1111) as u8;
    let imm8 = (word & 0xff) as u16;

    if !P && !U {
        // 64-bit core<->register transfers, or undefined; handled elsewhere.
        return decode_vfp_transfer(word);
    }

    let dreg = vfp_reg(sz, Vd, D);

    if P && !W {
        // VLDR / VSTR (single register, immediate offset).
        let opcode = if L { Opcode::VLDR } else { Opcode::VSTR };
        let offset = imm8 << 2;
        return Ok((opcode, [
            Operand::SIMDReg(dreg),
            Operand::RegDerefPreindexOffset(Reg::from_u8(Rn), offset, U, false),
            Operand::Nothing,
            Operand::Nothing,
        ]));
    }

    // VLDM / VSTM (and the VPUSH / VPOP aliases). addressing: IA when U (P==0), DB when !U (P==1).
    if P == U {
        // P==1,U==1 or P==0,U==0 are not valid load-multiple forms.
        return Err(DecodeError::InvalidOpcode);
    }
    let increment_after = U; // IA vs DB
    // for the double-precision (a1) form an odd imm8 selects the deprecated FLDMX/FSTMX encoding,
    // which carries one extra word; the register count is then `(imm8 - 1) / 2`.
    let is_fldmx = sz && (imm8 & 1 == 1);
    let mut count = if sz {
        if is_fldmx { ((imm8 - 1) >> 1) as u8 } else { (imm8 >> 1) as u8 }
    } else {
        imm8 as u8
    };
    if count == 0 {
        return Err(DecodeError::InvalidOperand);
    }
    // a VLDM/VSTM list holds at most 16 doubleword or 32 singleword registers (and never runs past
    // the end of the register file); clamp the printed list to match capstone on the unpredictable
    // over-long encodings.
    let max_count = if sz { 16 } else { 32 };
    count = count.min(max_count).min(32u8.saturating_sub(dreg.num));
    let list = Operand::SIMDRegList(dreg, count);

    if is_fldmx {
        let opcode = if L { Opcode::VLDMX(increment_after) } else { Opcode::VSTMX(increment_after) };
        return Ok((opcode, [
            Operand::RegWBack(Reg::from_u8(Rn), W),
            list,
            Operand::Nothing,
            Operand::Nothing,
        ]));
    }

    // VPUSH == VSTMDB sp!, VPOP == VLDMIA sp!.
    if Rn == 13 && W {
        if !increment_after && !L {
            return Ok((Opcode::VPUSH, [list, Operand::Nothing, Operand::Nothing, Operand::Nothing]));
        }
        if increment_after && L {
            return Ok((Opcode::VPOP, [list, Operand::Nothing, Operand::Nothing, Operand::Nothing]));
        }
    }

    let opcode = if L { Opcode::VLDM(increment_after) } else { Opcode::VSTM(increment_after) };
    Ok((opcode, [
        Operand::RegWBack(Reg::from_u8(Rn), W),
        list,
        Operand::Nothing,
        Operand::Nothing,
    ]))
}

/// the VFP "other" data-processing group (`cccc 11101D11 ...`): VMOV(imm), VMOV(reg), VABS, VNEG,
/// VSQRT, VCMP(E), VCVT and rounding variants. `opc2` (bits 19:16) plus bit6/bit7 select the op.
#[allow(non_snake_case)]
fn decode_vfp_other(word: u32) -> Result<(Opcode, [Operand; 4]), DecodeError> {
    let sz = (word >> 8) & 1 != 0;
    let D = ((word >> 22) & 1) as u8;
    let M = ((word >> 5) & 1) as u8;
    let Vd = ((word >> 12) & 0b1111) as u8;
    let Vm = (word & 0b1111) as u8;
    let opc2 = ((word >> 16) & 0b1111) as u8;
    let op6 = ((word >> 6) & 1) as u8; // for most, distinguishes the two ops sharing opc2

    let bit7 = (word >> 7) & 1 != 0;
    let _ = op6;

    let dd = vfp_reg(sz, Vd, D);
    let dm = vfp_reg(sz, Vm, M);
    let two = |op: Opcode| {
        Ok((op, [Operand::SIMDReg(dd), Operand::SIMDReg(dm), Operand::Nothing, Operand::Nothing]))
    };

    // bit6 == 0 is the VMOV-immediate form (bits 7:4 == 0000). its display requires expanding the
    // 8-bit encoded value to capstone's `%e` float text, which is deferred for now.
    if (word >> 6) & 1 == 0 {
        return Err(DecodeError::InvalidOpcode);
    }

    match opc2 {
        0b0000 => if bit7 { two(Opcode::VABS) } else { two(Opcode::VMOV) },
        0b0001 => if bit7 { two(Opcode::VSQRT) } else { two(Opcode::VNEG) },
        0b0100 | 0b0101 => {
            // VCMP / VCMPE; opc2 == 0b0101 compares against zero (m field must be 0).
            if opc2 == 0b0101 {
                if Vm != 0 || M != 0 {
                    return Err(DecodeError::InvalidOperand);
                }
                Ok((Opcode::VCMP(bit7), [
                    Operand::SIMDReg(dd), Operand::Imm(0), Operand::Nothing, Operand::Nothing,
                ]))
            } else {
                two(Opcode::VCMP(bit7))
            }
        }
        _ => decode_vfp_cvt(word),
    }
}

/// the VFP conversion sub-group of the "other" data-processing ops (VCVT between float and
/// integer, and between f32 and f64). fixed-point and half-precision forms are deferred.
#[allow(non_snake_case)]
fn decode_vfp_cvt(word: u32) -> Result<(Opcode, [Operand; 4]), DecodeError> {
    let sz = (word >> 8) & 1 != 0; // operation size: 0 -> f32, 1 -> f64
    let D = ((word >> 22) & 1) as u8;
    let M = ((word >> 5) & 1) as u8;
    let Vd = ((word >> 12) & 0b1111) as u8;
    let Vm = (word & 0b1111) as u8;
    let opc2 = ((word >> 16) & 0b1111) as u8;
    let bit7 = (word >> 7) & 1 != 0;

    let fp_ty = if sz { VfpType::F64 } else { VfpType::F32 };

    match opc2 {
        0b0111 => {
            // VCVT between f32 and f64 (only valid with bit7 == 1). the destination has the other
            // width from the operation size.
            if !bit7 {
                return Err(DecodeError::InvalidOpcode);
            }
            let (to, from, dd, dm) = if sz {
                // f64 -> f32
                (VfpType::F32, VfpType::F64, vfp_reg(false, Vd, D), vfp_reg(true, Vm, M))
            } else {
                // f32 -> f64
                (VfpType::F64, VfpType::F32, vfp_reg(true, Vd, D), vfp_reg(false, Vm, M))
            };
            Ok((Opcode::VCVT { to, from, round: false }, [
                Operand::SIMDReg(dd), Operand::SIMDReg(dm), Operand::Nothing, Operand::Nothing,
            ]))
        }
        0b1000 => {
            // VCVT integer -> floating-point. source is a 32-bit integer in a single register.
            let from = if bit7 { VfpType::S32 } else { VfpType::U32 };
            let dd = vfp_reg(sz, Vd, D);
            let dm = vfp_reg(false, Vm, M);
            Ok((Opcode::VCVT { to: fp_ty, from, round: false }, [
                Operand::SIMDReg(dd), Operand::SIMDReg(dm), Operand::Nothing, Operand::Nothing,
            ]))
        }
        0b1100 | 0b1101 => {
            // VCVT floating-point -> integer. destination is a 32-bit integer in a single register;
            // bit7 selects round-toward-zero (vcvt) over round-per-FPSCR (vcvtr).
            let to = if opc2 == 0b1101 { VfpType::S32 } else { VfpType::U32 };
            let round = !bit7;
            let dd = vfp_reg(false, Vd, D);
            let dm = vfp_reg(sz, Vm, M);
            Ok((Opcode::VCVT { to, from: fp_ty, round }, [
                Operand::SIMDReg(dd), Operand::SIMDReg(dm), Operand::Nothing, Operand::Nothing,
            ]))
        }
        _ => Err(DecodeError::InvalidOpcode),
    }
}

/// the VFP register-transfer forms: VMSR, VMRS, VMOV (core<->single, and the 64-bit core-pair
/// forms). the `cccc 1110 ...` scalar/VDUP forms are not yet handled (`InvalidOpcode`).
#[allow(non_snake_case)]
fn decode_vfp_transfer(word: u32) -> Result<(Opcode, [Operand; 4]), DecodeError> {
    let top = (word >> 24) & 0b1111;
    let L = (word >> 20) & 1 != 0;
    let Rt = ((word >> 12) & 0b1111) as u8;

    if top == 0b1110 {
        let cp = (word >> 8) & 0b1111; // 1010 single/sysreg, 1011 scalar/dup
        let sel23 = (word >> 21) & 0b111;
        if cp == 0b1010 {
            if sel23 == 0b111 {
                // VMSR (L==0) / VMRS (L==1). bits 19:16 select the FP system register.
                let reg = ((word >> 16) & 0b1111) as u8;
                if L {
                    return Ok((Opcode::VMRS, [
                        Operand::Reg(Reg::from_u8(Rt)), Operand::Imm(reg as u32),
                        Operand::Nothing, Operand::Nothing,
                    ]));
                } else {
                    return Ok((Opcode::VMSR, [
                        Operand::Imm(reg as u32), Operand::Reg(Reg::from_u8(Rt)),
                        Operand::Nothing, Operand::Nothing,
                    ]));
                }
            }
            if sel23 == 0b000 {
                // VMOV single <-> core register.
                let N = ((word >> 7) & 1) as u8;
                let Vn = ((word >> 16) & 0b1111) as u8;
                let sn = vfp_reg(false, Vn, N);
                if L {
                    return Ok((Opcode::VMOV, [
                        Operand::Reg(Reg::from_u8(Rt)), Operand::SIMDReg(sn),
                        Operand::Nothing, Operand::Nothing,
                    ]));
                } else {
                    return Ok((Opcode::VMOV, [
                        Operand::SIMDReg(sn), Operand::Reg(Reg::from_u8(Rt)),
                        Operand::Nothing, Operand::Nothing,
                    ]));
                }
            }
        }
        if cp == 0b1011 {
            // VMOV (core<->scalar) and VDUP (core->SIMD). the element size comes from bit22 and
            // bit5; the lane index from bit21 and (for smaller sizes) bit6/bit5.
            let bit22 = (word >> 22) & 1;
            let bit21 = ((word >> 21) & 1) as u8;
            let bit6 = ((word >> 6) & 1) as u8;
            let bit5 = ((word >> 5) & 1) as u8;
            let bit23 = (word >> 23) & 1;
            let N = ((word >> 7) & 1) as u8;
            let Vn = ((word >> 16) & 0b1111) as u8;
            let dn = SIMDReg::new(SIMDSizeCode::D, (N << 4) | Vn);

            // (size, lane) from the element-size encoding.
            let (size, lane) = if bit22 == 1 {
                (8u8, (bit21 << 2) | (bit6 << 1) | bit5)
            } else if bit5 == 1 {
                (16u8, (bit21 << 1) | bit6)
            } else {
                (32u8, bit21)
            };

            if !L && bit23 == 1 {
                // VDUP: broadcast Rt across Dd (Q==0) or Qd (Q==1).
                if (word >> 6) & 1 != 0 || word & 0xf != 0 {
                    return Err(DecodeError::InvalidOpcode);
                }
                let Q = (word >> 21) & 1 != 0;
                let vsize = if bit22 == 1 { 8 } else if bit5 == 1 { 16 } else { 32 };
                // for VDUP the destination is Vd=bits[19:16] with the high bit in bit7 (== N here).
                let dd_num = (N << 4) | Vn;
                let dd = if Q {
                    SIMDReg::new(SIMDSizeCode::Q, dd_num >> 1)
                } else {
                    SIMDReg::new(SIMDSizeCode::D, dd_num)
                };
                return Ok((Opcode::VDUP(SIMDDataType::Any(vsize)), [
                    Operand::SIMDReg(dd), Operand::Reg(Reg::from_u8(Rt)),
                    Operand::Nothing, Operand::Nothing,
                ]));
            }

            if L {
                // VMOV lane -> core. 8/16-bit forms carry a signedness (bit23 == U); 32-bit does not.
                let dt = if size == 32 {
                    SIMDDataType::Any(32)
                } else if bit23 == 1 {
                    SIMDDataType::Unsigned(size)
                } else {
                    SIMDDataType::Signed(size)
                };
                return Ok((Opcode::VMOVLane(dt), [
                    Operand::Reg(Reg::from_u8(Rt)), Operand::SIMDRegLane(dn, lane),
                    Operand::Nothing, Operand::Nothing,
                ]));
            } else {
                // VMOV core -> lane.
                return Ok((Opcode::VMOVLane(SIMDDataType::Any(size)), [
                    Operand::SIMDRegLane(dn, lane), Operand::Reg(Reg::from_u8(Rt)),
                    Operand::Nothing, Operand::Nothing,
                ]));
            }
        }
        return Err(DecodeError::InvalidOpcode);
    }

    if top == 0b1100 {
        // 64-bit VMOV: `cccc 1100 010o uuuu tttt 101s 00m1 mmmm`. reject anything not matching that
        // fixed field layout (e.g. the ARMv8-M vlldm/vlstm encodings share the top bits).
        if (word >> 21) & 0b111 != 0b010 || (word >> 6) & 0b11 != 0 || (word >> 4) & 1 == 0 {
            return Err(DecodeError::InvalidOpcode);
        }
        // two core registers <-> a doubleword register or two consecutive singles.
        let sz = (word >> 8) & 1 != 0; // 0 -> two singles, 1 -> one double
        let M = ((word >> 5) & 1) as u8;
        let Vm = (word & 0b1111) as u8;
        let Rt2 = ((word >> 16) & 0b1111) as u8;
        if sz {
            let dm = vfp_reg(true, Vm, M);
            if L {
                return Ok((Opcode::VMOV, [
                    Operand::Reg(Reg::from_u8(Rt)), Operand::Reg(Reg::from_u8(Rt2)),
                    Operand::SIMDReg(dm), Operand::Nothing,
                ]));
            } else {
                return Ok((Opcode::VMOV, [
                    Operand::SIMDReg(dm), Operand::Reg(Reg::from_u8(Rt)),
                    Operand::Reg(Reg::from_u8(Rt2)), Operand::Nothing,
                ]));
            }
        } else {
            let sm = vfp_reg(false, Vm, M);
            let sm1 = SIMDReg::new(SIMDSizeCode::S, sm.num.wrapping_add(1));
            if L {
                return Ok((Opcode::VMOV, [
                    Operand::Reg(Reg::from_u8(Rt)), Operand::Reg(Reg::from_u8(Rt2)),
                    Operand::SIMDReg(sm), Operand::SIMDReg(sm1),
                ]));
            } else {
                return Ok((Opcode::VMOV, [
                    Operand::SIMDReg(sm), Operand::SIMDReg(sm1),
                    Operand::Reg(Reg::from_u8(Rt)), Operand::Reg(Reg::from_u8(Rt2)),
                ]));
            }
        }
    }

    Err(DecodeError::InvalidOpcode)
}

/// decode a Thumb-2 coprocessor / Advanced-SIMD / floating-point 32-bit instruction by mapping it
/// onto the equivalent A32 encoding and reusing the A32 decoders.
///
/// the VFP (coproc `101x`) forms share bits[27:0] with A32 exactly - only the condition differs, and
/// Thumb takes its condition from the enclosing IT state - so we splice in an `AL` condition (`1110`)
/// and call `decode_vfp`. the Advanced-SIMD element/structure load/store forms share bits[23:0] with
/// A32 and differ only in the top byte (Thumb `1111 1001`, A32 `1111 0100`), so we rebuild that byte
/// and call `decode_neon_loadstore`. `hw1`/`hw2` are the two instruction halfwords in order.
pub(crate) fn decode_thumb_coproc(hw1: u16, hw2: u16) -> Result<(Opcode, [Operand; 4]), DecodeError> {
    let thumb = ((hw1 as u32) << 16) | (hw2 as u32);
    let coproc = ((thumb >> 8) & 0b1111) as u8;

    // VFP / floating-point (coproc 101x): identical to A32 in bits[27:0]. force cond = AL (0b1110).
    if coproc & 0b1110 == 0b1010 {
        let arm = 0xe000_0000 | (thumb & 0x0fff_ffff);
        return decode_vfp(arm);
    }

    // Advanced SIMD element/structure load/store: Thumb `1111 1001 ...`, A32 `1111 0100 ...`.
    // bits[23:0] match; rebuild the A32 top byte (0xf4) and reuse the shared decoder.
    if (thumb >> 24) == 0b1111_1001 {
        let arm = 0xf400_0000 | (thumb & 0x00ff_ffff);
        return decode_neon_loadstore(arm);
    }

    // generic coprocessor (STC/LDC/MCR/MRC/CDP...) and NEON data-processing are not yet decoded.
    Err(DecodeError::Incomplete)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[allow(missing_docs)]
pub enum StatusRegMask {
    // Note 0b0000 is unused (as is 0b10000)
    CPSR_C = 0b0001,
    CPSR_X = 0b0010,
    CPSR_XC = 0b0011,
    APSR_G = 0b0100,
    CPSR_SC = 0b0101,
    CPSR_SX = 0b0110,
    CPSR_SXC = 0b0111,
    APSR_NZCVQ = 0b1000,
    CPSR_FC = 0b1001,
    CPSR_FX = 0b1010,
    CPSR_FXC = 0b1011,
    APSR_NZCVQG = 0b1100,
    CPSR_FSC = 0b1101,
    CPSR_FSX = 0b1110,
    CPSR_FSXC = 0b1111,
    SPSR = 0b10000,
    SPSR_C = 0b10001,
    SPSR_X = 0b10010,
    SPSR_XC = 0b10011,
    SPSR_S = 0b10100,
    SPSR_SC = 0b10101,
    SPSR_SX = 0b10110,
    SPSR_SXC = 0b10111,
    SPSR_F = 0b11000,
    SPSR_FC = 0b11001,
    SPSR_FX = 0b11010,
    SPSR_FXC = 0b11011,
    SPSR_FS = 0b11100,
    SPSR_FSC = 0b11101,
    SPSR_FSX = 0b11110,
    SPSR_FSXC = 0b11111,
}

impl StatusRegMask {
    fn from_raw(raw: u8) -> Result<StatusRegMask, DecodeError> {
        if raw == 0 {
            // invalid status reg mask value
            return Err(DecodeError::InvalidOperand);
        }
        Ok([
            StatusRegMask::CPSR_C, // actually unreachable
            StatusRegMask::CPSR_C,
            StatusRegMask::CPSR_X,
            StatusRegMask::CPSR_XC,
            StatusRegMask::APSR_G,
            StatusRegMask::CPSR_SC,
            StatusRegMask::CPSR_SX,
            StatusRegMask::CPSR_SXC,
            StatusRegMask::APSR_NZCVQ,
            StatusRegMask::CPSR_FC,
            StatusRegMask::CPSR_FX,
            StatusRegMask::CPSR_FXC,
            StatusRegMask::APSR_NZCVQG,
            StatusRegMask::CPSR_FSC,
            StatusRegMask::CPSR_FSX,
            StatusRegMask::CPSR_FSXC,
            StatusRegMask::SPSR,
            StatusRegMask::SPSR_C,
            StatusRegMask::SPSR_X,
            StatusRegMask::SPSR_XC,
            StatusRegMask::SPSR_S,
            StatusRegMask::SPSR_SC,
            StatusRegMask::SPSR_SX,
            StatusRegMask::SPSR_SXC,
            StatusRegMask::SPSR_F,
            StatusRegMask::SPSR_FC,
            StatusRegMask::SPSR_FX,
            StatusRegMask::SPSR_FXC,
            StatusRegMask::SPSR_FS,
            StatusRegMask::SPSR_FSC,
            StatusRegMask::SPSR_FSX,
            StatusRegMask::SPSR_FSXC,
        ][raw as usize])
    }
}

/// an operand in an `arm` instruction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "non-exhaustive-enums", non_exhaustive)]
pub enum Operand {
    /// a general-purpose register.
    Reg(Reg),
    /// a general-purpose register, with writeback. these generally imply an increment by width of
    /// a memort operand, depending on the instruction.
    RegWBack(Reg, bool),
    /// a list of registers specified as a bitmask from bits 0 to 15.
    RegList(u16),
    /// a memory access, dereferencing a general-purpose register.
    RegDeref(Reg),
    /// a memory access, dereferencing a shifted general-purpose register with register or
    /// immediate offset.
    RegShift(RegShift),
    /// a memory access of a register, post-indexed a register shifted by register or immediate.
    /// the first bool indicates if the shifted-register is added or subtracted ot the base
    /// register, while the second bool indicates if the resulting address is written back to the
    /// base register.
    RegDerefPostindexRegShift(Reg, RegShift, bool, bool), // add/sub, wback
    /// a memory access of a register, pre-indexed with a register shifted by register or
    /// immediate. the first bool indicates if the shifted-register is added or subtracted ot the
    /// base register, while the second bool indicates if the resulting address is written back to
    /// the base register.
    RegDerefPreindexRegShift(Reg, RegShift, bool, bool), // add/sub, wback
    /// a memory access of a register, post-indexed with an immediate. the first bool indicates if
    /// the shifted-register is added or subtracted ot the base register, while the second bool
    /// indicates if the resulting address is written back to the base register.
    RegDerefPostindexOffset(Reg, u16, bool, bool), // add/sub, wback
    /// a memory access of a register, pre-indexed with an immediate. the first bool indicates if
    /// the shifted-register is added or subtracted ot the base register, while the second bool
    /// indicates if the resulting address is written back to the base register.
    RegDerefPreindexOffset(Reg, u16, bool, bool), // add/sub, wback
    /// a memory access of a register, post-indexed with a register. the first bool indicates if the
    /// shifted-register is added or subtracted ot the base register, while the second bool
    /// indicates if the resulting address is written back to the base register.
    RegDerefPostindexReg(Reg, Reg, bool, bool), // add/sub, wback
    /// a memory access of a register, pre-indexed with a register. the first bool indicates if the
    /// shifted-register is added or subtracted ot the base register, while the second bool
    /// indicates if the resulting address is written back to the base register.
    RegDerefPreindexReg(Reg, Reg, bool, bool), // add/sub, wback
    /// a 12-bit immediate, stored in a `u16`.
    Imm12(u16),
    /// a 32-bit immediate, stored in a `u32`.
    Imm32(u32),
    /// a pc-relative branch, with 32-bit signed offset, left-shifted by 2.
    BranchOffset(i32),
    /// a pc-relative branch, with 32-bit signed offset, left-shifted by 1.
    BranchThumbOffset(i32),
    /// a coprocessor index.
    #[deprecated(
        since = "0.3.2",
        note = "`Coprocessor` was prematurely added: `CoprocOption` the operand used to indicate \
                coprocessor selection and has always been the variant used as such"
    )]
    Coprocessor(u8),
    /// a coprocessor option number.
    CoprocOption(u8),
    /// an `arm` control register.
    CReg(CReg),
    /// an `arm` banked register, either `usr` (general-purpose) bank or one of the alternate sets
    /// of `arm` registers.
    BankedReg(Bank, Reg),
    /// `spsr` in some `arm` register bank.
    BankedSPSR(Bank),
    /// a mask of bits for the `spsr` register.
    StatusRegMask(StatusRegMask),
    /// the `apsr` register.
    APSR,
    /// the `spsr` register.
    SPSR,
    /// the `cpsr` register.
    CPSR,
    /// a VFP/Advanced-SIMD register (`s0`-`s31`, `d0`-`d31`, or `q0`-`q15`).
    SIMDReg(SIMDReg),
    /// a single lane of a VFP/Advanced-SIMD register, e.g. `d0[1]`. the `u8` is the lane index.
    SIMDRegLane(SIMDReg, u8),
    /// a contiguous list of VFP/Advanced-SIMD registers, e.g. `{d0, d1, d2, d3}`, as used by the
    /// extension-register load/store instructions (`vldm`/`vstm`/`vpush`/`vpop`). fields are the
    /// first register and the count of registers in the list.
    SIMDRegList(SIMDReg, u8),
    /// a NEON structure-load/store memory operand: `[rn]`, `[rn:align]`, `[rn]!`, or `[rn], rm`.
    SIMDDeref {
        /// the base register holding the address.
        base: Reg,
        /// the alignment in bits (0 = unaligned).
        align: u16,
        /// the `!` writeback form (encoded by Rm == SP).
        wback: bool,
        /// the post-indexed register form (any other Rm), added to the base after the access.
        post: Option<Reg>,
    },
    /// a bare immediate with no `#`-vs-value reinterpretation, printed as written. used for the
    /// VFP `vmov` immediate encodings and fbits counts.
    Imm(u32),
    /// "no operand". since an instruction's `operands` array is always four entries, this is used
    /// to fill space, if any, after recording an instruction's extant operands.
    Nothing,
}

/// a trait describing functions in support of processing operands
///
/// this is interesting more for future implementation of `yaxpeax-arm`: operands currently are
/// backed by an `Operand` enum, which means that operating on an operand involves potentially more
/// copies and data management than strictly necessary. in the future, operands may be described
/// more granularly, where `OperandVisitor` is the stable interface to the pre-enum constituent
/// parts of operands.
///
/// see `yaxpeax-x86`'s equivalent trait for examples of that direction.
pub trait OperandVisitor {
    /// the result for successful processing of an operand. for formatting, as an example, this is
    /// likely `()`.
    type Ok;
    /// the result for an error in processing an operand.
    type Error;

    /// process an operand that is a simple general purpose register.
    fn visit_reg(&mut self, reg: Reg) -> Result<Self::Ok, Self::Error>;
    /// process an operand that is a simple general purpose register with writeback.
    fn visit_reg_wback(&mut self, reg: Reg, wback: bool) -> Result<Self::Ok, Self::Error>;
    /// process an operand that is a list of registers.
    fn visit_reglist(&mut self, list: u16) -> Result<Self::Ok, Self::Error>;
    /// process an operand that is a memory access through a general purpose register.
    fn visit_reg_deref(&mut self, reg: Reg) -> Result<Self::Ok, Self::Error>;
    /// process an operand that is a shifted general pupose register.
    fn visit_reg_shift(&mut self, reg_shift: RegShift) -> Result<Self::Ok, Self::Error>;
    /// process an operand that is the dereference of a register, afterward incremented by index.
    fn visit_reg_deref_postindex_reg_shift(&mut self, base: Reg, index: RegShift, add: bool, wback: bool) -> Result<Self::Ok, Self::Error>;
    /// process an operand that is the dereference of a register incremented by index.
    fn visit_reg_deref_preindex_reg_shift(&mut self, base: Reg, index: RegShift, add: bool, wback: bool) -> Result<Self::Ok, Self::Error>;
    /// process an operand that is the dereference of a register, afterward incremented by offset.
    fn visit_reg_deref_postindex_offset(&mut self, base: Reg, offset: u16, add: bool, wback: bool) -> Result<Self::Ok, Self::Error>;
    /// process an operand that is the dereference of a register incremented by offset.
    fn visit_reg_deref_preindex_offset(&mut self, base: Reg, offset: u16, add: bool, wback: bool) -> Result<Self::Ok, Self::Error>;
    /// process an operand that is the dereference of a register, afterward incremented by offset.
    fn visit_reg_deref_postindex_reg(&mut self, base: Reg, offset: Reg, add: bool, wback: bool) -> Result<Self::Ok, Self::Error>;
    /// process an operand that is the dereference of a register incremented by offset.
    fn visit_reg_deref_preindex_reg(&mut self, base: Reg, offset: Reg, add: bool, wback: bool) -> Result<Self::Ok, Self::Error>;
    /// process an operand that is a 12-bit immediate.
    fn visit_imm12(&mut self, imm: u16) -> Result<Self::Ok, Self::Error>;
    /// process an operand that is a 32-bit immediate.
    fn visit_imm32(&mut self, imm: u32) -> Result<Self::Ok, Self::Error>;
    /// process an operand that is a branch with i32 offset.
    fn visit_branch_offset(&mut self, offset: i32) -> Result<Self::Ok, Self::Error>;
    /// process an operand that is a branch with i32 offset that also exchanges instruction sets.
    ///
    /// this is typically rendered in the same way as `visit_branch_offset` but is a distinct
    /// helper due to support uses emphasizing the ISA-changing behavior.
    fn visit_blx_offset(&mut self, offset: i32) -> Result<Self::Ok, Self::Error>;
    /// process an operand that is a coprocessor option access. not exactly clear what this means,
    /// this may need to get foled into an opcode or redefinition of the operand.
    fn visit_coprocessor_option(&mut self, nr: u8) -> Result<Self::Ok, Self::Error>;
    /// process an operand that is a control register.
    fn visit_creg(&mut self, creg: CReg) -> Result<Self::Ok, Self::Error>;
    /// process an operand that is a banked register.
    fn visit_banked_reg(&mut self, bank: Bank, reg: u16) -> Result<Self::Ok, Self::Error>;
    /// process an operand that is a banked version of `SPSR`.
    fn visit_banked_spsr(&mut self, bank: Bank) -> Result<Self::Ok, Self::Error>;
    /// process an operand that is some set of bits out of a status register.
    fn visit_status_reg_mask(&mut self, mask: StatusRegMask) -> Result<Self::Ok, Self::Error>;
    /// process an operand that is `APSR`.
    fn visit_apsr(&mut self) -> Result<Self::Ok, Self::Error>;
    /// process an operand that is `SPSR`.
    fn visit_spsr(&mut self) -> Result<Self::Ok, Self::Error>;
    /// process an operand that is `CPSR`.
    fn visit_cpsr(&mut self) -> Result<Self::Ok, Self::Error>;
    /// process an operand that is a VFP/Advanced-SIMD register.
    fn visit_simd_reg(&mut self, reg: SIMDReg) -> Result<Self::Ok, Self::Error>;
    /// process an operand that is a single lane of a VFP/Advanced-SIMD register.
    fn visit_simd_reg_lane(&mut self, reg: SIMDReg, lane: u8) -> Result<Self::Ok, Self::Error>;
    /// process an operand that is a contiguous list of VFP/Advanced-SIMD registers.
    fn visit_simd_reg_list(&mut self, first: SIMDReg, count: u8) -> Result<Self::Ok, Self::Error>;
    /// process an operand that is a NEON structure-load/store memory reference.
    fn visit_simd_deref(&mut self, base: Reg, align: u16, wback: bool, post: Option<Reg>) -> Result<Self::Ok, Self::Error>;
    /// process an operand that is a bare immediate printed as written.
    fn visit_imm(&mut self, imm: u32) -> Result<Self::Ok, Self::Error>;
    /// process an operand that is not defined above. there are no parameters as for an unknown
    /// operand kind there is no appropriate tuple of values to provide.
    fn visit_other(&mut self) -> Result<Self::Ok, Self::Error>;
}

/// a register bank for a register in `armv7` or below.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum Bank {
    Usr,
    Fiq,
    Irq,
    Svc,
    Abt,
    Und,
    Mon,
    Hyp,
}

/// a `armv7` or below instruction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Instruction {
    /// the condition code for this instruction, defaults to `AL` if the instruction is
    /// unconditional.
    pub condition: ConditionCode,
    /// the opcode of this instruction.
    pub opcode: Opcode,
    /// operands for the decoded instruction. operands are populated from index 0, to 1, 2, and 3.
    /// operands from the instruction are non-`Operand::Nothing`.
    pub operands: [Operand; 4],
    /// does this instruction update flags, while variants that do not update flags exist?
    pub s: bool,
    /// is this a 32-bit thumb instruction?
    pub wide: bool,
    /// and if it is a 32-bit thumb instruction, should the .w suffix be shown?
    pub thumb_w: bool,
    /// and generally speaking, was this just a thumb-encoded instruction?
    pub thumb: bool,
}

/// the kinds of errors possibly encountered in trying to decode an `armv7` or below instruction.
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum DecodeError {
    /// the input was insufficient to decode a full instruction. for non-thumb instructions, this means
    /// the input was not at least four bytes long. for thumb instructions, the input was either
    /// not two bytes, or not four bytes, depending on how much the instruction would need.
    ExhaustedInput,
    /// the instruction encodes an opcode that is not valid.
    InvalidOpcode,
    /// the instruction encodes an operand that is not valid.
    InvalidOperand,
    /// `yaxpeax-arm` doesn't know how to decode this, but it may be a valid instruction. the
    /// instruction decoder is not complete, sorry. :(
    ///
    /// in practice this typically indicates some kinds of coprocessor instruction, or `ARMv7` SIMD
    /// instruction.
    Incomplete,
    /// the instruction includes reserved bits that were not set as required.
    Nonconforming,
    /// the input encodes an instruction that is explicitly undefined.
    Undefined,
    /// the input encodes an instruction with unpredictable behavior.
    Unpredictable,
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f:  &mut fmt::Formatter) -> fmt::Result {
        use yaxpeax_arch::DecodeError;
        f.write_str(self.description())
    }
}

#[cfg(feature = "std")]
extern crate std;
#[cfg(feature = "std")]
impl std::error::Error for DecodeError {
    fn description(&self) -> &str {
        <Self as yaxpeax_arch::DecodeError>::description(self)
    }
}

impl From<ReadError> for DecodeError {
    fn from(_e: ReadError) -> DecodeError {
        DecodeError::ExhaustedInput
    }
}

impl yaxpeax_arch::DecodeError for DecodeError {
    fn data_exhausted(&self) -> bool { self == &DecodeError::ExhaustedInput }
    fn bad_opcode(&self) -> bool { self == &DecodeError::InvalidOpcode }
    fn bad_operand(&self) -> bool {
        self == &DecodeError::InvalidOperand || self == &DecodeError::Unpredictable
    }
    fn description(&self) -> &'static str {
        match self {
            DecodeError::ExhaustedInput => "exhausted input",
            DecodeError::InvalidOpcode => "invalid opcode",
            DecodeError::InvalidOperand => "invalid operand",
            DecodeError::Incomplete => "incomplete decoder",
            DecodeError::Nonconforming => "invalid reserved bits",
            DecodeError::Undefined => "undefined encoding",
            DecodeError::Unpredictable => "unpredictable instruction",
        }
    }
}

impl yaxpeax_arch::Instruction for Instruction {
    // TODO: this is wrong!!
    fn well_defined(&self) -> bool { true }
}

impl Default for Instruction {
    fn default() -> Self {
        Instruction {
            condition: ConditionCode::AL,
            opcode: Opcode::Invalid,
            operands: [Operand::Nothing, Operand::Nothing, Operand::Nothing, Operand::Nothing],
            s: false,
            thumb_w: false,
            wide: false,
            thumb: false,
        }
    }
}

impl Instruction {
    fn set_s(&mut self, value: bool) {
        self.s = value;
    }
    /// does this instruction set status flags?
    pub fn s(&self) -> bool { self.s }
    pub(crate) fn set_w(&mut self, value: bool) {
        self.thumb_w = value;
    }
    /// was this instruction encoded in `thumb` mode and still 4 bytes, *and* requires a `.w`
    /// suffix on the opcode?
    pub fn w(&self) -> bool { self.thumb_w }
    pub(crate) fn set_wide(&mut self, value: bool) {
        self.wide = value;
    }
    /// was this instruction encoded in `thumb` mode and still 4 bytes?
    pub fn wide(&self) -> bool { self.wide }
    pub(crate) fn set_thumb(&mut self, value: bool) {
        self.thumb = value;
    }
    /// was this instruction encoded in `thumb` mode?
    pub fn thumb(&self) -> bool { self.thumb }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        #[allow(deprecated)]
        self.contextualize(&NoColors, 0, Some(&NoContext), f)
    }
}

impl LengthedInstruction for Instruction {
    type Unit = AddressDiff<<ARMv7 as Arch>::Address>;
    fn min_size() -> Self::Unit {
        // TODO: this is contingent on the decoder mode...
        AddressDiff::from_const(4)
    }
    fn len(&self) -> Self::Unit {
        if self.thumb && !self.wide {
            AddressDiff::from_const(2)
        } else {
            AddressDiff::from_const(4)
        }
    }
}

/// a condition code for am `armv7` or below instruction.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum ConditionCode {
    EQ,
    NE,
    HS,
    LO,
    MI,
    PL,
    VS,
    VC,
    HI,
    LS,
    GE,
    LT,
    GT,
    LE,
    AL
}

impl ConditionCode {
    fn name(&self) -> &'static [u8; 2] {
        match self {
            ConditionCode::EQ => &[b'e', b'q'],
            ConditionCode::NE => &[b'n', b'e'],
            ConditionCode::HS => &[b'h', b's'],
            ConditionCode::LO => &[b'l', b'o'],
            ConditionCode::MI => &[b'm', b'i'],
            ConditionCode::PL => &[b'p', b'l'],
            ConditionCode::VS => &[b'v', b's'],
            ConditionCode::VC => &[b'v', b'c'],
            ConditionCode::HI => &[b'h', b'i'],
            ConditionCode::LS => &[b'l', b's'],
            ConditionCode::GE => &[b'g', b'e'],
            ConditionCode::LT => &[b'l', b't'],
            ConditionCode::GT => &[b'g', b't'],
            ConditionCode::LE => &[b'l', b'e'],
            ConditionCode::AL => &[b'a', b'l'],
        }
    }
}

impl Display for ConditionCode {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        if *self != ConditionCode::AL {
            use core::fmt::Write;
            f.write_char(self.name()[0] as char)?;
            f.write_char(self.name()[1] as char)?;
        }

        Ok(())
    }
}

impl ConditionCode {
    #[inline]
    fn build(value: u8) -> ConditionCode {
        match value {
            0b0000 => ConditionCode::EQ,
            0b0001 => ConditionCode::NE,
            0b0010 => ConditionCode::HS,
            0b0011 => ConditionCode::LO,
            0b0100 => ConditionCode::MI,
            0b0101 => ConditionCode::PL,
            0b0110 => ConditionCode::VS,
            0b0111 => ConditionCode::VC,
            0b1000 => ConditionCode::HI,
            0b1001 => ConditionCode::LS,
            0b1010 => ConditionCode::GE,
            0b1011 => ConditionCode::LT,
            0b1100 => ConditionCode::GT,
            0b1101 => ConditionCode::LE,
            0b1110 => ConditionCode::AL,
            _ => {
                // this means the argument `value` must never be outside [0,15]
                // which itself means this function shouldn't be public
                unreachable!();
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[allow(dead_code)]
enum DecodeMode {
    User,
    FIQ,
    IRQ,
    Supervisor,
    Monitor,
    Abort,
    Hyp,
    Undefined,
    System,
    /// Catch-all mode to try decoding all ARM instructions. Some instructions are `UNDEFINED` or
    /// `UNPREDICTABLE` in some modes, but `Any` will attempt to decode all.
    Any,
}

impl Default for DecodeMode {
    fn default() -> Self {
        DecodeMode::Any
    }
}

impl DecodeMode {
    fn is_user(&self) -> bool {
        match self {
            DecodeMode::Any |
            DecodeMode::User => true,
            _ => false
        }
    }
    #[allow(dead_code)]
    fn is_supervisor(&self) -> bool {
        match self {
            DecodeMode::Any |
            DecodeMode::Supervisor => true,
            _ => false
        }
    }
    fn is_hyp(&self) -> bool {
        match self {
            DecodeMode::Any |
            DecodeMode::Hyp => true,
            _ => false
        }
    }
    fn is_system(&self) -> bool {
        match self {
            DecodeMode::Any |
            DecodeMode::System => true,
            _ => false
        }
    }
    fn is_any(&self) -> bool {
        match self {
            DecodeMode::Any => true,
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
enum ARMVersion {
    v4,
    v5,
    v6,
    v6t2,
    v7,
    v7ve,
    v7vese,
    Any,
}

impl Default for ARMVersion {
    fn default() -> Self {
        ARMVersion::Any
    }
}

// nothing checks/rejects by arm version yet, but.. soon....
/// a struct with decode configuration for `ARMv7` and below. the same decoder is used for `thumb`
/// and non-`thumb` modes, and the same instruction struct is used for decoded instructions in
/// either mode.
///
/// NOTE: helper functions here create `InstDecoder` for specific revisions, extensions, or lack
/// thereof, in the supported instruction set. `yaxpeax-arm` does not actually honor these settings
/// yet. this means any `InstDecoder` will decode all known instructions through the latest `ARMv7`
/// extensions.
#[allow(unused)]
#[derive(Debug)]
pub struct InstDecoder {
    mode: DecodeMode,
    version: ARMVersion,
    should_is_must: bool,
    thumb: bool,
}

impl Default for InstDecoder {
    fn default() -> Self {
        Self {
            mode: DecodeMode::Any,
            version: ARMVersion::Any,
            should_is_must: true,
            thumb: false,
        }
    }
}

impl InstDecoder {
    /// set the decoder to decoding in thumb mode as the specified bool provides; `true` means
    /// "yes, decode in `thumb` mode", where `false` means to decode as a normal `arm` instruction.
    pub fn set_thumb_mode(&mut self, thumb: bool) {
        self.thumb = thumb;
    }

    /// set the decoder to decoding in thumb mode as the specified bool provides; `true` means
    /// "yes, decode in `thumb` mode", where `false` means to decode as a normal `arm` instruction.
    ///
    /// (this consumes and returns the `InstDecoder` to support use in chained calls.)`
    pub fn with_thumb_mode(mut self, thumb: bool) -> Self {
        self.set_thumb_mode(thumb);
        self
    }

    /// returns whether the decoder is currently set to decode in Thumb mode
    pub fn in_thumb_mode(&self) -> bool {
        self.thumb
    }

    /// initialize a new `arm` `InstDecoder` with default ("everything") support, but in `thumb`
    /// mode.
    pub fn default_thumb() -> Self {
        Self::default().with_thumb_mode(true)
    }

    /// create an `InstDecoder` that supports only instructions through to `ARMv4`.
    pub fn armv4() -> Self {
        Self {
            mode: DecodeMode::Any,
            version: ARMVersion::v4,
            should_is_must: true,
            thumb: false,
        }
    }

    /// create an `InstDecoder` that supports only instructions through to `ARMv5`.
    pub fn armv5() -> Self {
        Self {
            mode: DecodeMode::Any,
            version: ARMVersion::v5,
            should_is_must: true,
            thumb: false,
        }
    }

    /// create an `InstDecoder` that supports only instructions through to `ARMv6`.
    pub fn armv6() -> Self {
        Self {
            mode: DecodeMode::Any,
            version: ARMVersion::v6,
            should_is_must: true,
            thumb: false,
        }
    }

    /// create an `InstDecoder` that supports only instructions through to `ARMv6t2`.
    pub fn armv6t2() -> Self {
        Self {
            mode: DecodeMode::Any,
            version: ARMVersion::v6t2,
            should_is_must: true,
            thumb: false,
        }
    }

    /// create an `InstDecoder` that supports only instructions through to `ARMv6t2` in thumb mode.
    pub fn armv6t2_thumb() -> Self {
        Self {
            mode: DecodeMode::Any,
            version: ARMVersion::v6t2,
            should_is_must: true,
            thumb: true,
        }
    }

    /// create an `InstDecoder` that supports only instructions through to `ARMv7`.
    pub fn armv7() -> Self {
        Self {
            mode: DecodeMode::Any,
            version: ARMVersion::v7,
            should_is_must: true,
            thumb: false,
        }
    }

    /// create an `InstDecoder` that supports only instructions through to `ARMv7` in thumb mode.
    pub fn armv7_thumb() -> Self {
        Self {
            mode: DecodeMode::Any,
            version: ARMVersion::v7,
            should_is_must: true,
            thumb: true,
        }
    }

    /// create an `InstDecoder` that supports only instructions through to `ARMv7ve`.
    pub fn armv7ve() -> Self {
        Self {
            mode: DecodeMode::Any,
            version: ARMVersion::v7ve,
            should_is_must: true,
            thumb: false,
        }
    }

    /// create an `InstDecoder` that supports only instructions through to `ARMv7ve` in thumb mode.
    pub fn armv7ve_thumb() -> Self {
        Self {
            mode: DecodeMode::Any,
            version: ARMVersion::v7ve,
            should_is_must: true,
            thumb: true,
        }
    }

    /// create an `InstDecoder` that supports only instructions through to `ARMv7vese`.
    pub fn armv7vese() -> Self {
        Self {
            mode: DecodeMode::Any,
            version: ARMVersion::v7vese,
            should_is_must: true,
            thumb: false,
        }
    }

    fn unpredictable(&self) -> Result<(), DecodeError> {
        if self.mode != DecodeMode::Any {
            Err(DecodeError::Unpredictable)
        } else {
            Ok(())
        }
    }
}

#[allow(non_snake_case)]
impl Decoder<ARMv7> for InstDecoder {
    #[inline]
    fn decode_into<T: Reader<<ARMv7 as Arch>::Address, <ARMv7 as Arch>::Word>>(&self, inst: &mut Instruction, words: &mut T) -> Result<(), <ARMv7 as Arch>::DecodeError> {
        inst.set_w(false);
        inst.set_wide(false);
        if self.thumb {
            return thumb::decode_into(&self, inst, words);
        } else {
            inst.set_thumb(false);
        }

        let mut word_bytes = [0u8; 4];
        words.next_n(&mut word_bytes)?;
        let word = u32::from_le_bytes(word_bytes);

        let (cond, opc_upper) = {
            let top_byte = word >> 24;
            (
                ((top_byte >> 4) & 0xf) as u8,
                ((top_byte >> 1) & 0x7) as u8
            )
        };

        if cond == 0b1111 {
            // unconditional instructions, section A5.7/page A5-214
            inst.condition = ConditionCode::AL;
            let op1 = (word >> 20) as u8;
            if op1 >= 0b1000_0000 {
                match (op1 >> 5) & 0b11 {
                    0b00 => {
                        match op1 & 0b101 {
                            0b000 |
                            0b101 => {
                                return Err(DecodeError::InvalidOpcode);
                            }
                            0b100 => {
                                // SRS (see table A5.7, op1 = 0b100xx1x0, page A5-214)
                                if !self.mode.is_any() && self.mode.is_hyp() {
                                    return Err(DecodeError::Undefined);
                                }
                                if self.should_is_must {
                                    if word & 0x000fffe0 != 0x000d0500 {
                                        return Err(DecodeError::Nonconforming);
                                    }
                                }
                                let puxw = (word >> 21) & 0b1111;
                                let P = puxw & 0b1000 != 0;
                                let U = puxw & 0b0100 != 0;
                                let W = puxw & 0b0001 != 0;
                                inst.opcode = Opcode::SRS(P, U);
                                inst.operands = [
                                    Operand::RegWBack(Reg::from_u8(13), W),
                                    // the mode field is 5 bits.
                                    Operand::Imm32(word & 0b11111),
                                    Operand::Nothing,
                                    Operand::Nothing,
                                ];
                            },
                            0b001 => {
                                // RFE (see table A5.7, op1 = 0b100xx0x1, page A5-214)
                                if !self.mode.is_any() && self.mode.is_hyp() {
                                    return Err(DecodeError::Undefined);
                                }
                                if self.should_is_must {
                                    if word & 0xffff != 0x0a00 {
                                        return Err(DecodeError::Nonconforming);
                                    }
                                }
                                let puxw = (word >> 21) & 0b1111;
                                let P = puxw & 0b1000 != 0;
                                let U = puxw & 0b0100 != 0;
                                let W = puxw & 0b0001 != 0;
                                inst.opcode = Opcode::RFE(P, U);
                                inst.operands = [
                                    Operand::RegWBack(Reg::from_u8((word >> 16) as u8 & 0b1111), W),
                                    Operand::Nothing,
                                    Operand::Nothing,
                                    Operand::Nothing,
                                ];
                            }
                            _ => {
                                unreachable!("op1 mask is 0b101 but somehow we got an invalid pattern");
                            }
                        }
                    }
                    0b01 => {
                        inst.opcode = Opcode::BLX;
                        let operand = ((word & 0xffffff) as i32) << 8 >> 7;
                        inst.operands = [
                            // BranchThumbOffset(v) targets addr + (v << 1); the `+ 4` accounts for
                            // the +8 byte A32 pipeline offset (H bit is the low halfword bit).
                            Operand::BranchThumbOffset(
                                (operand | (
                                    ((word >> 24) & 0b1) as i32
                                )) + 4
                            ),
                            Operand::Nothing,
                            Operand::Nothing,
                            Operand::Nothing,
                        ];
                    }
                    0b10 => {
                        // op1=0b110xxxxx, see table A5-23
                        if (word >> 20) & 0b11010 == 0b00000 {
                            // the `not 11000x0{0,1}` cases in table A5-23, MCRR or MRRC
                            // but first check that bit 2 of op1 is in fact 1:
                            if (word >> 20) & 0b00100 != 0 {
                                // actually MCRR or MRRC
                                let CRm = word as u8 & 0b1111;
                                let opc1 = (word >> 4) as u8 & 0b1111;
                                let coproc = (word >> 8) as u8 & 0b1111;
                                if coproc & 0b1110 == 0b1010 {
                                    // TODO: `UNDEFINED`
                                    return Err(DecodeError::InvalidOpcode);
                                }
                                let Rt = (word >> 12) as u8 & 0b1111;
                                let Rt2 = (word >> 16) as u8 & 0b1111;
                                if Rt == 15 || Rt2 == 15 || Rt == Rt2 {
                                    // TODO: actually `UNPREDICTABLE`
                                    return Err(DecodeError::InvalidOperand);
                                }
                                if (word >> 20) & 0b00001 != 0 {
                                    inst.opcode = Opcode::MRRC(coproc, opc1, true);
                                } else {
                                    inst.opcode = Opcode::MCRR(coproc, opc1, true);
                                }
                                inst.operands = [
                                    Operand::Reg(Reg::from_u8(Rt)),
                                    Operand::Reg(Reg::from_u8(Rt2)),
                                    Operand::CReg(CReg::from_u8(CRm)),
                                    Operand::Nothing,
                                ];
                            } else {
                                return Err(DecodeError::InvalidOpcode);
                            }
                        } else {
                            // STC or LDC
                            let pudw = (word >> 21) as u8 & 0b1111;
                            let Rn = (word >> 16) as u8 & 0b1111;
                            let CRd = (word >> 12) as u8 & 0b1111;
                            let coproc = (word >> 8) as u8 & 0b1111;
                            let imm8 = word & 0b11111111;

                            if coproc & 0b1110 == 0b1010 {
                                return Err(DecodeError::InvalidOpcode);
                            }

                            if (word >> 20) & 0b00001 == 0 {
                                // op=110xxxx0, STC
                                // page A8-663

                                if pudw & 0b0010 != 0 {
                                    inst.opcode = Opcode::STCL(coproc, true);
                                } else {
                                    inst.opcode = Opcode::STC(coproc, true);
                                }
                            } else {
                                // op=110xxxx1, LDC
                                // page A8-393

                                if pudw & 0b0010 != 0 {
                                    inst.opcode = Opcode::LDCL(coproc, true);
                                } else {
                                    inst.opcode = Opcode::LDC(coproc, true);
                                }
                            }

                            let P = pudw & 0b1000 != 0;
                            let U = pudw & 0b0100 != 0;
                            let W = pudw & 0b0001 != 0;

                            inst.operands = [
                                Operand::CReg(CReg::from_u8(CRd)),
                                if P {
                                    Operand::RegDerefPreindexOffset(Reg::from_u8(Rn), (imm8 << 2) as u16, U, W)
                                } else {
                                    if W {
                                        // preindex has no wback
                                        Operand::RegDerefPostindexOffset(Reg::from_u8(Rn), (imm8 << 2) as u16, U, false)
                                    } else {
                                        Operand::RegDeref(Reg::from_u8(Rn))
                                    }
                                },
                                if !P && !W {
                                    // TODO: not sure what ldc2{l}'s <option> field really means?
                                    // at this point the invalid encoding and mrrc/mcrr forms have
                                    // been tested, so..
                                    debug_assert!(U);
                                    Operand::CoprocOption(imm8 as u8)
                                } else {
                                    Operand::Nothing
                                },
                                Operand::Nothing,
                            ];
                        }
                    }
                    0b11 => {
                        // We know the instruction looks like this...
                        // |1 1 1 1|1 1 1|x x x x|x|x x x x|x x x x|x x x x x|x x|x|x x x x|
                        //                ^ We need to check that this bit is zero, if any of the
                        //                  instructions decoded in this block are to match
                        //                  correctly.
                        // See A5-214 for the table that shows the required forms for these
                        // instructions.
                        if (op1 >> 4) & 1 != 0 {
                            return Err(DecodeError::InvalidOpcode);
                        }

                        // operands are shared between cdp2 and mcr2/mrc2, but Rt is repurposed as
                        // CRd
                        let CRm = word as u8 & 0b1111;
                        let opc2 = (word >> 5) as u8 & 0b111;
                        let coproc = (word >> 8) as u8 & 0b1111;
                        let Rt = (word >> 12) as u8 & 0b1111;
                        let CRn = (word >> 16) as u8 & 0b1111;

                        if (word >> 4) & 1 == 0 {
                            // CDP2, page A8-356
                            let opc1 = (word >> 20) as u8 & 0b1111;
                            inst.opcode = Opcode::CDP(coproc, opc1, opc2, true);
                            inst.operands = [
                                Operand::CReg(CReg::from_u8(Rt)),
                                Operand::CReg(CReg::from_u8(CRn)),
                                Operand::CReg(CReg::from_u8(CRm)),
                                Operand::Nothing,
                            ];
                        } else {
                            // MCR2/MRC2, page A8-477/A8-493
                            let opc1 = (word >> 21) as u8 & 0b111;
                            if (word >> 20) & 1 == 0 {
                                inst.opcode = Opcode::MCR(coproc, opc1, opc2, true);
                            } else {
                                inst.opcode = Opcode::MRC(coproc, opc1, opc2, true);
                            }
                            inst.operands = [
                                Operand::Reg(Reg::from_u8(Rt)),
                                Operand::CReg(CReg::from_u8(CRn)),
                                Operand::CReg(CReg::from_u8(CRm)),
                                Operand::Nothing,
                            ];
                        }
                    }
                    _ => {
                        unreachable!("op1 is two bits");
                    }
                }
            } else {
                // op1=0xxxxxxx, "Memory hints, Advanced SIMD instructions, and miscellaneous
                // instructions on pge A5-215"
                if (word >> 24) & 0b1111 == 0b0100 {
                    // NEON structure load/store (1111 0100 ...).
                    let (opcode, operands) = decode_neon_loadstore(word)?;
                    inst.opcode = opcode;
                    inst.operands = operands;
                    return Ok(());
                }
                return Err(DecodeError::Incomplete);
            }
            return Ok(());
        } else {
            inst.condition = ConditionCode::build(cond);
        }

        // distinction at this point is on page A5-192
        match opc_upper {
            0b000 => {
                // the instruction looks like
                // |c o n d|0 0 0|x x x x|x|x x x x|x x x x|x x x x x|x x|x|x x x x|
                let (s, opcode) = {
                    let part = word >> 20;
                    (
                        (part & 0x01) == 1,
                        ((part >> 1) & 0x0f) as u8
                    )
                };

                if (word & 0b10010000) == 0b10010000 {
                    // the instruction looks like
                    // |c o n d|0 0 0|x x x x|x|x x x x|x x x x|x x x x 1|x x|1|x x x x|
                    // which is a category of multiplies and extra load/store
                    if (word & 0x0f0000f0) == 0x00000090 {
                    // |c o n d|0 0 0 0|x x x x x x x x x x x x x x x x|1 0 0 1|x x x x|
                        // Multiply instruction extension space
                        // (page A5-200)
                        let op = ((word >> 20) & 0x0f) as u8;
                        let s = (op & 1) == 1;
                        let op = op >> 1;
                        let R = [
                            (word & 0x0f) as u8,
                            ((word >> 8) & 0x0f) as u8,
                            ((word >> 12) & 0x0f) as u8,
                            ((word >> 16) & 0x0f) as u8
                        ];
                        inst.set_s(s);
                        match op {
                            0b000 => {
                                inst.opcode = Opcode::MUL;
                                inst.operands = [
                                    Operand::Reg(Reg::from_u8(R[3])),
                                    Operand::Reg(Reg::from_u8(R[0])),
                                    Operand::Reg(Reg::from_u8(R[1])),
                                    Operand::Nothing,
                                ];
                            },
                            0b001 => {
                                inst.opcode = Opcode::MLA;
                                inst.operands = [
                                    Operand::Reg(Reg::from_u8(R[3])),
                                    Operand::Reg(Reg::from_u8(R[0])),
                                    Operand::Reg(Reg::from_u8(R[1])),
                                    Operand::Reg(Reg::from_u8(R[2])),
                                ];
                            },
                            0b010 => {
                                if s {
                                    inst.opcode = Opcode::Invalid;
                                    return Err(DecodeError::InvalidOpcode);
                                }
                                inst.opcode = Opcode::UMAAL;
                                inst.operands = [
                                    Operand::Reg(Reg::from_u8(R[2])),
                                    Operand::Reg(Reg::from_u8(R[3])),
                                    Operand::Reg(Reg::from_u8(R[0])),
                                    Operand::Reg(Reg::from_u8(R[1])),
                                ];
                            },
                            0b011 => {
                                if s {
                                    inst.opcode = Opcode::Invalid;
                                    return Err(DecodeError::InvalidOpcode);
                                }
                                inst.opcode = Opcode::MLS;
                                inst.operands = [
                                    Operand::Reg(Reg::from_u8(R[3])),
                                    Operand::Reg(Reg::from_u8(R[0])),
                                    Operand::Reg(Reg::from_u8(R[1])),
                                    Operand::Reg(Reg::from_u8(R[2])),
                                ];
                            }
                            0b100 => {
                                inst.opcode = Opcode::UMULL;
                                inst.operands = [
                                    Operand::Reg(Reg::from_u8(R[2])),
                                    Operand::Reg(Reg::from_u8(R[3])),
                                    Operand::Reg(Reg::from_u8(R[0])),
                                    Operand::Reg(Reg::from_u8(R[1])),
                                ];
                            }
                            0b101 => {
                                inst.opcode = Opcode::UMLAL;
                                inst.operands = [
                                    Operand::Reg(Reg::from_u8(R[2])),
                                    Operand::Reg(Reg::from_u8(R[3])),
                                    Operand::Reg(Reg::from_u8(R[0])),
                                    Operand::Reg(Reg::from_u8(R[1])),
                                ];
                            }
                            0b110 => {
                                inst.opcode = Opcode::SMULL;
                                inst.operands = [
                                    Operand::Reg(Reg::from_u8(R[2])),
                                    Operand::Reg(Reg::from_u8(R[3])),
                                    Operand::Reg(Reg::from_u8(R[0])),
                                    Operand::Reg(Reg::from_u8(R[1])),
                                ];
                            }
                            0b111 => {
                                inst.opcode = Opcode::SMLAL;
                                inst.operands = [
                                    Operand::Reg(Reg::from_u8(R[2])),
                                    Operand::Reg(Reg::from_u8(R[3])),
                                    Operand::Reg(Reg::from_u8(R[0])),
                                    Operand::Reg(Reg::from_u8(R[1])),
                                ];
                            }
                            _ => { unreachable!("mul opcode is only three bits, got: {:x}", op) }
                        }
                    } else {
                    // |c o n d|0 0 0 u|x x x x x x x x x x x x x x x x|1 u u 1|x x x x|
                    // with at least one of u being 1
                    // misc instructions
                        let (flags, Rn, Rd, HiOffset, op, LoOffset) = {
                            let LoOffset = (word & 0x0f) as u8;
                            let word = word >> 5;
                            let op = (word & 0x3) as u8;
                            let word = word >> 3;
                            let HiOffset = (word & 0x0f) as u8;
                            let word = word >> 4;
                            let Rd = (word & 0x0f) as u8;
                            let word = word >> 4;
                            let Rn = (word & 0x0f) as u8;
                            let word = word >> 4;
                            let flags = (word & 0x1f) as u8;
                            (flags, Rn, Rd, HiOffset, op, LoOffset)
                        };
                        match op {
                            0b00 => {
                    // |c o n d|0 0 0 1|x x x x x x x x x x x x x x x x|1 0 0 1|x x x x|
                                // this is swp or {ld,st}ex, conditional on bit 23
                                // see page A5-203
                                //
                                // the exclusive-access encodings (flags 0b11xxx) fix bits[11:8]
                                // (HiOffset) to 1111. other values here are the ARMv8
                                // load-acquire/store-release family (LDA/STL/LDAEX/STLEX/...),
                                // which is not part of ARMv7 - reject rather than mis-decode as
                                // LDREX/STREX.
                                if flags >= 0b11000 && HiOffset != 0b1111 {
                                    return Err(DecodeError::InvalidOpcode);
                                }
                                match flags {
                                    0b10000 => {
                                        inst.opcode = Opcode::SWP;
                                        inst.operands = [
                                            Operand::Reg(Reg::from_u8(Rd)),
                                            Operand::Reg(Reg::from_u8(LoOffset)),
                                            Operand::RegDeref(Reg::from_u8(Rn)),
                                            Operand::Nothing,
                                        ];
                                    },
                                    0b10001 | 0b10010 | 0b10011 => {
                                        inst.opcode = Opcode::Invalid;
                                        return Err(DecodeError::InvalidOpcode);
                                    }
                                    0b10100 => {
                                        inst.opcode = Opcode::SWPB;
                                        inst.operands = [
                                            Operand::Reg(Reg::from_u8(Rd)),
                                            Operand::Reg(Reg::from_u8(LoOffset)),
                                            Operand::RegDeref(Reg::from_u8(Rn)),
                                            Operand::Nothing,
                                        ];
                                    },
                                    0b10101 | 0b10110 | 0b10111 => {
                                        inst.opcode = Opcode::Invalid;
                                        return Err(DecodeError::InvalidOpcode);
                                    }
                                    0b11000 => {
                                        // TODO: flag v6
                                        inst.opcode = Opcode::STREX;
                                        inst.operands = [
                                            Operand::Reg(Reg::from_u8(Rd)),
                                            Operand::Reg(Reg::from_u8(LoOffset)),
                                            Operand::RegDeref(Reg::from_u8(Rn)),
                                            Operand::Nothing,
                                        ];
                                    }
                                    0b11001 => {
                                        // TODO: flag v6
                                        inst.opcode = Opcode::LDREX;
                                        inst.operands = [
                                            Operand::Reg(Reg::from_u8(Rd)),
                                            Operand::RegDeref(Reg::from_u8(Rn)),
                                            Operand::Nothing,
                                            Operand::Nothing,
                                        ];
                                    }
                                    0b11010 => {
                                        inst.opcode = Opcode::STREXD;
                                        if LoOffset == 0b1110 || (LoOffset & 1 == 1) || Rd == 15 || Rn == 15 {
                                            return Err(DecodeError::InvalidOperand);
                                        }
                                        inst.operands = [
                                            Operand::Reg(Reg::from_u8(Rd)),
                                            Operand::Reg(Reg::from_u8(LoOffset)),
                                            Operand::Reg(Reg::from_u8(LoOffset + 1)),
                                            Operand::RegDeref(Reg::from_u8(Rn)),
                                        ];
                                    }
                                    0b11011 => {
                                        inst.opcode = Opcode::LDREXD;
                                        if Rd == 0b1110 || (Rd & 1 == 1) || Rn == 15 {
                                            return Err(DecodeError::InvalidOperand);
                                        }
                                        inst.operands = [
                                            Operand::Reg(Reg::from_u8(Rd)),
                                            Operand::Reg(Reg::from_u8(Rd + 1)),
                                            Operand::RegDeref(Reg::from_u8(Rn)),
                                            Operand::Nothing,
                                        ];
                                    }
                                    0b11100 => {
                                        inst.opcode = Opcode::STREXB;
                                        inst.operands = [
                                            Operand::Reg(Reg::from_u8(Rd)),
                                            Operand::Reg(Reg::from_u8(LoOffset)),
                                            Operand::RegDeref(Reg::from_u8(Rn)),
                                            Operand::Nothing,
                                        ];
                                    }
                                    0b11101 => {
                                        inst.opcode = Opcode::LDREXB;
                                        inst.operands = [
                                            Operand::Reg(Reg::from_u8(Rd)),
                                            Operand::RegDeref(Reg::from_u8(Rn)),
                                            Operand::Nothing,
                                            Operand::Nothing,
                                        ];
                                    }
                                    0b11110 => {
                                        inst.opcode = Opcode::STREXH;
                                        inst.operands = [
                                            Operand::Reg(Reg::from_u8(Rd)),
                                            Operand::Reg(Reg::from_u8(LoOffset)),
                                            Operand::RegDeref(Reg::from_u8(Rn)),
                                            Operand::Nothing,
                                        ];
                                    }
                                    0b11111 => {
                                        inst.opcode = Opcode::LDREXH;
                                        inst.operands = [
                                            Operand::Reg(Reg::from_u8(Rd)),
                                            Operand::RegDeref(Reg::from_u8(Rn)),
                                            Operand::Nothing,
                                            Operand::Nothing,
                                        ];
                                    }
                                    _ => {
                                        /*
                                         * This is unreachable because we have checked op is b1001,
                                         * meaning the high bit of flags *MUST* be 1.
                                         *
                                         * high bit and mid-bits of op all being 0 was checked
                                         * before reaching here.
                                         */
                                        unreachable!("load/store flags: {:x}", flags);
                                    }
                                }
                            }
                            0b01 => {
                    // |c o n d|0 0 0 x|x x x x x x x x x x x x x x x x|1 0 1 1|x x x x|
                    // page A5-201
                                let P = flags & 0b10000 != 0;
                                let U = flags & 0b01000 != 0;
                                let W = flags & 0b00010 != 0;

                                match flags & 0b00101 {
                                    0b00000 => {
                                        // STRHT or STRH
                                        if !P && W { // flags == 0b0x010
                                            inst.opcode = Opcode::STRHT;
                                        } else {
                                            inst.opcode = Opcode::STRH;
                                        }
                                        inst.operands = [
                                            Operand::Reg(Reg::from_u8(Rd)),
                                            if P {
                                                Operand::RegDerefPreindexReg(Reg::from_u8(Rn), Reg::from_u8(LoOffset), U, W)
                                            } else {
                                                // either this is !P && W, so STRHT, and no wback,
                                                // or this is !W, and no wback
                                                Operand::RegDerefPostindexReg(Reg::from_u8(Rn), Reg::from_u8(LoOffset), U, false)
                                            },
                                            Operand::Nothing,
                                            Operand::Nothing,
                                        ];
                                    }
                                    0b00001 => {
                                        // LDRHT or LDRH
                                        if !P && W { // flags == 0b0x011
                                            inst.opcode = Opcode::LDRHT;
                                        } else {
                                            inst.opcode = Opcode::LDRH;
                                        }
                                        inst.operands = [
                                            Operand::Reg(Reg::from_u8(Rd)),
                                            if P {
                                                Operand::RegDerefPreindexReg(Reg::from_u8(Rn), Reg::from_u8(LoOffset), U, W)
                                            } else {
                                                // either this is !P && W, so LDRHT, and no wback,
                                                // or this is !W, and no wback
                                                Operand::RegDerefPostindexReg(Reg::from_u8(Rn), Reg::from_u8(LoOffset), U, false)
                                            },
                                            Operand::Nothing,
                                            Operand::Nothing,
                                        ];
                                    }
                                    0b00100 => {
                                        // STRHT or STRH
                                        if !P && W { // flags == 0b0x110
                                            inst.opcode = Opcode::STRHT;
                                        } else {
                                            inst.opcode = Opcode::STRH;
                                        }
                                        let imm = (HiOffset << 4) as u16 | LoOffset as u16;
                                        inst.operands = [
                                            Operand::Reg(Reg::from_u8(Rd)),
                                            if P {
                                                Operand::RegDerefPreindexOffset(Reg::from_u8(Rn), imm, U, W)
                                            } else {
                                                // either this is !P && W, so STRHT, and no wback,
                                                // or this is !W, and no wback
                                                Operand::RegDerefPostindexOffset(Reg::from_u8(Rn), imm, U, false)
                                            },
                                            Operand::Nothing,
                                            Operand::Nothing,
                                        ];
                                    }
                                    0b00101 => {
                                        // LDRHT or LDRH
                                        if !P && W { // flags == 0b0x111
                                            inst.opcode = Opcode::LDRHT;
                                        } else {
                                            inst.opcode = Opcode::LDRH;
                                        }
                                        let imm = (HiOffset << 4) as u16 | LoOffset as u16;
                                        inst.operands = [
                                            Operand::Reg(Reg::from_u8(Rd)),
                                            if P {
                                                Operand::RegDerefPreindexOffset(Reg::from_u8(Rn), imm, U, W)
                                            } else {
                                                // either this is !P && W, so LDRHT, and no wback,
                                                // or this is !W, and no wback
                                                Operand::RegDerefPostindexOffset(Reg::from_u8(Rn), imm, U, false)
                                            },
                                            Operand::Nothing,
                                            Operand::Nothing,
                                        ];
                                    }
                                    o => {
                                        unreachable!("all of op2=01 in table A5-10 should be handled, got unexpected pattern {:b}", o);
                                    }
                                }
                                return Ok(());
                            }
                            0b10 => {
                    // |c o n d|0 0 0 x|x x x x x x x x x x x x x x x x|1 1 0 1|x x x x|
                    // page A5-201
                                let P = flags & 0b10000 != 0;
                                let U = flags & 0b01000 != 0;
                                let W = flags & 0b00010 != 0;

                                match flags & 0b00101 {
                                    0b00000 => {
                                        // LDRD or invalid
                                        if !P && W { // flags == 0b0x010
                                            return Err(DecodeError::InvalidOperand);
                                        } else {
                                            inst.opcode = Opcode::LDRD;
                                        }
                                        if self.should_is_must {
                                            if (word >> 8) & 0b1111 == 0 {
                                                return Err(DecodeError::Nonconforming);
                                            }
                                        }
                                        let Rm = word as u8 & 0b1111;
                                        let Rt = (word >> 12) as u8 & 0b1111;
                                        if Rt & 1 != 0 {
                                            return Err(DecodeError::InvalidOperand);
                                        }
                                        if Rt == 15 {
                                            return Err(DecodeError::InvalidOperand);
                                        }
                                        let Rn = (word >> 16) as u8 & 0b1111;
                                        inst.operands = [
                                            Operand::Reg(Reg::from_u8(Rt)),
                                            Operand::Reg(Reg::from_u8(Rt + 1)),
                                            if P {
                                                Operand::RegDerefPreindexReg(Reg::from_u8(Rn), Reg::from_u8(Rm), U, W)
                                            } else {
                                                // either !P & W (invalid) or !W
                                                Operand::RegDerefPostindexReg(Reg::from_u8(Rn), Reg::from_u8(Rm), U, false)
                                            },
                                            Operand::Nothing,
                                        ];
                                    },
                                    0b00001 => {
                                        // LDRSB or LDRSBT
                                        if !P && W { // flags == 0b0x010
                                            inst.opcode = Opcode::LDRSBT;
                                        } else {
                                            inst.opcode = Opcode::LDRSB;
                                        }
                                        if self.should_is_must {
                                            if (word >> 8) & 0b1111 == 0 {
                                                return Err(DecodeError::Nonconforming);
                                            }
                                        }
                                        let Rm = word as u8 & 0b1111;
                                        let Rt = (word >> 12) as u8 & 0b1111;
                                        let Rn = (word >> 16) as u8 & 0b1111;
                                        inst.operands = [
                                            Operand::Reg(Reg::from_u8(Rt)),
                                            if P {
                                                Operand::RegDerefPreindexReg(Reg::from_u8(Rn), Reg::from_u8(Rm), U, W)
                                            } else {
                                                // either !P & W (ldrsbt) or !W
                                                Operand::RegDerefPostindexReg(Reg::from_u8(Rn), Reg::from_u8(Rm), U, false)
                                            },
                                            Operand::Nothing,
                                            Operand::Nothing,
                                        ];
                                    },
                                    0b00100 => {
                                        // LDRD (immediate)
                                        if !P && W { // flags == 0b0x110
                                            return Err(DecodeError::InvalidOperand);
                                        } else {
                                            inst.opcode = Opcode::LDRD;
                                        }
                                        let Rt = (word >> 12) as u8 & 0b1111;
                                        if Rt & 1 != 0 {
                                            return Err(DecodeError::InvalidOperand);
                                        }
                                        if Rt == 14 || Rn == 15 {
                                            return Err(DecodeError::InvalidOperand);
                                        }
                                        let Rn = (word >> 16) as u8 & 0b1111;
                                        let imm = (HiOffset << 4) as u16 | LoOffset as u16;
                                        inst.operands = [
                                            Operand::Reg(Reg::from_u8(Rt)),
                                            Operand::Reg(Reg::from_u8(Rt + 1)),
                                            if P {
                                                Operand::RegDerefPreindexOffset(Reg::from_u8(Rn), imm, U, W)
                                            } else {
                                                // either !P & W (invalid) or !W
                                                Operand::RegDerefPostindexOffset(Reg::from_u8(Rn), imm, U, false)
                                            },
                                            Operand::Nothing,
                                        ];
                                    },
                                    0b00101 => {
                                        // LDRSB or LDRSBT
                                        if !P && W { // flags == 0b0x010
                                            inst.opcode = Opcode::LDRSBT;
                                        } else {
                                            inst.opcode = Opcode::LDRSB;
                                        }
                                        if self.should_is_must {
                                            if (word >> 8) & 0b1111 == 0 {
                                                return Err(DecodeError::Nonconforming);
                                            }
                                        }
                                        let Rt = (word >> 12) as u8 & 0b1111;
                                        let Rn = (word >> 16) as u8 & 0b1111;
                                        let imm = (HiOffset << 4) as u16 | LoOffset as u16;
                                        inst.operands = [
                                            Operand::Reg(Reg::from_u8(Rt)),
                                            if P {
                                                Operand::RegDerefPreindexOffset(Reg::from_u8(Rn), imm, U, W)
                                            } else {
                                                // either !P & W (ldrsbt) or !W
                                                Operand::RegDerefPostindexOffset(Reg::from_u8(Rn), imm, U, false)
                                            },
                                            Operand::Nothing,
                                            Operand::Nothing,
                                        ];
                                    },
                                    _ => { unreachable!("impossible bit pattern"); }
                                }
                            }
                            0b11 => {
                    // |c o n d|0 0 0 x|x x x x x x x x x x x x x x x x|1 1 1 1|x x x x|
                    // page A5-201
                                let P = flags & 0b10000 != 0;
                                let U = flags & 0b01000 != 0;
                                let W = flags & 0b00010 != 0;

                                match flags & 0b00101 {
                                    0b00000 => {
                                        // STRD or invalid
                                        if !P && W { // flags == 0b0x010
                                            return Err(DecodeError::InvalidOperand);
                                        } else {
                                            inst.opcode = Opcode::STRD;
                                        }
                                        if self.should_is_must {
                                            if (word >> 8) & 0b1111 == 0 {
                                                return Err(DecodeError::Nonconforming);
                                            }
                                        }
                                        let Rm = word as u8 & 0b1111;
                                        let Rt = (word >> 12) as u8 & 0b1111;
                                        if Rt & 1 != 0 {
                                            return Err(DecodeError::InvalidOperand);
                                        }
                                        let Rn = (word >> 16) as u8 & 0b1111;
                                        inst.operands = [
                                            Operand::Reg(Reg::from_u8(Rt)),
                                            Operand::Reg(Reg::from_u8(Rt + 1)),
                                            if P {
                                                Operand::RegDerefPreindexReg(Reg::from_u8(Rn), Reg::from_u8(Rm), U, W)
                                            } else {
                                                // either !P & W (invalid) or !W
                                                Operand::RegDerefPostindexReg(Reg::from_u8(Rn), Reg::from_u8(Rm), U, false)
                                            },
                                            Operand::Nothing,
                                        ];
                                    },
                                    0b00001 => {
                                        // LDRSH or LDRSHT
                                        if !P && W { // flags == 0b0x010
                                            inst.opcode = Opcode::LDRSHT;
                                        } else {
                                            inst.opcode = Opcode::LDRSH;
                                        }
                                        if self.should_is_must {
                                            if (word >> 8) & 0b1111 == 0 {
                                                return Err(DecodeError::Nonconforming);
                                            }
                                        }
                                        let Rm = word as u8 & 0b1111;
                                        let Rt = (word >> 12) as u8 & 0b1111;
                                        let Rn = (word >> 16) as u8 & 0b1111;
                                        inst.operands = [
                                            Operand::Reg(Reg::from_u8(Rt)),
                                            if P {
                                                Operand::RegDerefPreindexReg(Reg::from_u8(Rn), Reg::from_u8(Rm), U, W)
                                            } else {
                                                // either !P & W (ldrsht) or !W
                                                Operand::RegDerefPostindexReg(Reg::from_u8(Rn), Reg::from_u8(Rm), U, false)
                                            },
                                            Operand::Nothing,
                                            Operand::Nothing,
                                        ];
                                    },
                                    0b00100 => {
                                        // STRD (immediate)
                                        if !P && W { // flags == 0b0x110
                                            return Err(DecodeError::InvalidOperand);
                                        } else {
                                            inst.opcode = Opcode::STRD;
                                        }
                                        let Rt = (word >> 12) as u8 & 0b1111;
                                        if Rt & 1 != 0 {
                                            return Err(DecodeError::InvalidOperand);
                                        }
                                        if Rt == 15 {
                                            return Err(DecodeError::InvalidOperand);
                                        }
                                        let Rn = (word >> 16) as u8 & 0b1111;
                                        let imm = (HiOffset << 4) as u16 | LoOffset as u16;
                                        inst.operands = [
                                            Operand::Reg(Reg::from_u8(Rt)),
                                            Operand::Reg(Reg::from_u8(Rt + 1)),
                                            if P {
                                                Operand::RegDerefPreindexOffset(Reg::from_u8(Rn), imm, U, W)
                                            } else {
                                                // either !P & W (invalid) or !W
                                                Operand::RegDerefPostindexOffset(Reg::from_u8(Rn), imm, U, false)
                                            },
                                            Operand::Nothing,
                                        ];
                                    },
                                    0b00101 => {
                                        // LDRSH or LDRSHT
                                        if !P && W { // flags == 0b0x010
                                            inst.opcode = Opcode::LDRSHT;
                                        } else {
                                            inst.opcode = Opcode::LDRSH;
                                        }
                                        if self.should_is_must {
                                            if (word >> 8) & 0b1111 == 0 {
                                                return Err(DecodeError::Nonconforming);
                                            }
                                        }
                                        let Rt = (word >> 12) as u8 & 0b1111;
                                        let Rn = (word >> 16) as u8 & 0b1111;
                                        let imm = (HiOffset << 4) as u16 | LoOffset as u16;
                                        inst.operands = [
                                            Operand::Reg(Reg::from_u8(Rt)),
                                            if P {
                                                Operand::RegDerefPreindexOffset(Reg::from_u8(Rn), imm, U, W)
                                            } else {
                                                // either !P & W (ldrsht) or !W
                                                Operand::RegDerefPostindexOffset(Reg::from_u8(Rn), imm, U, false)
                                            },
                                            Operand::Nothing,
                                            Operand::Nothing,
                                        ];
                                    },
                                    _ => { unreachable!("impossible bit pattern"); }
                                }
                            }
                            _ => { unreachable!("op is two bits"); }
                        }
                    }
                } else {
                    // we know this is data processing with imm or reg shift, OR
                    // misc instructions in Figure A5-4

                    if s == false && opcode >= 0b1000 && opcode < 0b1100 {
                        // Data-processing and miscellaneous instructions on page A5-194
                        let op2 = ((word >> 4) & 0x0f) as u8;
                        // the instruction looks like
                        // |c o n d|0 0 0|1 0 x x|0|x x x x|x x x x|x x x x x|x x|x|x x x x|
                        if op2 & 0x08 == 0x00 {
                            let op2 = op2 & 0x07;
                            // |c o n d|0 0 0|1 0 x x|0|x x x x|x x x x|x x x x|0|x x|x|x x x x|
                            // misc instructions (page A5-194)
                            match op2 {
                                0b000 => {
                                    // test B bit (bit 9)
                                    if word & 0b10_0000_0000 != 0 {
                                        // TODO: ARMv7VE flag
                                        // SYSm<4> is the M bit (bit8); SYSm<3:0> is M1 (bits19:16).
                                        let SYSm = (((word >> 8) & 1) << 4) as u8 | ((word >> 16) & 0x0f) as u8;
                                        let R = (word >> 22) & 1;

                                        if opcode & 0b01 == 0b01 {
                                            if self.should_is_must {
                                                if word & 0b1111_1100_0000_0000 != 0xf000 {
                                                    return Err(DecodeError::InvalidOperand);
                                                }
                                            }
                                            inst.opcode = Opcode::MSR;
                                            inst.operands[0] = if let Some(reg) = Reg::from_sysm(R != 0, SYSm) {
                                                reg
                                            } else {
                                                return Err(DecodeError::InvalidOperand);
                                            };
                                            inst.operands[1] = Operand::Reg(Reg::from_u8(word as u8 & 0b1111));
                                            inst.operands[2] = Operand::Nothing;
                                            inst.operands[3] = Operand::Nothing;
                                        } else {
                                            if self.should_is_must {
                                                if word & 0b0000_1100_0000_1111 != 0x0000 {
                                                    return Err(DecodeError::InvalidOperand);
                                                }
                                            }
                                            inst.opcode = Opcode::MRS;
                                            inst.operands[0] = Operand::Reg(Reg::from_u8((word >> 12) as u8 & 0b1111));
                                            inst.operands[1] = if let Some(reg) = Reg::from_sysm(R != 0, SYSm) {
                                                reg
                                            } else {
                                                return Err(DecodeError::InvalidOperand);
                                            };
                                            inst.operands[2] = Operand::Nothing;
                                            inst.operands[3] = Operand::Nothing;
                                        }
                                    } else {
                                        match opcode & 0b11 {
                                            0b00 |
                                            0b10 => {
                                                inst.opcode = Opcode::MRS;
                                                let src = if self.mode.is_system() {
                                                    let R = (word >> 22) & 1 != 0;
                                                    if R {
                                                        Operand::SPSR
                                                    } else {
                                                        Operand::CPSR
                                                    }
                                                } else {
                                                    Operand::APSR
                                                };
                                                inst.operands[0] = Operand::Reg(Reg::from_u8((word >> 12) as u8 & 0b1111));
                                                inst.operands[1] = src;
                                                inst.operands[2] = Operand::Nothing;
                                                inst.operands[3] = Operand::Nothing;
                                            }
                                            0b01 => {
                                                inst.opcode = Opcode::MSR;
                                                let mask = (word >> 16) & 0b1111;
                                                if mask & 0b11 == 0 {
                                                    if self.mode.is_user() {
                                                        inst.operands[0] = Operand::StatusRegMask(StatusRegMask::from_raw(mask as u8)?);
                                                        inst.operands[1] = Operand::Reg(Reg::from_u8(word as u8 & 0b1111));
                                                        inst.operands[2] = Operand::Nothing;
                                                        inst.operands[3] = Operand::Nothing;
                                                    } else {
                                                        // MSR with op == 01, op == xx00, but not
                                                        // user mode
                                                        return Err(DecodeError::InvalidOperand);
                                                    }
                                                } else {
                                                    if self.mode.is_system() {
                                                        // bit 22 is the high bit of opcode, so..
                                                        let R = (word >> 22) as u8 & 1;
                                                        inst.operands[0] = Operand::StatusRegMask(StatusRegMask::from_raw((R << 4) | mask as u8)?);
                                                        inst.operands[1] = Operand::Reg(Reg::from_u8(word as u8 & 0b1111));
                                                        inst.operands[2] = Operand::Nothing;
                                                        inst.operands[3] = Operand::Nothing;
                                                    } else {
                                                        // MSR with op == 01, op != xx00
                                                        return Err(DecodeError::InvalidOperand);
                                                    }
                                                }
                                            },
                                            0b11 => {
                                                if !self.mode.is_system() {
                                                    return Err(DecodeError::InvalidOperand);
                                                }

                                                inst.opcode = Opcode::MSR;
                                                let mask = (word >> 16) & 0b1111;
                                                // bit 22 is the high bit of opcode, so..
                                                let R = (word >> 22) & 1;
                                                inst.operands[0] = Operand::StatusRegMask(StatusRegMask::from_raw((R << 4) as u8 | mask as u8)?);
                                                inst.operands[1] = Operand::Reg(Reg::from_u8(word as u8 & 0b1111));
                                                inst.operands[2] = Operand::Nothing;
                                                inst.operands[3] = Operand::Nothing;
                                            }
                                            _ => {
                                                unreachable!("opcode is masked to two bits here");
                                            }
                                        }
                                    }
                                },
                                0b001 => {
                                    match opcode & 0b11 {
                                        0b01 => {
                                            inst.opcode = Opcode::BX;
                                            inst.operands = [
                                                Operand::Reg(Reg::from_u8(word as u8 & 0b1111)),
                                                Operand::Nothing,
                                                Operand::Nothing,
                                                Operand::Nothing,
                                            ];
                                        }
                                        0b11 => {
                                            inst.opcode = Opcode::CLZ;
                                            inst.operands = [
                                                Operand::Reg(Reg::from_u8((word >> 12) as u8 & 0b1111)),
                                                Operand::Reg(Reg::from_u8(word as u8 & 0b1111)),
                                                Operand::Nothing,
                                                Operand::Nothing,
                                            ];
                                        }
                                        _ => {
                                            return Err(DecodeError::InvalidOpcode);
                                        }
                                    }
                                },
                                0b010 => {
                                    return Err(DecodeError::InvalidOpcode);
                                },
                                0b011 => {
                                    if opcode & 0b11 == 0b01 {
                                        inst.opcode = Opcode::BLX;
                                        inst.operands = [
                                            Operand::Reg(Reg::from_u8(word as u8 & 0x0f)),
                                            Operand::Nothing,
                                            Operand::Nothing,
                                            Operand::Nothing,
                                        ];
                                        return Ok(());
                                    } else {
                                        return Err(DecodeError::InvalidOpcode);
                                    }
                                },
                                0b100 => {
                                    // no row for op2 == 0b100 in table A5-14 (page A5-203)
                                    return Err(DecodeError::InvalidOpcode);
                                },
                                0b101 => {
                                    // TODO: "Saturating addition and subtraction" page A5-200
                                    match (word >> 21) & 0b11 {
                                        0b00 => {
                                            inst.opcode = Opcode::QADD;
                                        },
                                        0b01 => {
                                            inst.opcode = Opcode::QSUB;
                                        },
                                        0b10 => {
                                            inst.opcode = Opcode::QDADD;
                                        },
                                        0b11 => {
                                            inst.opcode = Opcode::QDSUB;
                                        },
                                        _ => {
                                            unreachable!("bit pattern masked by 0b11 but large value observed");
                                        }
                                    }

                                    if self.should_is_must {
                                        if (word >> 8) & 0b1111 != 0 {
                                            return Err(DecodeError::Nonconforming);
                                        }
                                    }

                                    inst.operands = [
                                        Operand::Reg(Reg::from_u8(((word >> 12) & 0b1111) as u8)),
                                        Operand::Reg(Reg::from_u8((word & 0b1111) as u8)),
                                        Operand::Reg(Reg::from_u8(((word >> 16) & 0b1111) as u8)),
                                        Operand::Nothing
                                    ];
                                }
                                0b110 => {
                                    if (word >> 21) & 0b11 != 0b11 {
                                        return Err(DecodeError::InvalidOpcode);
                                    }

                                    // "ERET" page B9-1968, from A5.2.12, page A5-205
                                    if self.should_is_must {
                                        if word & 0b1111_1111_1111_1111 != 0b0000_0000_0000_0110_1110 {
                                            return Err(DecodeError::Nonconforming);
                                        }
                                    }
                                    inst.opcode = Opcode::ERET;
                                    inst.operands = [
                                        Operand::Nothing,
                                        Operand::Nothing,
                                        Operand::Nothing,
                                        Operand::Nothing,
                                    ];
                                },
                                0b111 => {
                                    // "BKPT, HVC, SMC" from table A5-14/page A5-205
                                    match (word >> 21) & 0b11 {
                                        0b00 => {
                                            return Err(DecodeError::InvalidOpcode);
                                        }
                                        0b01 => {
                                            // BKPT
                                            if self.should_is_must {
                                                if (word >> 28) & 0b1111 != 0b1110 {
                                                    // "BKPT must be encoded with AL condition"
                                                    return Err(DecodeError::InvalidOpcode);
                                                }
                                            }
                                            inst.opcode = Opcode::BKPT;
                                            let immlo = word & 0x0f;
                                            let imm = ((word >> 4) & 0xfff0) | immlo;
                                            inst.operands = [
                                                Operand::Imm32(imm),
                                                Operand::Nothing,
                                                Operand::Nothing,
                                                Operand::Nothing,
                                            ];
                                        }
                                        0b10 => {
                                            // HVC
                                            if self.should_is_must {
                                                if (word >> 28) & 0b1111 != 0b1110 {
                                                    // "HVC must be encoded with AL condition"
                                                    return Err(DecodeError::InvalidOpcode);
                                                }
                                            }
                                            inst.opcode = Opcode::HVC;
                                            let immlo = word & 0x0f;
                                            let imm = ((word >> 4) & 0xfff0) | immlo;
                                            inst.operands = [
                                                Operand::Imm32(imm),
                                                Operand::Nothing,
                                                Operand::Nothing,
                                                Operand::Nothing,
                                            ];
                                        }
                                        0b11 => {
                                            // SMC
                                            if self.should_is_must {
                                                if word & 0xf_ff_ff_70 == 0x1_60_00_70 {
                                                    // "SMC must be encoded with AL condition"
                                                    return Err(DecodeError::InvalidOpcode);
                                                }
                                            }
                                            inst.opcode = Opcode::SMC;
                                            let immlo = word & 0x0f;
                                            let imm = ((word >> 4) & 0xfff0) | immlo;
                                            inst.operands = [
                                                Operand::Imm32(imm),
                                                Operand::Nothing,
                                                Operand::Nothing,
                                                Operand::Nothing,
                                            ];
                                        }
                                        _ => { unreachable!("impossible bit pattern"); }
                                    }
                                },
                                _ => {
                                    unreachable!("op2 is a three bit field, got an invalid pattern");
                                }
                            }
                        } else {
                            // |c o n d|0 0 0|1 0 x x|0|x x x x|x x x x|x x x x|1|x x|x|x x x x|
                            // Halfword multiply and multiply accumulate on page A5-200
                            match (word >> 21) & 0b11 {
                                0b00 => {
                                    // N (bit5) selects the Rn halfword, M (bit6) the Rm; true == top.
                                    let n_top = ((word >> 5) & 1) == 1;
                                    let m_top = ((word >> 6) & 1) == 1;
                                    inst.opcode = Opcode::SMLA(n_top, m_top);
                                    inst.operands = [
                                        Operand::Reg(Reg::from_u8((word >> 16) as u8 & 0b1111)),
                                        Operand::Reg(Reg::from_u8(word as u8 & 0b1111)),
                                        Operand::Reg(Reg::from_u8((word >> 8) as u8 & 0b1111)),
                                        Operand::Reg(Reg::from_u8((word >> 12) as u8 & 0b1111)),
                                    ];
                                    return Ok(());
                                },
                                0b01 => {
                                    // op (bit5): 0 => SMLAW, 1 => SMULW. M (bit6) selects the Rm
                                    // halfword; true == top.
                                    let m_top = ((word >> 6) & 1) == 1;
                                    if (word >> 5) & 1 == 0 {
                                        // SMLAWB, SMLAWT page A8-631
                                        inst.opcode = Opcode::SMLAW(m_top);
                                        inst.operands = [
                                            Operand::Reg(Reg::from_u8((word >> 16) as u8 & 0b1111)),
                                            Operand::Reg(Reg::from_u8(word as u8 & 0b1111)),
                                            Operand::Reg(Reg::from_u8((word >> 8) as u8 & 0b1111)),
                                            Operand::Reg(Reg::from_u8((word >> 12) as u8 & 0b1111)),
                                        ];
                                        return Ok(());
                                    } else {
                                        // SMULWB, SMULWT page A8-649
                                        inst.opcode = Opcode::SMULW(m_top);
                                        inst.operands = [
                                            Operand::Reg(Reg::from_u8((word >> 16) as u8 & 0b1111)),
                                            Operand::Reg(Reg::from_u8(word as u8 & 0b1111)),
                                            Operand::Reg(Reg::from_u8((word >> 8) as u8 & 0b1111)),
                                            Operand::Nothing,
                                        ];
                                        return Ok(());
                                    }
                                }
                                0b10 => {
                                    let n_top = ((word >> 5) & 1) == 1;
                                    let m_top = ((word >> 6) & 1) == 1;
                                    inst.opcode = Opcode::SMLAL_halfword(n_top, m_top);
                                    inst.operands = [
                                        Operand::Reg(Reg::from_u8((word >> 12) as u8 & 0b1111)),
                                        Operand::Reg(Reg::from_u8((word >> 16) as u8 & 0b1111)),
                                        Operand::Reg(Reg::from_u8(word as u8 & 0b1111)),
                                        Operand::Reg(Reg::from_u8((word >> 8) as u8 & 0b1111)),
                                    ];
                                    if inst.operands[0] == inst.operands[1] {
                                        // TODO: this is actually "UNPREDICTABLE" (A8-627)
                                        return Err(DecodeError::InvalidOperand);
                                    }
                                    return Ok(());
                                }
                                0b11 => {
                                    let n_top = ((word >> 5) & 1) == 1;
                                    let m_top = ((word >> 6) & 1) == 1;
                                    inst.opcode = Opcode::SMUL(n_top, m_top);
                                    inst.operands = [
                                        Operand::Reg(Reg::from_u8((word >> 16) as u8 & 0b1111)),
                                        Operand::Reg(Reg::from_u8(word as u8 & 0b1111)),
                                        Operand::Reg(Reg::from_u8((word >> 8) as u8 & 0b1111)),
                                        Operand::Nothing,
                                    ];
                                    if inst.operands[0] == inst.operands[1] {
                                        // TODO: this is actually "UNPREDICTABLE" (A8-627)
                                        return Err(DecodeError::InvalidOperand);
                                    }
                                    return Ok(());
                                }
                                _ => {
                                    unreachable!("word masked to two bits");
                                }
                            }
                        }
                    } else {
                        // `Table A5-3 Data-processing (register) instructions`, not op=10xx0.
                        if opcode >= 16 {
                            unreachable!();
                        }

                        inst.opcode = DATA_PROCESSING_OPCODES[opcode as usize];
                        inst.set_s(s);

                        // |c o n d|0 0 0|x x x x|0|x x x x|x x x x|x x x x x|x x|x|x x x x|
                        // interpret the operands as
                        // | Rn | Rd | shift info... |

                        let (Rn, Rd, shift_spec, Rm) = {
                            let Rm = (word & 0x0f) as u8;
                            let shift_spec = (word & 0xfff) as u16;
                            let word = word >> 12;
                            let Rd = (word & 0x0f) as u8;
                            let Rn = ((word >> 4) & 0x0f) as u8;
                            (Rn, Rd, shift_spec, Rm)
                        };

                        let reg_shift = RegShift::from_raw(shift_spec);
                        let last_operand = if reg_shift.is_noop() {
                            // No shift, so the operand is just a register
                            Operand::Reg(Reg::from_u8(Rm))
                        } else {
                            Operand::RegShift(reg_shift)
                        };

                        match inst.opcode {
                            Opcode::MOV
                            |Opcode::MVN => {
                                if self.should_is_must {
                                    if Rn != 0 {
                                        return Err(DecodeError::Nonconforming);
                                    }
                                }
                                inst.operands = [
                                    Operand::Reg(Reg::from_u8(Rd)),
                                    last_operand,
                                    Operand::Nothing,
                                    Operand::Nothing
                                ];
                            }

                            Opcode::CMP
                            |Opcode::CMN
                            |Opcode::TST
                            |Opcode::TEQ => {
                                if self.should_is_must {
                                    if Rd != 0 {
                                        return Err(DecodeError::Nonconforming);
                                    }
                                }
                                inst.operands = [
                                    Operand::Reg(Reg::from_u8(Rn)),
                                    last_operand,
                                    Operand::Nothing,
                                    Operand::Nothing
                                ];
                            }

                            _ => {
                                inst.operands = [
                                    Operand::Reg(Reg::from_u8(Rd)),
                                    Operand::Reg(Reg::from_u8(Rn)),
                                    last_operand,
                                    Operand::Nothing
                                ];
                            }
                        }
                    }
                }
            },
            0b001 => {
                // the instruction looks like
                // |c o n d|0 0 1|x x x x|x|x x x x|x x x x|x x x x x|x x|x|x x x x|
                // bottom part of table A5-2 on page A5-194
                let (s, opcode) = {
                    let part = word >> 20;
                    (
                        (part & 0x01) == 1,
                        ((part >> 1) & 0x0f) as u8
                    )
                };
                if s == false && opcode >= 0b1000 && opcode < 0b1100 {
                // the instruction looks like
                // |c o n d|0 0 1|1 0 x x|0|x x x x|x x x x|x x x x x|x x|x|x x x x|
                // which means 16-bit immediate load, high half immediate load, or MSR (immediate)
                // + hints.
                // See table A5-2, page A5-194.
                    match opcode & 0b0011 {
                        0b00 => {
                            // 16-bit immediate load, MOV (immediate) on page A8-485
                            inst.opcode = Opcode::MOV;
                            inst.operands = [
                                Operand::Reg(Reg::from_u8(((word >> 12) & 0b1111) as u8)),
                                Operand::Imm32(((word >> 4) & 0xf000) | (word & 0xfff)),
                                Operand::Nothing,
                                Operand::Nothing,
                            ];
                        }
                        0b10 => {
                            // High halfword 16-bit immediate load, MOVT on page A8-492
                            inst.opcode = Opcode::MOVT;
                            inst.operands = [
                                Operand::Reg(Reg::from_u8(((word >> 12) & 0b1111) as u8)),
                                Operand::Imm32(((word >> 4) & 0xf000) | (word & 0xfff)),
                                Operand::Nothing,
                                Operand::Nothing,
                            ];
                        }
                        0b01 => {
                            // MSR (immediate), and hints on page A5-204, op==0
                            let op1 = (word >> 16) & 0b1111;
                            match op1 {
                                0b0000 => {

                                },
                                _ => {
                                    // Move to Special register, Application level MSR (immediate) on
                                    // page A8-499
                                    if self.should_is_must {
                                        if (word >> 12) & 0b1111 != 0b1111 {
                                            return Err(DecodeError::Nonconforming);
                                        }
                                    }
                                    inst.operands = [
                                        Operand::StatusRegMask(StatusRegMask::from_raw(op1 as u8)?),
                                        // MSR (immediate) uses a modified immediate.
                                        Operand::Imm32((word & 0xff).rotate_right(2 * ((word >> 8) & 0xf))),
                                        Operand::Nothing,
                                        Operand::Nothing,
                                    ];
                                    inst.opcode = Opcode::MSR;
                                },
                            }
                        }
                        0b11 => {
                            // MSR (immediate), and hints on page A5-204, op==1
                            if self.should_is_must {
                                if (word >> 12) & 0b1111 != 0b1111 {
                                    return Err(DecodeError::Nonconforming);
                                }
                            }
                            inst.operands = [
                                Operand::StatusRegMask(StatusRegMask::from_raw((word >> 16) as u8 & 0b1111 | 0b10000)?),
                                // MSR (immediate) uses a modified immediate (imm8 ror 2*rot), not
                                // the raw 12-bit field.
                                Operand::Imm32((word & 0xff).rotate_right(2 * ((word >> 8) & 0xf))),
                                Operand::Nothing,
                                Operand::Nothing,
                            ];
                            inst.opcode = Opcode::MSR;
                        }
                        _ => { unreachable!("impossible bit pattern"); }
                    }
                } else {
                    // Data-processing (immediate)
                    // Page A5-197
                    if opcode >= 16 {
                        unreachable!();
                    }
                    inst.opcode = DATA_PROCESSING_OPCODES[opcode as usize];
                    inst.set_s(s);

                    let (Rn, Rd, imm) = {
                        let rot = word & 0x00000f00;
                        let imm = word & 0x000000ff;
                        let imm = (imm as u32).rotate_right(2 * (rot >> 8));
                        ((word >> 16) as u8 & 0x0f, (word >> 12) as u8 & 0x0f, imm)
                    };
                    if (opcode == 0b0010 || opcode == 0b0100) && Rn == 0b1111 {
                        inst.opcode = Opcode::ADR;
                    }
                    match opcode {
                        // MOV (0b1101) and MVN (0b1111) immediate: no Rn operand.
                        0b1101 | 0b1111 => {
                            inst.operands = [
                                Operand::Reg(Reg::from_u8(Rd)),
                                Operand::Imm32(imm),
                                Operand::Nothing,
                                Operand::Nothing,
                            ];
                        }
                        // CMP (immediate)
                        0b1010
                        // CMN (immediate)
                        |0b1011
                        // TEQ (immediate)
                        |0b1001
                        // TST (immediate)
                        |0b1000 => {
                            // According to A8-368, there are 4 bits right above the immediate that
                            // are reserved and should be zero
                            if self.should_is_must && (word >> 12) as u8 & 0b1111 != 0 {
                                return Err(DecodeError::Nonconforming);
                            }

                            // Unlike most of the other instructions in this category, there are
                            // only two operands to an immediate CMP.
                            inst.operands = [
                                Operand::Reg(Reg::from_u8(Rn)),
                                Operand::Imm32(imm),
                                Operand::Nothing,
                                Operand::Nothing,
                            ];
                        }
                        _ => {
                            inst.operands = [
                                Operand::Reg(Reg::from_u8(Rd)),
                                Operand::Reg(Reg::from_u8(Rn)),
                                Operand::Imm32(imm),
                                Operand::Nothing,
                            ];
                        }
                    }

                }
                /* ... */
            }
            0b010 => {
                // Load/store word and unsigned byte
                // A5.3
                // Page A5-206
                let Rn = ((word >> 16) & 0x0f) as u8;
                let op = ((word >> 20) & 0x1f) as u8;
                let add = (op & 0b01000) != 0;
                let (imm, Rt) = {
                    ((word & 0x0fff) as u16, ((word >> 12) & 0x0f) as u8)
                };
                if (op & 0b10010) == 0b00010 {
                    let op = op & 0b00111;
                // |c o n d|0 1 0|0 x x 1 x|x x x x x x x x x x x x x x x|x|x x x x|
                    /*
                    0x010 -> STRT
                    0x011 -> LDRT
                    0x110 -> STRBT
                    0x111 -> LDRBT
                    */
                    inst.opcode = match op {
                        0b010 => Opcode::STRT,
                        0b011 => Opcode::LDRT,
                        0b110 => Opcode::STRBT,
                        0b111 => Opcode::LDRBT,
                        _ => { return Err(DecodeError::InvalidOpcode); }
                    };
                    inst.operands = [
                        Operand::Reg(Reg::from_u8(Rt)),
                        Operand::RegDerefPostindexOffset(Reg::from_u8(Rn), imm as u16, add, false),
                        Operand::Nothing,
                        Operand::Nothing,
                    ];
                } else {
                    /*
                    xx0x0 not 0x010 -> STR (imm)
                    xx0x1 not 0x011 -> LDR (imm)
                    xx1x0 not 0x110 -> STRB (imm)
                    xx1x1 not 0x111 -> LDRB (imm)
                    */
                    let P = (op & 0b10000) != 0;
                    let W = (op & 0b00010) != 0;
                    let op = op & 0b00101;
                    inst.opcode = if !P && W {
                        // Table A5-15
                        // strt, ldrt, strbt, ldrbt
                        match op {
                            0b000 => Opcode::STRT,
                            0b001 => Opcode::LDRT,
                            0b100 => Opcode::STRBT,
                            0b101 => Opcode::LDRBT,
                            _ => { unreachable!("bad bit pattern, table A5-15"); }
                        }
                    } else {
                        match op {
                            0b000 => Opcode::STR,
                            0b001 => {
                                if Rn == 0b1111 {
                                    inst.operands = [
                                        Operand::Reg(Reg::from_u8(Rt)),
                                        if P {
                                            Operand::RegDerefPreindexOffset(Reg::from_u8(Rn), imm as u16, add, W)
                                        } else {
                                            Operand::RegDerefPostindexOffset(Reg::from_u8(Rn), imm as u16, add, W)
                                        },
                                        Operand::Nothing,
                                        Operand::Nothing,
                                    ];
                                    inst.opcode = Opcode::LDR;
                                    return Ok(());
                                }
                                Opcode::LDR
                            },
                            0b100 => Opcode::STRB,
                            0b101 => {
                                if Rn == 0b1111 {
                                    inst.operands = [
                                        Operand::Reg(Reg::from_u8(Rt)),
                                        if P {
                                            Operand::RegDerefPreindexOffset(Reg::from_u8(Rn), imm as u16, add, W)
                                        } else {
                                            Operand::RegDerefPostindexOffset(Reg::from_u8(Rn), imm as u16, add, W)
                                        },
                                        Operand::Nothing,
                                        Operand::Nothing,
                                    ];
                                    inst.opcode = Opcode::LDRB;
                                    return Ok(());
                                }
                                Opcode::LDRB
                            },
                            _ => { return Err(DecodeError::InvalidOpcode); }
                        }
                    };
                    inst.operands = [
                        Operand::Reg(Reg::from_u8(Rt)),
                        if P {
                            Operand::RegDerefPreindexOffset(Reg::from_u8(Rn), imm as u16, add, W)
                        } else {
                            Operand::RegDerefPostindexOffset(Reg::from_u8(Rn), imm as u16, add, W)
                        },
                        Operand::Nothing,
                        Operand::Nothing,
                    ];
                }
            },
            0b011 => {
                // page A5-192 to distinguish the following:
                // check for media instructions, and if not, load/store word and unsigned byte
                if (word & 0x00000010) != 0 {
                // |c o n d|0 1 1|x x x x|x|x x x x|x x x x|x x x x x|x x|1|x x x x|
                    // using language from A5-206: A == 1 and B == 1
                    // so this is media instructions (A5-207)
                    return Err(DecodeError::Incomplete);
                } else {
                // |c o n d|0 1 1|x x x x|x|x x x x|x x x x|x x x x x|x x|0|x x x x|
                    // instructions here are A == 1, B == 0 in A5-206
                    let op = ((word >> 20) & 0x1f) as u8;

                    let P = (op & 0b10000) != 0;
                    let U = (op & 0b01000) != 0;
                    let W = (op & 0b00010) != 0;
                    /*
                        xx0x0 not 0x010 -> STR (register)
                        0x010 -> STRT
                        xx0x1 not 0x011 -> LDR (register)
                        0x011 -> LDRT
                        xx1x0 not 0x110 -> STRB (register)
                        0x110 -> STRBT
                        xx1x1 not 0x111 -> LDRB (register)
                        0x111 -> LDRBT
                    */
                    let _Rn = ((word >> 16) & 0x0f) as u8;
                    if (op & 0b10010) == 0b00010 {
                        let op = op & 0b00111;
                // |c o n d|0 1 1|0 x x 1 x|x x x x x x x x x x x x x x x|0|x x x x|
                        /*
                        0x010 -> STRT
                        0x011 -> LDRT
                        0x110 -> STRBT
                        0x111 -> LDRBT
                        */
                        inst.opcode = match op {
                            0b010 => Opcode::STRT,
                            0b011 => Opcode::LDRT,
                            0b110 => Opcode::STRBT,
                            0b111 => Opcode::LDRBT,
                            _ => { return Err(DecodeError::InvalidOpcode); }
                        };
                    } else {
                        /*
                        xx0x0 not 0x010 -> STR (imm)
                        xx0x1 not 0x011 -> LDR (imm)
                        xx1x0 not 0x110 -> STRB (imm)
                        xx1x1 not 0x111 -> LDRB (imm)
                        */
                        let op = op & 0b00101;
                        inst.opcode = match op {
                            0b000 => Opcode::STR,
                            0b001 => Opcode::LDR,
                            0b100 => Opcode::STRB,
                            0b101 => Opcode::LDRB,
                            _ => { return Err(DecodeError::InvalidOpcode); }
                        };
                    }
                    let (Rt, shift) = {
                        let shift = (word & 0xfff) as u16;
                        let word = word >> 12;
                        let Rt = (word & 0xf) as u8;
                        (Rt, shift)
                    };
                    let Rn = (word >> 16) as u8 & 0b1111;
                    inst.operands = [
                        Operand::Reg(Reg::from_u8(Rt)),
                        if P {
                            Operand::RegDerefPreindexRegShift(Reg::from_u8(Rn), RegShift::from_raw(shift), U, W)
                        } else {
                            Operand::RegDerefPostindexRegShift(Reg::from_u8(Rn), RegShift::from_raw(shift), U, false)
                        },
                        Operand::Nothing,
                        Operand::Nothing,
                    ];
                }
                return Ok(());
            },
            0b100 | 0b101 => {
                // branch, branch with link, and block data transfer
                // page A5-212
                let op = (word >> 20) & 0x3f;
                if op < 0b100000 {
                    let wback = (op & 0b000010) != 0;
                    let add = (op & 0b001000) != 0;
                    let pre = (op & 0b010000) != 0;
                    let usermode = (op & 0b000100) != 0;
                    inst.opcode = if (op & 1) == 0 {
                        Opcode::STM(add, pre, false, usermode)
                    } else {
                        Opcode::LDM(add, pre, false, usermode)
                    };
                    inst.operands = [
                        Operand::RegWBack(Reg::from_u8(((word >> 16) & 0xf) as u8), wback),
                        Operand::RegList((word & 0xffff) as u16),
                        Operand::Nothing,
                        Operand::Nothing,
                    ];
                } else if op < 0b110000 {
                    // 10xxxx
                    inst.opcode = Opcode::B;

                    // sign-extend the 24-bit field first, then add 2 to compensate for the
                    // architecturally-defined initial offset (adding before sign extension
                    // overflows the field for the largest positive displacements).
                    let imm24 = (((word & 0x00ff_ffff) << 8) as i32 >> 8) + 2;

                    inst.operands = [
                        Operand::BranchOffset(imm24),
                        Operand::Nothing,
                        Operand::Nothing,
                        Operand::Nothing,
                    ];
                } else {
                    // 11xxxx

                    // sign-extend the 24-bit field first, then add 2 to compensate for the
                    // architecturally-defined initial offset (adding before sign extension
                    // overflows the field for the largest positive displacements).
                    let imm24 = (((word & 0x00ff_ffff) << 8) as i32 >> 8) + 2;

                    inst.opcode = Opcode::BL;
                    inst.operands = [
                        Operand::BranchOffset(imm24),
                        Operand::Nothing,
                        Operand::Nothing,
                        Operand::Nothing,
                    ];
                }
            },
            0b110 => {
                // coprocessor instructions and supervisor call
                // page A5-213
                // low bit of 0b110 or 0b111 corresponds to high bit of op1
                //
                // op1=0b110xxxxx, see table A5-23
                let coproc = (word >> 8) as u8 & 0b1111;

                if coproc & 0b1110 == 0b1010 {
                    // Advanced SIMD, Floating-point (coproc 101x) -> VFP decode.
                    let (opcode, operands) = decode_vfp(word)?;
                    inst.opcode = opcode;
                    inst.operands = operands;
                    return Ok(());
                }

                if (word >> 20) & 0b11010 == 0b00000 {
                    // the `not 11000x0{0,1}` cases in table A5-23, MCRR or MRRC
                    // but first check that bit 2 of op1 is in fact 1:
                    if (word >> 20) & 0b00100 != 0 {
                        // actually MCRR or MRRC
                        let CRm = word as u8 & 0b1111;
                        let opc1 = (word >> 4) as u8 & 0b1111;
                        let Rt = (word >> 12) as u8 & 0b1111;
                        let Rt2 = (word >> 16) as u8 & 0b1111;
                        if Rt == 15 || Rt2 == 15 || Rt == Rt2 {
                            // TODO: actually `UNPREDICTABLE`
                            return Err(DecodeError::InvalidOperand);
                        }
                        if (word >> 20) & 0b00001 != 0 {
                            inst.opcode = Opcode::MRRC(coproc, opc1, false);
                        } else {
                            inst.opcode = Opcode::MCRR(coproc, opc1, false);
                        }
                        inst.operands = [
                            Operand::Reg(Reg::from_u8(Rt)),
                            Operand::Reg(Reg::from_u8(Rt2)),
                            Operand::CReg(CReg::from_u8(CRm)),
                            Operand::Nothing,
                        ];
                    } else {
                        return Err(DecodeError::InvalidOpcode);
                    }
                } else {
                    // STC or LDC
                    let pudw = (word >> 21) as u8 & 0b1111;
                    let Rn = (word >> 16) as u8 & 0b1111;
                    let CRd = (word >> 12) as u8 & 0b1111;
                    let imm8 = word & 0b11111111;

                    if (word >> 20) & 0b00001 == 0 {
                        // op=110xxxx0, STC
                        // page A8-663

                        if pudw & 0b0010 != 0 {
                            inst.opcode = Opcode::STCL(coproc, false);
                        } else {
                            inst.opcode = Opcode::STC(coproc, false);
                        }
                    } else {
                        // op=110xxxx1, LDC
                        // page A8-393

                        if pudw & 0b0010 != 0 {
                            inst.opcode = Opcode::LDCL(coproc, false);
                        } else {
                            inst.opcode = Opcode::LDC(coproc, false);
                        }
                    }

                    let P = pudw & 0b1000 != 0;
                    let U = pudw & 0b0100 != 0;
                    let W = pudw & 0b0001 != 0;

                    inst.operands = [
                        Operand::CReg(CReg::from_u8(CRd)),
                        if P {
                            Operand::RegDerefPreindexOffset(Reg::from_u8(Rn), (imm8 << 2) as u16, U, W)
                        } else {
                            if W {
                                // postindex always has wback
                                Operand::RegDerefPostindexOffset(Reg::from_u8(Rn), (imm8 << 2) as u16, U, true)
                            } else {
                                Operand::RegDeref(Reg::from_u8(Rn))
                            }
                        },
                        if !P && !W {
                            // TODO: not sure what ldc2{l}'s <option> field really means?
                            // at this point the invalid encoding and mrrc/mcrr forms have
                            // been tested, so..
                            debug_assert!(U);
                            Operand::CoprocOption(imm8 as u8)
                        } else {
                            Operand::Nothing
                        },
                        Operand::Nothing,
                    ];
                }
            },
            0b111 => {
                // coprocessor instructions and supervisor call
                // page A5-213
                // low bit of 0b110 or 0b111 corresponds to high bit of op1
                //
                // op1=0b111xxxxx, see table A5-22
                let op1 = (word >> 20) & 0b111111;
                if ((op1 >> 4) & 1) != 0 {
                    inst.opcode = Opcode::SVC;
                    let imm = word & 0xff_ff_ff;
                    inst.operands = [
                        Operand::Imm32(imm),
                        Operand::Nothing,
                        Operand::Nothing,
                        Operand::Nothing,
                    ];
                    return Ok(());
                }

                let coproc = (word >> 8) as u8 & 0b1111;

                if coproc & 0b1110 == 0b1010 {
                    // Advanced SIMD, Floating-point (coproc 101x) -> VFP decode.
                    let (opcode, operands) = decode_vfp(word)?;
                    inst.opcode = opcode;
                    inst.operands = operands;
                    return Ok(());
                }

                if (word >> 4) & 1 == 0 {
                    // CDP, page A8-356
                    let CRm = word as u8 & 0b1111;
                    let opc2 = (word >> 5) as u8 & 0b111;
                    let Rt = (word >> 12) as u8 & 0b1111;
                    let CRn = (word >> 16) as u8 & 0b1111;

                    let opc1 = (word >> 20) as u8 & 0b1111;
                    inst.opcode = Opcode::CDP(coproc, opc1, opc2, false);
                    inst.operands = [
                        Operand::CReg(CReg::from_u8(Rt)),
                        Operand::CReg(CReg::from_u8(CRn)),
                        Operand::CReg(CReg::from_u8(CRm)),
                        Operand::Nothing,
                    ];
                } else {
                    // MCR/MRC, page A8-493
                    let CRm = word as u8 & 0b1111;
                    let opc2 = (word >> 5) as u8 & 0b111;
                    let Rt = (word >> 12) as u8 & 0b1111;
                    let CRn = (word >> 16) as u8 & 0b1111;

                    let opc1 = (word >> 21) as u8 & 0b111;
                    if (word >> 20) & 1 == 0 {
                        inst.opcode = Opcode::MCR(coproc, opc1, opc2, false);
                    } else {
                        inst.opcode = Opcode::MRC(coproc, opc1, opc2, false);
                    }
                    inst.operands = [
                        Operand::Reg(Reg::from_u8(Rt)),
                        Operand::CReg(CReg::from_u8(CRn)),
                        Operand::CReg(CReg::from_u8(CRm)),
                        Operand::Nothing,
                    ];
                }
            }
            _ => { unreachable!("opc category is three bits"); }
        }
        Ok(())
    }
}

/// a struct with a summary of the `ARMv7` instruction set in an associated `impl Arch for ARMv7`.
#[cfg_attr(feature="use-serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub struct ARMv7;

impl Arch for ARMv7 {
    type Address = u32;
    type Word = u8;
    type Instruction = Instruction;
    type DecodeError = DecodeError;
    type Decoder = InstDecoder;
    type Operand = Operand;
}


/*
 * tests: (armv7?)
 * [0x00, 0x00, 0x90, 0xe0]
 * adds r0, r0, r0
 *
 * [0x00, 0x00, 0x82, 0xe0]
 * add r0, r2, r0
 *
 * [0x00, 0x00, 0x88, 0xe0]
 * add r0, r8, r0
 *
 * [0x00, 0x01, 0x80, 0xe0]
 * add r0, r0, r0, lsl 2
 *
 * [0x00, 0x80, 0x00, 0x00]
 * andeq r8, r0, r0
 *
 * [0xc0, 0x80, 0x20, 0x00]
 * eoreq r8, r0, r0, asr 1
 *
 * [0x00, 0x00, 0xa2, 0xe1]
 * mov r0, r0
 *
 * [0x00, 0x00, 0xaf, 0xe1]
 * mov r0, r0
 *
 * [0x10, 0x00, 0xaf, 0xe1]
 * invalid
 *
 * [0x01, 0x00, 0xaf, 0xe1]
 * mov r0, r1
 *
 * [0x00, 0x01, 0xaf, 0xe1]
 * invalid
 *
 * [0x00, 0x00, 0xa0, 0xe1]
 * mov r0, r0
 *
 * [0x00, 0x01, 0xa0, 0xe1]
 * lsl r0, r0, 2
 * # is this equivalent to mov r0, r0<<2?
 * 0180afe1 invalid
 * 018076e1 cmn r6, r1
 * 015096e1 orrs r5, r6, r1
 * 018086e1 orr r8, r6, r1
 * 0180a6e1 invalid
 * 0180d6e1 bics r8, r6, r1
 * 0180d0e1 bics r8, r0, r1
 * 8110d0e1 bics r1, r0, r1, lsl 1
 * 1200dfe1 bics r0, pc, r2, lsl r0
 * f110d0e1 ldrsh r1, [r0, 1]
 * 0101a0e1 lsl r0, r1, 2
 * 0110a0e1 mov r1, r1
 * 0111a0e1 lsl r1, r1, 2
 * 4110a0e1 asr r1, r1, 0x20
 * 2110a0e1 lsr r1, r1, 0x20
 *
 */
