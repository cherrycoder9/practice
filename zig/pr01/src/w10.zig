const std = @import("std");

pub fn main() !void {
    // #1 메모리 할당과 해제 복습
    // Allocator 생성: std.heap 또는 std.testing.allocator 사용해 적절한 할당기 선택
    // std.heap: 일반적인 프로그램에서 동적 메모리를 관리할 때
    // 메모리 풀, 아레나 할당기 등을 활용해 성능 최적화 가능
    // std.testing.allocator: 단위 테스트에서 메모리 관련 오류를 감지하는데 사용
    // 메모리 누수 감지, 중복 해제 감지, 오버플로우 검출 기능이 있음
    // 메모리 할당: alloc 또는 create 메서드를 사용해 메모리 할당
    // 메모리 해제: free 또는 destroy 호출해 할당된 메모리 해제
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    // 메모리 할당이 실패할 경우 적절한 처리를 하지 않으면 프로그램이 비정상 종료될수 있으므로
    // try와 catch를 사용하면 안전하게 예외 처리 가능
    const buffer = try allocator.alloc(u8, 100);
    // const buffer = allocator.alloc(u8, 100) catch unreachable; // 가능
    // 실행중인 블록이 종료될때 코드 실행(정상 종료든, return이든, 에러가 발생하든)
    defer allocator.free(buffer);

    // #2 메모리 해제 차이
    // free(ptr): 배열을 할당 해제할 때 사용 (메모리 블록 해제)
    // destroy(ptr): 구조체 또는 단일 객체를 해제할 때 사용
    const data = try allocator.create(Data); // 구조체 할당
    defer allocator.destroy(data); // 구조체 해제
    const array = try allocator.alloc(u8, 10); // 배열 할당
    defer allocator.free(array); // 배열 해제
    array[0] = 42;
    std.debug.print("{}", .{array[0]}); // 42
}

// #2
const Data = struct {
    value: u32,
};
