// FULL-SOURCE FILE — copy to compiler/rustc_target/src/spec/riscv32imac_unknown_simpleos.rs
// Built-in rustc target spec for riscv32imac-unknown-simpleos.
// Modeled on riscv32imac_unknown_none_elf.rs; mirrors
// src/os/toolchain/rust/riscv32imac-unknown-simpleos.json.

use crate::spec::{Arch, Cc, Env, LinkerFlavor, Lld, LlvmAbi, Os, PanicStrategy, RelocModel, StackProbeType, Target, TargetOptions};

pub fn target() -> Target {
    let opts = TargetOptions {
        os: Os::SimpleOs,
        env: Env::Unspecified,
        vendor: "unknown".into(),
        linker_flavor: LinkerFlavor::Gnu(Cc::No, Lld::Yes),
        linker: Some("rust-lld".into()),
        executables: true,
        panic_strategy: PanicStrategy::Abort,
        relocation_model: RelocModel::Static,
        code_model: Some(crate::spec::CodeModel::Medium),
        disable_redzone: true,
        max_atomic_width: Some(32),
        atomic_cas: true,
        stack_probes: StackProbeType::None,
        eh_frame_header: false,
        emit_debug_gdb_scripts: false,
        has_thread_local: false,
        singlethread: true,
        crt_static_default: true,
        crt_static_respected: true,
        dynamic_linking: false,
        position_independent_executables: false,
        static_position_independent_executables: false,
        features: "+m,+a,+c".into(),
        llvm_abiname: LlvmAbi::Ilp32,
        supported_sanitizers: crate::spec::SanitizerSet::empty(),
        pre_link_args: crate::spec::TargetOptions::link_args(
            LinkerFlavor::Gnu(Cc::No, Lld::Yes),
            &[
                "-T",
                "${SDKROOT}/share/simpleos/simpleos.ld",
                "${SDKROOT}/lib/crt0.o",
            ],
        ),
        post_link_args: crate::spec::TargetOptions::link_args(
            LinkerFlavor::Gnu(Cc::No, Lld::Yes),
            &["-lsimpleos_c"],
        ),
        ..Default::default()
    };

    Target {
        llvm_target: "riscv32-unknown-none-elf".into(),
        metadata: crate::spec::TargetMetadata {
            description: Some("riscv32imac SimpleOS (imac, ilp32, static, simpleos.ld)".into()),
            tier: Some(3),
            host_tools: Some(false),
            std: Some(true),
        },
        pointer_width: 32,
        data_layout: "e-m:e-p:32:32-i64:64-n32-S128".into(),
        arch: Arch::RiscV32,
        options: opts,
    }
}
