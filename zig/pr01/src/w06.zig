const std = @import("std");

// # 함수 선언 및 기본 구조
fn add(a: i32, b: i32) i32 {
    return a + b;
}

// # void 반환: 반환값이 필요 없는 경우
fn sayHello() void {
    std.debug.print("헬로 zig\n", .{});
}

// # 다중 반환값 (튜플 형태)
fn getCoordinates() struct { i32, i32 } {
    return .{ 10, 20 };
}

// # 오류 반환(error! 타입)
// 함수가 실패할 가능성이 있다면 error!T 형태로 정의
fn safeDivide(a: i32, b: i32) error{DivideByZero}!i32 {
    if (b == 0) {
        return error.DivideByZero;
    }
    // return a / b; // 부호 있는 정수 나눗셈 시 / 연산자를 사용하면 안됨
    // @divTrunc(a, b): 나눗셈 결과를 0에 가까운 방향으로 버림함 (C스타일)
    // @divFloor(a, b): 나눗셈 결과를 음의 무한대로 버림함
    // @divExact(a, b): 나눗셈이 정확히 떨어지는 경우에만 결과를 반환하고 그렇지 않으면 런타임 오류 발생시킴
    return @divTrunc(a, b);
}

// # 컴파일 타임에 실행되는 함수 정의
// 최근 Zig 버전에서는 comptime 키워드를 생략해도 됨
// 컴파일 타임에 평가되어야 하는 요소들을 Zig 컴파일러가 자동으로 처리함
fn square(x: i32) i32 {
    return x * x;
}

pub fn main() !void {
    const result = add(10, 20);
    std.debug.print("결과: {}\n", .{result});
    sayHello();
    const coords = getCoordinates();
    std.debug.print("x: {}, y: {}\n", .{ coords[0], coords[1] });

    const result2 = safeDivide(10, 0) catch |err| {
        std.debug.print("에러: {}\n", .{err});
        return;
    };
    std.debug.print("결과: {}\n", .{result2});
    const value = square(5);
    std.debug.print("제곱결과: {}\n", .{value});
}
