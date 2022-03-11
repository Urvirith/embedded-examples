const std = @import("std");
const builtin = @import("builtin");

pub fn build(b: *std.build.Builder) void {
    // Add the target in new Zig format
    const target = .{
        .cpu_arch = .thumb,
        .cpu_model = .{ .explicit = &std.Target.arm.cpu.cortex_m33 },
        .os_tag = .freestanding,
        .abi = .eabi,
    };
    
    // Standard release options allow the person running `zig build` to select
    // between Debug, ReleaseSafe, ReleaseFast, and ReleaseSmall.
    // const mode = b.standardReleaseOptions(.ReleaseSmall);

    const obj = b.addObject("main", "src/main.zig");
    obj.setTarget(target);
    obj.setBuildMode(.ReleaseSmall);
}
