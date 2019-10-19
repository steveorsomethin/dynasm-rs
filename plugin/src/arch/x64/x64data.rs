use std::collections::{hash_map, HashMap};

use super::compiler::Opdata;
use std::fmt::{self, Display};

use bitflags::bitflags;
use lazy_static::lazy_static;

macro_rules! constify {
    ($t:ty, $e:expr) => {{
        const C: &$t = &$e;
        C
    }};
}

macro_rules! OpInner {
    ($fmt:expr, $ops:expr, $reg:expr) => {
        Opdata {
            args: $fmt,
            ops: constify!([u8], $ops),
            reg: $reg,
            flags: Flags::DEFAULT,
            features: Features::X64_IMPLICIT,
        }
    };
    ($fmt:expr, $ops:expr, $reg:expr, $f:expr) => {
        Opdata {
            args: $fmt,
            ops: constify!([u8], $ops),
            reg: $reg,
            flags: Flags::make($f),
            features: Features::X64_IMPLICIT,
        }
    };
    ($fmt:expr, $ops:expr, $reg:expr, $f:expr, $ft:expr) => {
        Opdata {
            args: $fmt,
            ops: constify!([u8], $ops),
            reg: $reg,
            flags: Flags::make($f),
            features: Features::make($ft),
        }
    };
}

macro_rules! Ops {
    ( $( $name:tt = [ $( $( $e:expr ),+ ; )+ ] )* ) => {
        [ $(
            (
                $name,
                {
                    const OPDATA: &[Opdata] = &[$( OpInner!($( $e ),*) ,)+];
                    OPDATA
                }
            )
        ),* ]
    };
}

pub fn get_mnemnonic_data(name: &str) -> Option<&'static [Opdata]> {
    OPMAP.get(&name).cloned()
}

bitflags! {
    pub struct Flags: u32 {
        const DEFAULT   = 0x0000_0000; // this instruction has default encoding
        const VEX_OP    = 0x0000_0001; // this instruction requires a VEX prefix to be encoded
        const XOP_OP    = 0x0000_0002; // this instruction requires a XOP prefix to be encoded
        const IMM_OP    = 0x0000_0004; // this instruction encodes the final opcode byte in the immediate position, like 3DNow! ops.

        // note: the first 4 in this block are mutually exclusive
        const AUTO_SIZE = 0x0000_0008; // 16 bit -> OPSIZE , 32-bit -> None     , 64-bit -> REX.W/VEX.W/XOP.W
        const AUTO_NO32 = 0x0000_0010; // 16 bit -> OPSIZE , 32-bit -> None(x86), 64-bit -> None(x64)
        const AUTO_REXW = 0x0000_0020; // 16 bit -> illegal, 32-bit -> None     , 64-bit -> REX.W/VEX.W/XOP.W
        const AUTO_VEXL = 0x0000_0040; // 128bit -> None   , 256bit -> VEX.L
        const WORD_SIZE = 0x0000_0080; // implies opsize prefix
        const WITH_REXW = 0x0000_0100; // implies REX.W/VEX.W/XOP.W
        const WITH_VEXL = 0x0000_0200; // implies VEX.L/XOP.L
        const EXACT_SIZE= 0x0000_0400; // operands with unknown sizes cannot be assumed to match

        const PREF_66   = WORD_SIZE;   // mandatory prefix (same as WORD_SIZE)
        const PREF_67   = 0x0000_0800; // mandatory prefix (same as SMALL_ADDRESS)
        const PREF_F0   = 0x0000_1000; // mandatory prefix (same as LOCK)
        const PREF_F2   = 0x0000_2000; // mandatory prefix (REPNE)
        const PREF_F3   = 0x0000_4000; // mandatory prefix (REP)

        const LOCK      = 0x0000_8000; // user lock prefix is valid with this instruction
        const REP       = 0x0001_0000; // user rep prefix is valid with this instruction
        const REPE      = 0x0002_0000;

        const SHORT_ARG = 0x0004_0000; // a register argument is encoded in the last byte of the opcode
        const ENC_MR    = 0x0008_0000; // select alternate arg encoding
        const ENC_VM    = 0x0010_0000; // select alternate arg encoding
        const ENC_MIB   = 0x0020_0000; // A special encoding using the SIB to specify an immediate and two registers
        const X86_ONLY  = 0x0040_0000; // instructions available in protected mode, but not long mode
    }
}

