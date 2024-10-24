package constants_and_literals

import "fmt"

func typesOfLiterals() {
	// Go 언어의 리터럴은 값 그 자체를 의미함

	// 정수형 리터럴
	// 10진수, 16진수, 8진수로 표현 가능함
	var decimalLiteral int = 42
	var hexLiteral int = 0x2A  // 16진수 리터럴, 접두사 0x 사용
	var octalLiteral int = 052 // 8진수 리터럴, 접두사 0 사용
	fmt.Printf("10진수=%d, 16진수=%d, 8진수=%d\n", decimalLiteral, hexLiteral, octalLiteral)

	// 부동소수점 리터럴
	// 실수형 값을 나타내는 리터럴. 10진수와 지수 표기법 사용 가능
	var floatLiteral float64 = 3.14       // 부동소수점 리터럴
	var scientificLiteral float64 = 1.2e3 // 지수 표기법, 1.2 * 10^3
	fmt.Printf("기본형=%f, 지수표기법=%f\n", floatLiteral, scientificLiteral)

	// 문자열 리터럴 (String Literal)
	// 큰 따옴표를 사용한 이스케이스 시퀀스를 포함할 수 있는 문자열 리터럴과
	// 백틱을 사용한 Raw 문자열 리터럴
	var stringLiteral string = "안녕하세요, Go 언어입니다."          // 큰따옴표를 사용한 일반 문자열 리터럴
	var rawStringLiteral string = `이스케이프 시퀀스 \n도 그대로 출력됨.` // 백틱을 사용한 Raw 문자열 리터럴
	fmt.Printf("일반형=%s, Raw형=%s\n", stringLiteral, rawStringLiteral)

	// 불리언 리터럴
	var trueLiteral bool = true
	var falseLiteral bool = false
	fmt.Printf("참=%t, 거짓=%t\n", trueLiteral, falseLiteral)

	// 문자 리터럴 (Rune Literal)
	// 작은 따옴표를 사용해 유니코드 코드 포인트를 표현함
	// Go에서 문자 리터럴은 rune 타입으로 간주됨
	var charLiteral rune = '가'
	// 문자 리터럴 예시: 가 (유니코드: U+AC00)
	fmt.Printf("문자 리터럴 예시: %c (유니코드: %U)\n", charLiteral, charLiteral)
}
