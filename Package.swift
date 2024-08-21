// swift-tools-version:5.3
import PackageDescription

let package = Package(
    name: "TreeSitterPython",
    defaultLocalization: "en",
    products: [
        .library(name: "TreeSitterPython", targets: ["TreeSitterPython"]),
    ],
    dependencies: [
        .package(url: "https://github.com/ChimeHQ/SwiftTreeSitter", from: "0.8.0"),
    ],
    targets: [
        .target(
            name: "TreeSitterPython",
            dependencies: [],
            path: ".",
            sources: [
                "src/parser.c",
                "src/scanner.c",
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
