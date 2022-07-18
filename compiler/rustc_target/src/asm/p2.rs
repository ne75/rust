use super::{InlineAsmArch, InlineAsmType};
use rustc_macros::HashStable_Generic;
use rustc_span::Symbol;
use std::fmt;

def_reg_class! {
    P2 P2InlineAsmRegClass {
        reg,
    }
}

impl P2InlineAsmRegClass {
    pub fn valid_modifiers(self, _arch: super::InlineAsmArch) -> &'static [char] {
        &[]
    }

    pub fn suggest_class(self, _arch: InlineAsmArch, _ty: InlineAsmType) -> Option<Self> {
        None
    }

    pub fn suggest_modifier(
        self,
        _arch: InlineAsmArch,
        _ty: InlineAsmType,
    ) -> Option<(char, &'static str)> {
        None
    }

    pub fn default_modifier(self, _arch: InlineAsmArch) -> Option<(char, &'static str)> {
        None
    }

    pub fn supported_types(
        self,
        arch: InlineAsmArch,
    ) -> &'static [(InlineAsmType, Option<Symbol>)] {
        match (self, arch) {
            (Self::reg, _) => types! { _: I8, I16, I32; },
        }
    }
}

def_regs! {
    P2 P2InlineAsmReg P2InlineAsmRegClass {
        r0: reg = ["r0"],
        r1: reg = ["r1"],
        r2: reg = ["r2"],
        r3: reg = ["r3"],
        r4: reg = ["r4"],
        r5: reg = ["r5"],
        r6: reg = ["r6"],
        r7: reg = ["r7"],
        r8: reg = ["r8"],
        r9: reg = ["r9"],
        r10: reg = ["r10"],
        r11: reg = ["r11"],
        r12: reg = ["r12"],
        r13: reg = ["r13"],
        r14: reg = ["r14"],
        r15: reg = ["r15"],
        r16: reg = ["r16"],
        r17: reg = ["r17"],
        r18: reg = ["r18"],
        r19: reg = ["r19"],
        r20: reg = ["r20"],
        r21: reg = ["r21"],
        r22: reg = ["r22"],
        r23: reg = ["r23"],
        r24: reg = ["r24"],
        r25: reg = ["r25"],
        r26: reg = ["r26"],
        r27: reg = ["r27"],
        r28: reg = ["r28"],
        r29: reg = ["r29"],
        r30: reg = ["r30"],
        r31: reg = ["r31"],

        ijmp3: reg = ["ijmp3"],
        iret3: reg = ["iret3"],
        ijmp2: reg = ["ijmp2"],
        iret2: reg = ["iret2"],
        ijmp1: reg = ["ijmp1"],
        iret1: reg = ["iret1"],
        pa: reg = ["pa"],
        pb: reg = ["pb"],
        ptra: reg = ["ptra"],
        ptrb: reg = ["ptrb"],
        dira: reg = ["dira"],
        dirb: reg = ["dirb"],
        outa: reg = ["outa"],
        outb: reg = ["outb"],
        ina:  reg = ["ina"],
        inb:  reg = ["inb"],
    }
}

impl P2InlineAsmReg {
    pub fn emit(
        self,
        out: &mut dyn fmt::Write,
        _arch: InlineAsmArch,
        _modifier: Option<char>,
    ) -> fmt::Result {
        out.write_str(self.name())
    }
}
