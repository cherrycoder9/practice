const std = @import("std");

pub fn main() !void {
    // # 배열 []T
    // 같은 타입의 원소를 연속적으로 저장하는 자료구조
    // [_]는 배열의 길이를 자동으로 추론하라는 것
    const numbers = [_]i32{ 1, 2, 3 };
    // const numbers: [3]i32 = [_]i32{ 1, 2, 3 }; // 가능
    std.debug.print("{any}\n", .{numbers});

    // # 슬라이스 []const T
    // 슬라이스는 길이가 고정되지 않은 배열을 참조할 때 사용
    const text: []const u8 = "Hello";
    std.debug.print("{s}\n", .{text});

    // # 구조체 struct
    const Person = struct {
        name: []const u8,
        age: u8,
    };
    // .을 이용한 지정 초기자 문법은 코드의 명확성과 안정성을 높이기 위한 Zig의 설계 철학
    const mina: Person = .{
        .name = "Park Mina",
        .age = 20,
    };
    std.debug.print("이름: {s}, 나이: {}\n", .{ mina.name, mina.age });

    // # 열거형 enum
    // 기본적으로 u32로 저장됨
    const Color = enum {
        Red,
        Green,
        Blue,
    };
    std.debug.print("{any}\n", .{Color.Blue});
}
