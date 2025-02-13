const std = @import("std");

pub fn main() !void {
    // # 포인터와 참조 모델
    // Zig는 C 스타일의 포인터가 있지만, 더 안전한 방식으로 관리됨
    const number: i32 = 42;
    const ptr: *const i32 = &number; // 상수 포인터
    // *T -> 가변 포인터
    // *const T -> 상수 포인터
    // [N]*T -> 배열 포인터
    std.debug.print("포인터 주소: {p}\n", .{ptr}); // 포인터 주소: i32@7ff7c30bd600
    std.debug.print("포인터 주소: {x}\n", .{@intFromPtr(ptr)}); // 16진수로 출력

    // # 타입 변환과 캐스팅
    // Zig는 명시적인 타입 변환을 강제하며, 암시적 변환을 허용하지 않음
    const a: i32 = 100;
    // const b: u32 = @as(u32, a); // 이렇게 해도 됨
    const b: u32 = @intCast(a);
    std.debug.print("{}\n", .{b});

    // # 컴파일 타임 타입 검사 comptime
    const name = getTypeName(i32);
    std.debug.print("{s}\n", .{name});
}

fn getTypeName(comptime T: type) []const u8 {
    // @typeName()은 Zig의 빌트인 함수중 하나로, 주어진 타입 이름을 문자열로 반환하는 함수
    // 주로 디버깅, 로깅, 또는 제네릭 코드에서 타입 정보를 얻을 때 사용됨
    // 컴파일 타임 함수이므로 반드시 comptime 컨텍스트에서 실행되어야 함
    return @typeName(T);
}
