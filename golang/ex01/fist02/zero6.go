package main

import (
	"fmt"
	"unicode"
)

// 비밀번호 규칙을 검증하는 함수 작성
// 모든 규칙 만족시 true, 아니면 false 반환
// 1. 비밀번호는 8자 이상
// 2. 하나 이상의 숫자 포함
// 3. 하나 이상의 대문자와 소문자 포함
// 4. 하나 이상의 특수문자를 포함

func IsPasswordStrong(password string) bool {
	// 비밀번호 길이가 8자 이상인지 확인
	if len(password) < 8 {
		// 길이가 8자 미만이면 false 반환
		return false
	}

	// 필요한 조건들을 충족하는지 확인하기 위한 플래그 변수들 초기화
	hasDigit := false   // 숫자 포함 여부 확인
	hasUpper := false   // 대문자 포함 여부 확인
	hasLower := false   // 소문자 포함 여부 확인
	hasSpecial := false // 특수문자 포함 여부 확인

	// 비밀번호의 각 문자에 대해 반복문 실행
	for _, char := range password {
		// unicode 패키지의 IsDigit 함수는 문자가 숫자인지 확인함
		if unicode.IsDigit(char) {
			hasDigit = true // 숫자를 포함하고 있으면 플래그를 true로 설정
		}

		// 문자가 대문자인지 확인
		if unicode.IsUpper(char) {
			hasUpper = true
		}

		// 문자가 소문자인지 확인
		if unicode.IsLower(char) {
			hasLower = true
		}

		// 특수문자 포함 여부 확인
		// !@#$%^&*
		if char == '!' || char == '@' || char == '#' || char == '$' || char == '%' || char == '^' || char == '&' || char == '*' {
			hasSpecial = true
		}
	}

	// 모든 조건을 만족하면 true 반환, 하나라도 만족하지 않으면 false 반환
	return hasDigit && hasUpper && hasLower && hasSpecial
}

func Zero6() {
	// 사용자로부터 비밀번호 입력받음
	fmt.Print("비밀번호: ")
	var password string
	fmt.Scanln(&password)

	if IsPasswordStrong(password) {
		fmt.Println("적합한 비밀번호 입니다.")
	} else {
		fmt.Println("비밀번호를 다시 설정하세요.")
	}
}