bitflags! {
    pub struct OperandType: u8 {
        const Immediate = 0b01101001u8;
        const InstructionOffset = 0b01101111u8;
        const Memory = 0b01101101u8;
        const VSIB32 = 0b01101011u8; // vsib addressing, 32 bit result, size determines xmm or ymm
        const VSIB64 = 0b01101100u8; // 64 bit result, size determines xmm or ymm
        const LegacyMemory = 0b01110110u8;
        const MMXMemory = 0b01110101u8;
        const AVXMemory = 0b01110111u8;
    }
}
bitflags! {
    pub struct RegisterType: u8 {
        const Legacy = 0b01110010u8;
        const FP = 0b01100110u8;
        const MMX = 0b01111000u8;
        const AVX = 0b01111001u8;
        const Segment = 0b01110011u8;
        const Control = 0b01100011u8;
        const Debug = 0b01100100u8;
        const Bound = 0b01100010u8;
    }
}
bitflags! {
    pub struct RegisterX64: u8 {
        const rax = 0b01000001u8;
        const rcx = 0b01000010u8;
        const rdx = 0b01000011u8;
        const rbx = 0b01000100u8;
        const rsi = 0b01000101u8;
        const rdi = 0b01000110u8;
        const rsp = 0b01000111u8;
        const rbp = 0b01001000u8;
        const r8 = 0b01001001u8;
        const r9 = 0b01001010u8;
        const r10 = 0b01001011u8;
        const r11 = 0b01001100u8;
        const r12 = 0b01001101u8;
        const r13 = 0b01001110u8;
        const r14 = 0b01001111u8;
        const r15 = 0b01010000u8;
        const es = 0b01010001u8;
        const cs = 0b01010010u8;
        const ss = 0b01010011u8;
        const ds = 0b01010100u8;
        const fs = 0b01010101u8;
        const gs = 0b01010110u8;
    }
}
bitflags! {
    pub struct ControlRegister: u8 {
        const CR8 = 0b01010111u8;
        const st0 = 0b01011000u8;
    }
}
bitflags! {
    pub struct Size: u8 {
        const Byte = 0b01100010u8; // 8
        const Word = 0b01110111u8; // 16
        const DWord = 0b01100100u8; // 2 x 16
        const QWord = 0b01110001u8; // 4 x 16
        const OWord = 0b01101111u8; // 8 x 16
        const HWord = 0b01101000u8; // Hexadecimal
        const PWord = 0b01110000u8; // 10 bytes
        const FWord = 0b01100110u8; // 6 bytes
        const Auto = 0b00101010u8; // matches all possible sizes for this operand (w/d for i, w/d/q for r/v, o/h for y/w and everything for m)
        const MemAuto = 0b00100001u8; // matches a lack of size, only useful in combination with m
        const CalcAuto = 0b00111111u8; // matches any size and doesn't participate in the operand size calculation0
    }
}

pub const Immediate: u8 = OperandType::Immediate.bits;
pub const InstructionOffset: u8 = OperandType::InstructionOffset.bits;
pub const Memory: u8 = OperandType::Memory.bits;
pub const VSIB32: u8 = OperandType::VSIB32.bits; // vsib addressing, 32 bit result, size determines xmm or ymm
pub const VSIB64: u8 = OperandType::VSIB64.bits; // 64 bit result, size determines xmm or ymm
pub const LegacyMemory: u8 = OperandType::LegacyMemory.bits;
pub const MMXMemory: u8 = OperandType::MMXMemory.bits;
pub const AVXMemory: u8 = OperandType::AVXMemory.bits;

