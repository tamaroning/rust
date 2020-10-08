use crate::spec::{LinkerFlavor, Target};

pub fn target() -> Target {
    let mut base = super::openbsd_base::opts();
    base.target_endian = "big".to_string();
    base.cpu = "v9".to_string();
    base.pre_link_args.get_mut(&LinkerFlavor::Gcc).unwrap().push("-m64".to_string());
    base.max_atomic_width = Some(64);

    Target {
        llvm_target: "sparc64-unknown-openbsd".to_string(),
        pointer_width: 64,
        data_layout: "E-m:e-i64:64-n32:64-S128".to_string(),
        arch: "sparc64".to_string(),
        target_env: String::new(),
        target_vendor: "unknown".to_string(),
        linker_flavor: LinkerFlavor::Gcc,
        options: base,
    }
}
