fn main() {
    cc::Build::new()
        .file("src/hello.c")
        .flag("-v")
        .opt_level(1)
        .compile("hello");
}