pub const Legacy: u8 = RegisterType::Legacy.bits;
pub const FP: u8 = RegisterType::FP.bits;
pub const MMXRegister: u8 = RegisterType::MMX.bits;
pub const AVXRegister: u8 = RegisterType::AVX.bits;
pub const Segment: u8 = RegisterType::Segment.bits;
pub const Control: u8 = RegisterType::Control.bits;
pub const Debug: u8 = RegisterType::Debug.bits;
pub const Bound: u8 = RegisterType::Bound.bits;

pub const rax: u8 = RegisterX64::rax.bits;
pub const rcx: u8 = RegisterX64::rcx.bits;
pub const rdx: u8 = RegisterX64::rdx.bits;
pub const rbx: u8 = RegisterX64::rbx.bits;
pub const rsi: u8 = RegisterX64::rsi.bits;
pub const rdi: u8 = RegisterX64::rdi.bits;
pub const rsp: u8 = RegisterX64::rsp.bits;
pub const rbp: u8 = RegisterX64::rbp.bits;
pub const r8: u8 = RegisterX64::r8.bits;
pub const r9: u8 = RegisterX64::r9.bits;
pub const r10: u8 = RegisterX64::r10.bits;
pub const r11: u8 = RegisterX64::r11.bits;
pub const r12: u8 = RegisterX64::r12.bits;
pub const r13: u8 = RegisterX64::r13.bits;
pub const r14: u8 = RegisterX64::r14.bits;
pub const r15: u8 = RegisterX64::r15.bits;
pub const es: u8 = RegisterX64::es.bits;
pub const cs: u8 = RegisterX64::cs.bits;
pub const ss: u8 = RegisterX64::ss.bits;
pub const ds: u8 = RegisterX64::ds.bits;
pub const fs: u8 = RegisterX64::fs.bits;
pub const gs: u8 = RegisterX64::gs.bits;

pub const CR8: u8 = ControlRegister::CR8.bits;
pub const st0: u8 = ControlRegister::st0.bits;

pub const Byte: u8 = Size::Byte.bits; // 8
pub const Word: u8 = Size::Word.bits; // 16
pub const DWord: u8 = Size::DWord.bits; // 2 x 16
pub const QWord: u8 = Size::QWord.bits; // 4 x 16
pub const OWord: u8 = Size::OWord.bits; // 8 x 16
pub const HWord: u8 = Size::HWord.bits; // Hexadecimal
pub const PWord: u8 = Size::PWord.bits; // 10 bytes
pub const FWord: u8 = Size::FWord.bits; // 6 bytes
pub const Auto: u8 = Size::Auto.bits; // matches all possible sizes for this operand (w/d for i, w/d/q for r/v, o/h for y/w and everything for m)
pub const MemAuto: u8 = Size::MemAuto.bits; // matches a lack of size, only useful in combination with m
pub const CalcAuto: u8 = Size::CalcAuto.bits; // matches any size and doesn't participate in the operand size calculation0

impl Flags {
    const fn make(bits: u32) -> Flags {
        Flags { bits }
    }
}

bitflags! {
    pub struct Features: u32 {
        const X64_IMPLICIT = 0x0000_0000;
        const FPU          = 0x0000_0001;
          const MMX          = 0x0000_0002;
        const TDNOW        = 0x0000_0004;
        const SSE          = 0x0000_0008;
        const SSE2         = 0x0000_0010;
        const SSE3         = 0x0000_0020;
        const VMX          = 0x0000_0040;
        const SSSE3        = 0x0000_0080;
        const SSE4A        = 0x0000_0100;
        const SSE41        = 0x0000_0200;
        const SSE42        = 0x0000_0400;
        const SSE5         = 0x0000_0800;
        const AVX          = 0x0000_1000;
        const AVX2         = 0x0000_2000;
        const FMA          = 0x0000_4000;
        const BMI1         = 0x0000_8000;
        const BMI2         = 0x0001_0000;
        const TBM          = 0x0002_0000;
        const RTM          = 0x0004_0000;
        const INVPCID      = 0x0008_0000;
        const MPX          = 0x0010_0000;
        const SHA          = 0x0020_0000;
        const PREFETCHWT1  = 0x0040_0000;
        const CYRIX        = 0x0080_0000;
        const AMD          = 0x0100_0000;
    }
}

