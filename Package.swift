// swift-tools-version:5.3

import Foundation
import PackageDescription

let package = Package(
    name: "TreeSitterPython",
    products: [
        .library(name: "TreeSitterPython", targets: ["TreeSitterPython"]),
    ],
    dependencies: [
        .package(name: "SwiftTreeSitter", url: "https://github.com/tree-sitter/swift-tree-sitter", from: "0.9.0"),
    ],
    targets: [
        .target(
            name: "TreeSitterPython",
            dependencies: [],
            path: ".",
            sources: [
                "src/parser.c",
                "src/scanner.c"
            ],
            resources: [
                .copy("queries")
            ],
            publicHeadersPath: "bindings/swift",
            cSettings: [.headerSearchPath("src")]
        ),
        .testTarget(
            name: "TreeSitterPythonTests",
            dependencies: [
                "SwiftTreeSitter",
                "TreeSitterPython",
            ],
            path: "bindings/swift/TreeSitterPythonTests"
        )
    ],
    cLanguageStandard: .c11
)
