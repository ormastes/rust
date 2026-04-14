// FULL-SOURCE FILE — copy to compiler/rustc_target/src/spec/riscv64gc_unknown_simpleos.rs
// Built-in rustc target spec for riscv64gc-unknown-simpleos.
// Modeled on riscv64gc_unknown_none_elf.rs; mirrors
// src/os/toolchain/rust/riscv64gc-unknown-simpleos.json.

use crate::spec::{Cc, LinkerFlavor, Lld, PanicStrategy, RelocModel, StackProbeType, Target, TargetOptions};

pub fn target() -> Target {
    let opts = TargetOptions {
        os: "simpleos".into(),
        env: "".into(),
        vendor: "unknown".into(),
        linker_flavor: LinkerFlavor::Gnu(Cc::No, Lld::Yes),
        linker: Some("rust-lld".into()),
        executables: true,
        panic_strategy: PanicStrategy::Abort,
        relocation_model: RelocModel::Static,
        code_model: Some(crate::spec::CodeModel::Medium),
        disable_redzone: true,
        max_atomic_width: Some(64),
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
        features: "+m,+a,+f,+d,+c".into(),
        llvm_abiname: "lp64d".into(),
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
        llvm_target: "riscv64-unknown-none-elf".into(),
        metadata: crate::spec::TargetMetadata {
            description: Some("riscv64gc SimpleOS (imafdc, lp64d, static, simpleos.ld)".into()),
            tier: Some(3),
            host_tools: Some(false),
            std: Some(true),
        },
        pointer_width: 64,
        data_layout: "e-m:e-p:64:64-i64:64-i128:128-n32:64-S128".into(),
        arch: "riscv64".into(),
        options: opts,
    }
}
