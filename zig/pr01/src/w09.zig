const std = @import("std");

pub fn main() !void {
    // #1 Zig 메모리 할당 모델
    // allocator 개념을 사용해 수동으로 메모리 관리
    // 표준 라이브러리에서 여러 allocator 제공, 적절한 것을 선택해 메모리 관리
    // std.mem.Allocator는 아래와 같은 주요 기능 제공
    // allocator.alloc(T, count): T 타입의 배열을 count만큼 할당
    // allocator.free(ptr): 할당한 메모리 해제
    // allocator.realloc(ptr, new_size): 메모리 크기 조정

    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    // 10개의 i32 배열 동적 할당
    const array = try allocator.alloc(i32, 10);
    defer allocator.free(array);

    // *elem은 패턴 매칭 문법의 일부로, 각 요소가 포인터임을 감안하여 바인딩하도록 지시함
    // 해당 단계에선 실제 값을 추출하지 않고, 이 변수가 포인터임을 표시하는 역할
    for (array, 0..) |*elem, i| {
        // 바인딩된 변수 elem은 포인터이므로, 그 실제 값에 접근하려면 elem.*를 사용해야 함
        elem.* = @intCast(i); // 포인터 elem이 가리키는 메모리 위치에 변환한 정수 값을 할당
    }

    for (array) |elem| {
        std.debug.print("{}\n", .{elem});
    }

    // #2 Allocator 종류
    // 기본 제공되는 것 이외에 필요한 경우 직접 구현할수도 있음
    // std.heap.page_allocator: 운영체제 페이지 할당기, OS의 mmap 또는 VirtualAlloc을 직접 사용
    // std.heap.GeneralPurposeAllocator: 범용 할당기, malloc/free 대체, 디버깅 지원
    // std.heap.FixedbufferAllocator: 고정 크기 버퍼 할당기, 정적인 버퍼에서 동적 할당 지원
    // std.heap.ArenaAllocator: 일괄 할당 후 해제, 빠른 할당/해제, 메모리 재사용
    // 가령, FixedBufferAllocator를 사용하면 힙이 아닌 스택에서 동적 할당을 수행함
    var buffer: [1024]u8 = undefined;
    // FixedBufferAllocator가 상태(state)를 가지는 구조체이기 때문에 var로 선언함
    // 내부적으로 버퍼 사용 상태를 추적하기 때문에 구조체 인스턴스가 수정될 가능성이 있음
    var allocator2 = std.heap.FixedBufferAllocator.init(&buffer); // 초기화
    const allocator3 = allocator2.allocator(); // Allocator 인터페이스 가져오기

    const mem = try allocator3.alloc(u8, 100);
    defer allocator3.free(mem); // 할당 해제

    std.debug.print("스택에 100 바이트가 할당되었습니다.\n", .{});

    // #3 동적 할당이 필요한 함수는 allocator를 파라미터로 받는것이 좋다

    // #4 대량 할당 후 일괄해제시 Arena Allocator 활용
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit(); // 한번에 전체 메모리 해제

    const allocator4 = arena.allocator();

    var @"_" = try allocator4.alloc(u8, 100);
    @"_" = try allocator4.alloc(u8, 200);
    @"_" = try allocator4.alloc(u8, 50);
}

// #3
// 이렇게 하면 어떤 할당 전략을 사용할지 호출자가 결정할 수 있다
fn createBuffer(allocator: std.mem.Allocator, size: usize) ![]u8 {
    return try allocator.alloc(u8, size);
}
