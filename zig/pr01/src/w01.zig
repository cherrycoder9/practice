// zig 표준 라이브러리를 가져옴
const std = @import("std");

// 프로그램의 진입점, 반환 타입은 void
pub fn main() void {
    const stdout = std.io.getStdOut().writer(); // 표준 출력 스트림
    // catch unreachable은 오류 발생시 프로그램이 즉시 종료되도록 함
    // .{}는 튜플 리터럴임. 점 없이 {}만 쓰면 일반 블록 코드로 해석되어 컴파일 오류
    // std.debug.print()나 stdout.print() 같은 함수는 서식 문자열을 처리하는데
    // Zig에서는 항상 명시적으로 매개변수를 전달해야 함
    // .{}는 서식 문자열에 전달할 값이 없음을 의미
    // Zig에서는 빈 인자도 반드시 빈 튜플로 전달해야 함
    stdout.print("hello, zig!\n", .{}) catch unreachable;
    // Zig은 인터프리터처럼 코드를 바로 실행할 수 있음
    // zig run w01.zig
    // 일반적인 바이너리 실행 파일을 만드려면 zig build-exe w01.zig
    // 위는 최적화 없는 디버그 빌드임
    // 릴리스 모드 빌드는 실행 속도가 최적화되며 아래처럼 하면 됨
    // zig build-exe -O ReleaseSafe w01.zig
    // CPU 최적화를 적극 활용해 가장 빠른 실행 속도를 얻기 위해서는 아래처럼 해야함
    // zig build-exe -O ReleaseFast w01.zig
    // 실행 파일 크기를 최소화한 빌드는 아래와 같음
    // zig build-exe -O ReleaseSmall w01.zig
}