impl Features {
    const fn make(bits: u32) -> Features {
        Features { bits }
    }

    pub fn from_str(name: &str) -> Option<Features> {
        match name {
            "fpu" => Some(Features::FPU),
            "mmx" => Some(Features::MMX),
            "tdnow" => Some(Features::TDNOW),
            "sse" => Some(Features::SSE),
            "sse2" => Some(Features::SSE2),
            "sse3" => Some(Features::SSE3),
            "vmx" => Some(Features::VMX),
            "ssse3" => Some(Features::SSSE3),
            "sse4a" => Some(Features::SSE4A),
            "sse41" => Some(Features::SSE41),
            "sse42" => Some(Features::SSE42),
            "sse5" => Some(Features::SSE5),
            "avx" => Some(Features::AVX),
            "avx2" => Some(Features::AVX2),
            "fma" => Some(Features::FMA),
            "bmi1" => Some(Features::BMI1),
            "bmi2" => Some(Features::BMI2),
            "tbm" => Some(Features::TBM),
            "rtm" => Some(Features::RTM),
            "invpcid" => Some(Features::INVPCID),
            "mpx" => Some(Features::MPX),
            "sha" => Some(Features::SHA),
            "prefetchwt1" => Some(Features::PREFETCHWT1),
            "cyrix" => Some(Features::CYRIX),
            "amd" => Some(Features::AMD),
            _ => None,
        }
    }
}

impl Display for Features {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut keys = Vec::new();
        if self.contains(Features::FPU) {
            keys.push("fpu");
        }
        if self.contains(Features::MMX) {
            keys.push("mmx");
        }
        if self.contains(Features::TDNOW) {
            keys.push("tdnow");
        }
        if self.contains(Features::SSE) {
            keys.push("sse");
        }
        if self.contains(Features::SSE2) {
            keys.push("sse2");
        }
        if self.contains(Features::SSE3) {
            keys.push("sse3");
        }
        if self.contains(Features::VMX) {
            keys.push("vmx");
        }
        if self.contains(Features::SSSE3) {
            keys.push("ssse3");
        }
        if self.contains(Features::SSE4A) {
            keys.push("sse4a");
        }
        if self.contains(Features::SSE41) {
            keys.push("sse41");
        }
        if self.contains(Features::SSE42) {
            keys.push("sse42");
        }
        if self.contains(Features::SSE5) {
            keys.push("sse5");
        }
        if self.contains(Features::AVX) {
            keys.push("avx");
        }
        if self.contains(Features::AVX2) {
            keys.push("avx2");
        }
        if self.contains(Features::FMA) {
            keys.push("fma");
        }
        if self.contains(Features::BMI1) {
            keys.push("bmi1");
        }
        if self.contains(Features::BMI2) {
            keys.push("bmi2");
        }
        if self.contains(Features::TBM) {
            keys.push("tbm");
        }
        if self.contains(Features::RTM) {
            keys.push("rtm");
        }
        if self.contains(Features::INVPCID) {
            keys.push("invpcid");
        }
        if self.contains(Features::MPX) {
            keys.push("mpx");
        }
        if self.contains(Features::SHA) {
            keys.push("sha");
        }
        if self.contains(Features::PREFETCHWT1) {
            keys.push("prefetchwt1");
        }
        if self.contains(Features::CYRIX) {
            keys.push("cyrix");
        }
        if self.contains(Features::AMD) {
            keys.push("amd");
        }
        for (i, k) in keys.into_iter().enumerate() {
            if i != 0 {
                f.write_str(", ")?;
            }
            f.write_str(k)?;
        }
        Ok(())
    }
}

