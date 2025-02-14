const std = @import("std");

// #1 callable 함수 포인터
// 함수 포인터를 변수로 저장할 수 있다
const FnType = fn (i32, i32) i32;
fn multiply(a: i32, b: i32) i32 {
    return a * b;
}

// #2 제네릭 함수
// comptime 이용한 제네릭 함수를 만들 수 있음
fn max(comptime T: type, a: T, b: T) T {
    return if (a > b) a else b;
}

pub fn main() !void {
    // #1
    const operation: FnType = multiply;
    const result = operation(4, 5);
    std.debug.print("곱하기 결과: {}\n", .{result});

    // #2
    const result2 = max(i32, 10, 12);
    std.debug.print("최대값: {}\n", .{result2});
}
