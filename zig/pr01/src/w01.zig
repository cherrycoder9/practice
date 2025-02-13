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
    // catch unreachable 사용하면 에러를 무시함
    stdout.print("hello, zig!\n", .{}) catch unreachable;
    // try를 사용해 에러를 전파할 수도 있음. 에러 발생시 호출한 함수로 전파됨
    // 단, 함수의 반환값에 !를 붙여야함, main() !void
    // try stdout.print("hello, zig!\n", .{});
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

    // 변수 선언시 var 키워드 사용
    // 상수 선언시 const 키워드 사용, 한번 정의되면 변경불가
    var x: i128 = 10; // 128비트 정수형 변수 선언
    x = 20; // 값 변경 가능
    // var y = 30; // 자료형을 명시하지 않으면 컴파일타임 상수로 추론됨, 따라서 컴파일에러
    const y = 30;
    var z: f64 = y;
    z = 10.3;
    // i8, i16, i32, i64, i128 부호 있는 정수
    // u8, u16, u32, u64, u128 부호 없는 정수
    // 숫자는 비트
    // https://ziglang.org/documentation/master/#Primitive-Types
    // f80을 지원하는 이유는 x87의 80비트 부동소수점과의 호환성을 유지하기 위해서

    var flag: bool = true;
    flag = false;
    // std.debug.print()는 주로 디버깅 목적으로 사용함
    // 내부에서 출력 오류를 처리하므로 별도의 에러 처리가 필요없음
    // 일반적으로 표준 에러(stderr)로 출력됨
    std.debug.print("{}\n", .{flag});

    const letter: u31 = 'A';
    std.debug.print("{}\n", .{letter}); // 65

    const greeting: []const u8 = "Hello, Zig!";
    // 슬라이스 타입을 어떻게 출력할지 명확하게 알려줘야함 {s}, {any}
    // 슬라이스는 단순히 값의 나열인지, 문자열로 해석할 것인지 등 애매모호한 부분이 있기 때문
    std.debug.print("{s}\n", .{greeting}); // Hello, Zig!
    std.debug.print("{any}\n", .{greeting}); // {72, 101, 108 ...}
}
