const std = @import("std");

pub fn main() !void {
    // # Zig의 에러 처리 방식의 특징
    // 예외 대신 error! 타입을 사용
    // 함수 반환값을 통해 오류를 명확하게 표현
    // 자동 전파 없이 명시적으로 처리
    // 오류 핸들링 비용을 최소화

    // #1 error! 타입
    // 사용자 정의 에러를 열거(enum) 타입처럼 관리
    const result = mightFail();
    // 에러 유니온 타입을 출력하려면 {!} 또는 {any}
    std.debug.print("결과1: {!}\n", .{result});

    // #2 try를 활용한 간결한 에러 전파
    // try 키워드를 사용하면 오류 발생시 자동으로 현재 함수에서 오류 반환
    std.debug.print("결과2: {!}\n", .{readConfigFile()});

    // #3 catch를 이용한 에러 처리
    // catch 키워드를 사용하면 오류를 잡아서 기본 값을 설정하거나 특정 로직 수행할수 있음
    const value = mightFail2() catch 42;
    std.debug.print("결과3: {}\n", .{value});

    // #4 catch |err|
    // 오류 타입을 변수로 받아서 직접 처리할 수 있음
    const value2 = mightFail3() catch |err| {
        std.debug.print("결과4: {!}\n", .{err});
    };
    std.debug.print("결과5: {!}\n", .{value2});

    // #5 사용자 정의 에러 타입
    // 여러 개의 오류를 다루려면 error{} 구문을 사용해 명시적으로 에러 정의
    openDatabase() catch |err| {
        if (err == error.ConnectionFailed) {
            std.debug.print("데이터베이스 연결 실패\n", .{});
        } else {
            std.debug.print("잘못된 인증 정보\n", .{});
        }
        // return;
    };
    std.debug.print("데이터베이스 연결 성공\n", .{});

    // #6 오류 래핑과 복합 오류 처리

    // #7 switch를 활용한 에러 핸들링
    fetchData() catch |err| switch (err) {
        error.NetworkFailure => std.debug.print("네트워크 오류\n", .{}),
        error.Timeout => std.debug.print("시간 초과\n", .{}),
        error.InvalidResponse => std.debug.print("잘못된 응답\n", .{}),
    };
}

// #1
fn mightFail() error{FileNotFound}!void {
    return error.FileNotFound;
}

// #2
fn readConfigFile() !void {
    const file = try std.fs.cwd().openFile("config.txt", .{});
    defer file.close();

    std.debug.print("파일 오픈 성공\n", .{});
}

// #3
fn mightFail2() !i32 {
    return error.Failure;
}

// #4
fn mightFail3() !void {
    return error.Failure;
}

// #5
fn openDatabase() error{ ConnectionFailed, InvalidCredentials }!void {
    return error.ConnectionFailed;
}

// #6
fn readFile() error{ FileNotFound, PermissionDenied }!void {
    return error.FileNotFound;
}

// #6
// ReadFile()이 발생시킨 FileNotFound 오류를 ConfigError로 래핑해
// 상위 함수에서 더 추상적인 오류로 변환할 수 있음
fn loadConfig() error{ConfigError}!void {
    readFile() catch |err| {
        if (err == error.FileNotFound) return error.ConfigError;
        return err;
    };
}

// #7
fn fetchData() error{ NetworkFailure, Timeout, InvalidResponse }!void {
    return error.Timeout;
}
