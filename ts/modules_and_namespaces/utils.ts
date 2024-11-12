// 유틸리티 함수들을 정의하는 파일

// 네임스페이스는 코드를 논리적으로 그룹화하고 이름 충돌을 방지하기 위해 사용

export namespace Utils {
    // 이메일 주소 유효성 검사 함수
    export function isValidEmail(email: string): boolean {
        const emailRegex = /^[\w-\.]+@[\w-]+\.[a-z]{2,4}$/;
        // test() 메서드는 JS, TS에서 정규 표현식 객체의 메서드임
        // 문자열이 특정 정규 표현식과 일치하는지 확인하는 역할을 함 
        return emailRegex.test(email);
    }

    // 유저 이름이 유효한지 검사하는 함수
    export function isValidUserName(name: string): boolean {
        return name.length >= 2;
    }
}