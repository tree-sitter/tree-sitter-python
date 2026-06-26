const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const shared = b.option(bool, "build-shared", "Build a shared library") orelse true;
    const reuse_alloc = b.option(bool, "reuse-allocator", "Reuse the library allocator") orelse false;

    const library_name = "tree-sitter-python";

    const grammar = b.createModule(.{
        .target = target,
        .optimize = optimize,
        .link_libc = true,
        .pic = if (shared) true else null,
    });
    const lib: *std.Build.Step.Compile = b.addLibrary(.{
        .name = library_name,
        .linkage = if (shared) .dynamic else .static,
        .root_module = grammar,
    });

    grammar.addCSourceFiles(.{
        .files = &.{ "src/parser.c", "src/scanner.c" },
        .flags = &.{"-std=c11"},
    });

    if (reuse_alloc) {
        grammar.addCMacro("TREE_SITTER_REUSE_ALLOCATOR", "");
    }
    if (optimize == .Debug) {
        grammar.addCMacro("TREE_SITTER_DEBUG", "");
    }

    grammar.addIncludePath(b.path("src"));

    b.installArtifact(lib);

    b.installDirectory(.{
        .source_dir = b.path("queries"),
        .install_dir = .prefix,
        .install_subdir = "queries",
        .include_extensions = &.{"scm"},
    });

    const module = b.addModule(library_name, .{
        .root_source_file = b.path("bindings/zig/root.zig"),
        .target = target,
        .optimize = optimize,
    });
    module.linkLibrary(lib);

    const tests = b.addTest(.{
        .root_module = b.createModule(.{
            .root_source_file = b.path("bindings/zig/test.zig"),
            .target = target,
            .optimize = optimize,
        }),
    });
    tests.root_module.addImport(library_name, module);

    const run_tests = b.addRunArtifact(tests);
    const test_step = b.step("test", "Run unit tests");
    test_step.dependOn(&run_tests.step);
}
