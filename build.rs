fn main() {

    /* link the quad tree static library from the c-data-structures project */
    println!("cargo:rustc-link-lib=static=quad_tree");
    println!("cargo:rustc-link-search=native=c-data-structures/build/quad_tree/");
}
