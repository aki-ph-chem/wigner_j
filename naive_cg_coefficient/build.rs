fn main() {
    cc::Build::new()
        .file("src/c/cg_coefficient_c.c")
        .include("src")
        .compile("libcg_coefficient_c.a");
}
