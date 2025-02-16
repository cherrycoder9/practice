const std = @import("std");

pub fn main() !void {
    // #1 제네릭 함수의 타입 제약
    const res = multiplyByTwo(i32, 10);
    // const res = multiplyByTwo(f32, 10.0);
    std.debug.print("{}\n", .{res});

    // #2 제네릭 구조체
    const v1 = Vector(i32){ .x = 3, .y = 4 };
    const v2 = Vector(i32){ .x = 5, .y = 6 };
    const v3 = v1.add(v2);
    std.debug.print("{} {}\n", .{ v3.x, v3.y }); // 8 10
}

// #1
fn multiplyByTwo(comptime T: type, x: T) T {
    return switch (@typeInfo(T)) {
        .int => x * 2,
        // Zig에서 ++는 문자열 연결 연산자임
        else => @compileError("지원되지 않는 타입: " ++ @typeName(T) ++ "\n"),
    };
}

// #2 제네릭 구조체를 반환하는 함수 정의
pub fn Vector(comptime T: type) type {
    return struct {
        x: T,
        y: T,

        pub fn add(self: @This(), other: @This()) @This() {
            return @This(){
                .x = self.x + other.x,
                .y = self.y + other.y,
            };
        }
    };
}