#[allow(dead_code)]
pub fn mnemnonics() -> hash_map::Keys<'static, &'static str, &'static [Opdata]> {
    OPMAP.keys()
}

// workaround until bitflags can be used in const
const DEFAULT: u32 = Flags::DEFAULT.bits;
const VEX_OP: u32 = Flags::VEX_OP.bits;
const XOP_OP: u32 = Flags::XOP_OP.bits;
const IMM_OP: u32 = Flags::IMM_OP.bits;
const SHORT_ARG: u32 = Flags::SHORT_ARG.bits;
const AUTO_SIZE: u32 = Flags::AUTO_SIZE.bits;
const AUTO_NO32: u32 = Flags::AUTO_NO32.bits;
const AUTO_REXW: u32 = Flags::AUTO_REXW.bits;
const AUTO_VEXL: u32 = Flags::AUTO_VEXL.bits;
const WORD_SIZE: u32 = Flags::WORD_SIZE.bits;
const WITH_REXW: u32 = Flags::WITH_REXW.bits;
const WITH_VEXL: u32 = Flags::WITH_VEXL.bits;
const EXACT_SIZE: u32 = Flags::EXACT_SIZE.bits;
const PREF_66: u32 = Flags::PREF_66.bits;
const PREF_67: u32 = Flags::PREF_67.bits;
const PREF_F0: u32 = Flags::PREF_F0.bits;
const PREF_F2: u32 = Flags::PREF_F2.bits;
const PREF_F3: u32 = Flags::PREF_F3.bits;
const LOCK: u32 = Flags::LOCK.bits;
const REP: u32 = Flags::REP.bits;
const REPE: u32 = Flags::REPE.bits;
const ENC_MR: u32 = Flags::ENC_MR.bits;
const ENC_VM: u32 = Flags::ENC_VM.bits;
const ENC_MIB: u32 = Flags::ENC_MIB.bits;
const X86_ONLY: u32 = Flags::X86_ONLY.bits;

#[allow(dead_code)]
const X64_IMPLICIT: u32 = Features::X64_IMPLICIT.bits;
const FPU: u32 = Features::FPU.bits;
const MMX: u32 = Features::MMX.bits;
const TDNOW: u32 = Features::TDNOW.bits;
const SSE: u32 = Features::SSE.bits;
const SSE2: u32 = Features::SSE2.bits;
const SSE3: u32 = Features::SSE3.bits;
const VMX: u32 = Features::VMX.bits;
const SSSE3: u32 = Features::SSSE3.bits;
const SSE4A: u32 = Features::SSE4A.bits;
const SSE41: u32 = Features::SSE41.bits;
const SSE42: u32 = Features::SSE42.bits;
const SSE5: u32 = Features::SSE5.bits;
const AVX: u32 = Features::AVX.bits;
const AVX2: u32 = Features::AVX2.bits;
const FMA: u32 = Features::FMA.bits;
const BMI1: u32 = Features::BMI1.bits;
const BMI2: u32 = Features::BMI2.bits;
const TBM: u32 = Features::TBM.bits;
const RTM: u32 = Features::RTM.bits;
const INVPCID: u32 = Features::INVPCID.bits;
const MPX: u32 = Features::MPX.bits;
const SHA: u32 = Features::SHA.bits;
const PREFETCHWT1: u32 = Features::PREFETCHWT1.bits;
const CYRIX: u32 = Features::CYRIX.bits;
const AMD: u32 = Features::AMD.bits;

lazy_static! {
    static ref OPMAP: HashMap<&'static str, &'static [Opdata]> = {
        const X: u8 = 0xFF;
        static MAP: &[(&str, &[Opdata])] = &include!("gen_opmap.rs");
        MAP.iter().cloned().collect()
    };
}
