use crate::spec::{LinkerFlavor, LldFlavor, RelocModel, Endian, PanicStrategy, Target, TargetOptions};

/// A base target for P2 devices using the LLVM toolchain.
pub fn target() -> Target {
    Target {
        arch: "p2".into(),
        data_layout: "e-p:32:32-i32:32-i64:32".into(),
        llvm_target: "p2".into(),
        pointer_width: 32,

        options: TargetOptions {
            endian: Endian::Little,
            linker_flavor: LinkerFlavor::Lld(LldFlavor::Ld),
            max_atomic_width: Some(0),
            executables: true,
            linker: Some("ld.lld".into()),
            position_independent_executables: false,
            exe_suffix: ".elf".into(),
            panic_strategy: PanicStrategy::Abort,
            relocation_model: RelocModel::Static,
            emit_debug_gdb_scripts: false,
            ..TargetOptions::default()
        },
    }
}