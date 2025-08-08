use std::fs;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let files = fs::read_dir("zlib/src")?
        .into_iter()
        .filter(|entry| entry.is_ok())
        .map(|entry| entry.unwrap())
        .filter(|entry| entry.path().is_file())
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some() && path.extension().unwrap() == "c")
        .map(|path| String::from(path.to_str().unwrap()));

    cc::Build::new()
        .include("zlib/include")
        .include("zlib/include/zlib")
        .files(files)
        .compile("zlib");

    let files = fs::read_dir("libpng/src")?
        .into_iter()
        .filter(|entry| entry.is_ok())
        .map(|entry| entry.unwrap())
        .filter(|entry| entry.path().is_file())
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some() && path.extension().unwrap() == "c")
        .map(|path| String::from(path.to_str().unwrap()));

    cc::Build::new()
        .include("libpng/include")
        .include("libpng/include/libpng")
        .include("zlib/include")
        .include("zlib/include/zlib")
        .files(files)
        .compile("libpng");

    let files = fs::read_dir("xege/src")?
        .into_iter()
        .filter(|entry| entry.is_ok())
        .map(|entry| entry.unwrap())
        .filter(|entry| entry.path().is_file())
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some() && path.extension().unwrap() == "cpp")
        .map(|path| String::from(path.to_str().unwrap()));

    cc::Build::new()
        .include("xege/include")
        .include("xege/src")
        .include("zlib/include")
        .include("zlib/include/zlib")
        .include("libpng/include")
        .include("libpng/include/libpng")
        .flag("/utf-8")
        .files(files)
        .compile("xege");

    // let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());

    // bindgen::Builder::default()
    //     .header("xege/include/ege.cpp")
    //     .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
    //     .generate()
    //     .expect("Unable to generate bindings")
    //     .write_to_file(out_dir.join("bindings.rs"))
    //     .expect("Couldn't write bindings");

    Ok(())
}
