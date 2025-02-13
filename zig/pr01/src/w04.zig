const std = @import("std");

pub fn main() !void {
    // # if문
    // if문의 조건은 반드시 bool 타입이어야 함
    // if (x) 같은 C 스타일의 조건문은 허용되지 않음
    // 조건이 comptime으로 평가될 경우, 컴파일 타임 최적화 가능
    const x: i32 = 10;
    if (x > 5) {
        std.debug.print("x는 5보다 큽니다.\n", .{});
    }

    // # if/else문
    if (x > 12) {
        std.debug.print("x는 12보다 큽니다.\n", .{});
    } else {
        std.debug.print("x는 12보다 크지 않습니다.\n", .{});
    }

    // # if/else if/else문
    if (x > 12) {
        std.debug.print("x는 12보다 큽니다.\n", .{});
    } else if (x < 5) {
        std.debug.print("x는 5보다 작습니다.\n", .{});
    } else {
        std.debug.print("x는 12보다 크지 않고 5보다 큽니다.\n", .{});
    }

    // # while 반복문
    var i: i32 = 0;
    std.debug.print("while 반복문 1\n", .{});
    while (i < 5) {
        std.debug.print("i: {}\n", .{i});
        i += 1;
    }

    // # while과 continue, break
    // continue: 현재 반복을 건너뛰고 다음 반복으로 이동
    // break: 루프를 즉시 종료
    std.debug.print("while 반복문 2\n", .{});
    while (i < 19) {
        i += 1;
        // if (i % 2 == 0) { // zig에선 안됨.
        // zig에선 나머지 연산을 사용할때 정수 타입과 comptime_int 간의 연산을 명확하게 해야함
        // 특히 signed integer (i32) 에서 나머지 연산을 할경우, @rem 또는 @mod를 사용해야함
        // @rem(i, 2) 또는 @mod(i, 2)
        // @rem(a, b): 나머지 연산, a % b와 동일하지만 정수 타입에서만 사용 가능
        // @mod(a, b): 모듈로 연산, b의 부호를 따라감
        if (@rem(i, 2) == 0) {
            continue; // 짝수 건너뜀
        }
        if (i > 16) {
            break; //
        }
        std.debug.print("i: {}\n", .{i});
    }

    // # for 반복문
    // 배열, 슬라이스, 튜플, 문자열 등을 순회
    const numbers = [_]i32{ 1, 2, 3, 4, 5 };
    for (numbers) |num| { // num은 배열의 각 요소
        std.debug.print("숫자: {}\n", .{num});
    }

    // # for와 인덱스 사용
    // 0..을 100.. 으로 바꾸면 100~104 인덱스로 출력됨
    for (numbers, 0..) |num, index| {
        std.debug.print("인덱스: {}, 값: {}\n", .{ index, num });
    }
}
