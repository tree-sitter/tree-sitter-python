{
  "targets": [
    {
      "target_name": "tree_sitter_python_binding",
      "include_dirs": [
        "<!(node -e \"require('nan')\")",
        "src",
      ],
      "sources": [
        "bindings/node/binding.cc",
        "src/parser.c",
        "src/scanner.c",
      ],
      "cflags_c": [
        "-std=c11",
      ],
      "cflags_cc": [
        "-Wno-cast-function-type",
      ]
    }
  ]
}
