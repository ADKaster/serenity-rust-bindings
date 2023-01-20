use std::path::{ Path, PathBuf };

fn main() {

    let env_var = std::env::var("SERENITY_SOURCE_DIR").unwrap();
    let serenity_build_dir = Path::new(&env_var);

    let lagom = cmake::Config::new(serenity_build_dir.join("Meta/Lagom"))
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("CMAKE_CXX_COMPILER", "clang++")
        .define("CMAKE_C_COMPILER", "clang")
        .build();
    
    println!("cargo:rustc-link-search=native={}", lagom.join("lib").display());
    println!("cargo:rustc-link-lib=static={}", "lagom-core");

    let mut include_directories = Vec::<PathBuf>::new();
    include_directories.push(serenity_build_dir.to_path_buf());
    include_directories.push(serenity_build_dir.join("Userland/Libraries"));

    cxx_build::bridge("src/lib.rs")  // returns a cc::Build
        .cpp(true)
        .flag_if_supported("-std=c++20")
        .flag_if_supported("-fno-exceptions")
        .flag_if_supported("-Wno-expansion-to-defined")
        .flag_if_supported("-Wno-literal-suffix")
        .define("AK_DONT_REPLACE_STD", "1")
        .includes(include_directories)
        .include("include")
        .compile("wrapper");

    println!("cargo:rustc-link-lib=static={}", "wrapper");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=include/Shim.h");
}
