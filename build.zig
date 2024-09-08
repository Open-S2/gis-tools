const std = @import("std");
const S2Projection = @import("zig/geometry/s2/coords.zig").S2Projection;

const number_of_pages = 2;

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{
        .default_target = .{
            .cpu_arch = .wasm32,
            .os_tag = .freestanding,
        },
    });
    const options = b.addOptions();
    options.addOption(S2Projection, "S2_PROJECTION", .S2_QUADRATIC_PROJECTION); // Quadratic

    const exe = b.addExecutable(.{
        .name = "s2cell",
        .root_source_file = b.path("zig/s2cell.wasm.zig"),
        .target = target,
        .optimize = .ReleaseSmall,
    });
    exe.root_module.addOptions("buildOptions", options);
    exe.entry = .disabled;
    exe.rdynamic = true;
    exe.stack_size = std.wasm.page_size;
    // all this is experminemental
    // exe.global_base = 6560;
    // exe.import_memory = true;
    // exe.initial_memory = std.wasm.page_size * number_of_pages;
    // exe.max_memory = std.wasm.page_size * number_of_pages;

    // const step = b.step("s2cell", "Build the s2cell executable");
    // step.dependOn(&exe.step);

    b.installArtifact(exe);
}
