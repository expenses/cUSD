use std::path::PathBuf;

fn main() {
    let usd_lib_path = PathBuf::from(std::env::var("USD_LIB_PATH").expect("Missing USD_LIB_PATH"));
    let usd_include_path = std::env::var("USD_INCLUDE_PATH").expect("Missing USD_INCLUDE_PATH");

    cc::Build::new()
        .cpp(true)
        .file("../cusd.cpp")
        .include(&usd_include_path)
        // libusd_usdShade.so references all the others so we only really need to link
        // that, but let's be a bit more explicit.
        .object(usd_lib_path.join("libusd_usd.so"))
        .object(usd_lib_path.join("libusd_usdGeom.so"))
        .object(usd_lib_path.join("libusd_usdShade.so"))
        .warnings(false)
        .compile("cusd");

    let header_filename = PathBuf::from("../cusd.hpp").canonicalize().unwrap();
    let header_filename_str = header_filename.to_str().unwrap();

    println!("cargo:rerun-if-changed={}", header_filename_str);

    let bindings = bindgen::Builder::default()
        .header(header_filename_str)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .allowlist_var("cusd_.*")
        .allowlist_function("cusd_.*")
        .opaque_type("cusd_.*")
        .derive_default(true)
        .clang_arg(&format!("-I{}", usd_include_path))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    //let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file("src/bindings.rs")
        .expect("Couldn't write bindings!");
}
