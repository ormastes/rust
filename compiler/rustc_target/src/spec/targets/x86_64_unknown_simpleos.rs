// FULL-SOURCE FILE — copy to compiler/rustc_target/src/spec/x86_64_unknown_simpleos.rs
// Built-in rustc target spec for x86_64-unknown-simpleos.
// Mirrors src/os/toolchain/rust/x86_64-unknown-simpleos.json.

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
        features: "-mmx,-sse,-sse2,-sse3,-ssse3,-sse4.1,-sse4.2,-avx,-avx2,+soft-float".into(),
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
        llvm_target: "x86_64-unknown-none-elf".into(),
        metadata: crate::spec::TargetMetadata {
            description: Some("x86_64 SimpleOS (microkernel, soft-float, static, simpleos.ld)".into()),
            tier: Some(3),
            host_tools: Some(false),
            std: Some(true),
        },
        pointer_width: 64,
        data_layout: "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128".into(),
        arch: "x86_64".into(),
        options: opts,
    }
}
