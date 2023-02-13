extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let usd_lib_path = "../../external/usd/lib";
    let usd_include_path = "../../external/usd/include";

    // Tell cargo to look for shared libraries in the specified directory
    println!(
        "cargo:rustc-link-search={}",
        PathBuf::from(".").canonicalize().unwrap().display()
    );
    println!(
        "cargo:rustc-link-search={}",
        PathBuf::from(usd_lib_path)
            .canonicalize()
            .unwrap()
            .display()
    );

    let link_libs = [
        //"static=stdc++",
        "cusd",
        "usd_usdGeom",
        "usd_usd",
        "usd_sdf",
        "usd_vt",
        "usd_gf",
        "usd_tf",
        "usd_usdLux",
        "usd_usdShade",
    ];

    for link_lib in link_libs {
        println!("cargo:rustc-link-lib={}", link_lib);
    }

    println!("cargo:rustc-link-lib=static=stdc++");

    if !std::process::Command::new("clang")
        .arg("-c")
        .arg("-o")
        .arg("cusd.o")
        .arg("../cusd.cpp")
        .arg(&format!("-I{}", usd_include_path))
        .arg("-O3")
        .output()
        .expect("could not spawn `clang`")
        .status
        .success()
    {
        // Panic if the command was not successful.
        panic!("could not compile object file");
    }

    if !std::process::Command::new("ar")
        .arg("rcs")
        .arg("libcusd.a")
        .arg("cusd.o")
        .output()
        .expect("could not spawn `ar`")
        .status
        .success()
    {
        // Panic if the command was not successful.
        panic!("could not emit library file");
    }

    let bindings_filename = PathBuf::from("../cusd.hpp").canonicalize().unwrap();

    println!("cargo:rerun-if-changed={}", bindings_filename.display());

    let mut builder = bindgen::Builder::default()
        .header(&format!("{}", bindings_filename.display()))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .allowlist_var("cusd_.*")
        .allowlist_function("cusd_.*")
        .opaque_type("cusd_.*")
        .derive_default(true)
        .clang_arg(&format!("-I{}", usd_include_path));

    let bindings = builder.generate().expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    //let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file("src/bindings.rs")
        .expect("Couldn't write bindings!");
}
