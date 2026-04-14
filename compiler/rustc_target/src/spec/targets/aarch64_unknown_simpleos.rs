// FULL-SOURCE FILE — copy to compiler/rustc_target/src/spec/aarch64_unknown_simpleos.rs
// Built-in rustc target spec for aarch64-unknown-simpleos.
// Mirrors src/os/toolchain/rust/aarch64-unknown-simpleos.json.

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
        code_model: Some(crate::spec::CodeModel::Small),
        disable_redzone: true,
        max_atomic_width: Some(128),
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
        features: "+v8a,+strict-align".into(),
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
        llvm_target: "aarch64-unknown-none-elf".into(),
        metadata: crate::spec::TargetMetadata {
            description: Some("aarch64 SimpleOS (v8a, strict-align, static, simpleos.ld)".into()),
            tier: Some(3),
            host_tools: Some(false),
            std: Some(true),
        },
        pointer_width: 64,
        data_layout: "e-m:e-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128-Fn32".into(),
        arch: "aarch64".into(),
        options: opts,
    }
}
