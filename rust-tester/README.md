# Python Rust Tester

A tester to run the corpus tests against the rust scanner (because it's only used in the rust bindings)

Can be run with MIRI to check the unsafe C interface of the scanner

`tests.rs` is pulled from the tree sitter cli tester source (and then trimmed down)
