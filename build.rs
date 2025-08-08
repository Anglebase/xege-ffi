fn main() {
    println!("cargo:rustc-link-lib=zlib");
    println!("cargo:rustc-link-lib=libpng");
    println!("cargo:rustc-link-lib=xege");

    //指定查找路径
    println!("cargo:rustc-link-search=./lib");
}