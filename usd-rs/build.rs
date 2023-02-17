use std::path::PathBuf;

fn main() {
    let usd_lib_path = PathBuf::from(std::env::var("USD_LIB_PATH").expect("Missing USD_LIB_PATH"));
    let usd_include_path = std::env::var("USD_INCLUDE_PATH").expect("Missing USD_INCLUDE_PATH");

    let header_filename = PathBuf::from("../cusd.h").canonicalize().unwrap();
    let header_filename_str = header_filename.to_str().unwrap();
    let cpp_filename = PathBuf::from("../cusd.cpp").canonicalize().unwrap();
    let cpp_filename_str = cpp_filename.to_str().unwrap();

    cc::Build::new()
        .cpp(true)
        .file(&cpp_filename_str)
        .include(&usd_include_path)
        // libusd_usdShade.so references all the others so we only really need to link
        // that, but let's be a bit more explicit.
        .object(usd_lib_path.join("libusd_usd.so"))
        .object(usd_lib_path.join("libusd_usdGeom.so"))
        .object(usd_lib_path.join("libusd_usdShade.so"))
        .flag("-Wno-deprecated")
        .flag("-Wno-unused-parameter")
        .compile("cusd");

    println!("cargo:rerun-if-changed={}", header_filename_str);
    println!("cargo:rerun-if-changed={}", cpp_filename_str);

    let bindings = bindgen::Builder::default()
        .header(header_filename_str)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .derive_default(true)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
