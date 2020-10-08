use crate::spec::{LinkerFlavor, Target, TargetOptions};

pub fn target() -> Target {
    let mut base = super::freebsd_base::opts();
    base.cpu = "ppc64".to_string();
    base.pre_link_args.get_mut(&LinkerFlavor::Gcc).unwrap().push("-m64".to_string());
    base.max_atomic_width = Some(64);

    Target {
        llvm_target: "powerpc64-unknown-freebsd".to_string(),
        pointer_width: 64,
        data_layout: "E-m:e-i64:64-n32:64".to_string(),
        arch: "powerpc64".to_string(),
        target_os: "freebsd".to_string(),
        target_env: String::new(),
        target_vendor: "unknown".to_string(),
        linker_flavor: LinkerFlavor::Gcc,
        options: TargetOptions {
            target_endian: "big".to_string(),
            target_mcount: "_mcount".to_string(),
            ..base
        },
    }
}
