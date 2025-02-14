const std = @import("std");

pub fn main() !void {
    const value: i32 = 2;
    // # switch 문
    switch (value) {
        1 => std.debug.print("값은 1입니다.\n", .{}),
        2 => std.debug.print("값은 2입니다.\n", .{}),
        3, 4 => std.debug.print("값은 3 또는 4입니다.\n", .{}),
        else => std.debug.print("값이 범위안에 없습니다.\n", .{}),
    }
    // zig의 switch 문은 else 키워드로 기본값을 설정할 수 있음
    // 여러 개 값을 하나의 경우로 처리가능
    // switch 문은 comptime과 결합해 컴파일 타임 최적화 가능

    // # 컴파일 타임 조건문
    // comptime을 사용해 컴파일 타임에 조건을 평가해 최적화
    const is_i32_4_bytes = comptime @sizeOf(i32) == 4;

    // 런타임에서 출력
    if (is_i32_4_bytes) {
        std.debug.print("i32는 4바이트입니다.\n", .{});
    }
}
