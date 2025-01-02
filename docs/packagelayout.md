# Package Layout

source: [https://doc.rust-lang.org/cargo/guide/project-layout.html](https://doc.rust-lang.org/cargo/guide/project-layout.html)

Cargo uses conventions for file placement to make it easy to dive into a new Cargo [package](https://doc.rust-lang.org/cargo/appendix/glossary.html#package):

```
.
├── Cargo.lock
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── main.rs
│   └── bin/
│       ├── named-executable.rs
│       ├── another-executable.rs
│       └── multi-file-executable/
│           ├── main.rs
│           └── some_module.rs
├── benches/
│   ├── large-input.rs
│   └── multi-file-bench/
│       ├── main.rs
│       └── bench_module.rs
├── examples/
│   ├── simple.rs
│   └── multi-file-example/
│       ├── main.rs
│       └── ex_module.rs
└── tests/
    ├── some-integration-tests.rs
    └── multi-file-test/
        ├── main.rs
        └── test_module.rs
```

* `Cargo.toml` and `Cargo.lock` are stored in the root of your package (package root).
* Source code goes in the `src` directory.
* The default library file is `src/lib.rs`.
* The default executable file is `src/main.rs`.
    * Other executables can be placed in `src/bin/`.
* Benchmarks go in the `benches` directory.
* Examples go in the `examples` directory.
* Integration tests go in the `tests` directory.

If a binary, example, bench, or integration test consists of multiple source files, place a `main.rs` file along with the extra [modules](https://doc.rust-lang.org/cargo/appendix/glossary.html#module) within a subdirectory of the `src/bin`, `examples`, `benches`, or `tests` directory. The name of the executable will be the directory name.

You can learn more about Rust’s module system in [the book](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html).

