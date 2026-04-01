fn main() {
    let manifest_dir =
        std::path::PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap());

    let library_paths = std::collections::HashMap::from([(
        "sleek-ui".to_string(),
        manifest_dir.join("sleek-ui"),
    )]);

    let config = slint_build::CompilerConfiguration::new()
        .with_library_paths(library_paths);

    slint_build::compile_with_config("dashboard/app.slint", config).unwrap();

    println!("cargo:rerun-if-changed=dashboard/app.slint");
    println!("cargo:rerun-if-changed=sleek-ui");
}
