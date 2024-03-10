fn main() {
    let path = "./lib";
    let lib = "gogo";

    println!("cargo:rustc-link-search=native={}", path);
    println!("cargo:rustc-link-lib=static={}", lib);
}