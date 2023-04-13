mod tests;

fn main() {
    let language = tree_sitter_python::language();
    let corpus_dir = std::path::Path::new("../test/corpus");

    tests::run_tests_at_path(language, &corpus_dir, false, None, false)
        .expect("failed to run tests")
}
