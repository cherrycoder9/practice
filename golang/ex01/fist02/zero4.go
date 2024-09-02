package main

import (
	"fmt"
	"math"
)

// 주어진 정수 n이 소수인지 판별하는 함수 작성
// n이 소수일 경우, 해당 숫자의 각 자리 숫자들도 더한 값도 소수인지 추가로 확인하고
// 두 조건을 모두 만족하는 경우 true 반환, 그렇지 않으면 false 반환

// IsPrime 함수는 주어진 정수 n이 소수인지 판별
func IsPrime(n int) bool {
	// 소수는 1보다 큰 자연수여야 하므로, n이 2보다 작으면 소수가 아님
	if n <= 1 {
		return false
	}

	// n의 제곱근까지만 검사하면 됨
	sqrtN := int(math.Sqrt(float64(n)))

	// 2부터 n의 제곱근까지 모든 수를 n으로 나눠봄
	// 나누어 떨어지는 수가 있으면 소수가 아님
	for i := 2; i < sqrtN; i++ {
		if n%i == 0 {
			return false
		}
	}

	// 위 조건들을 통과하면 n은 소수임
	return true
}

// SumOfDigits 함수는 주어진 정수 n의 각 자리 숫자들의 합을 반환
func SumOfDigits(n int) int {
	sum := 0

	// n이 0보다 큰 동안 반복함
	for n > 0 {
		// n의 마지막 자리 숫자를 sum에 더함
		sum += n % 10
		// 마지막 자리 숫자를 제거하기 위해 n을 10으로 나눔
		n /= 10
	}

	return sum
}

// IsSpecialPrime 함수는 주어진 n이 소수인지, 그리고 각 자리 숫자 합도 소수인지 판별
// 두 조건을 모두 만족하면 true 반환
func IsSpecialPrime(n int) bool {
	// 먼저 n이 소수인지 확인함
	if !IsPrime(n) {
		return false
	}

	// n의 각 자리 숫자들의 합을 구함
	sumDigits := SumOfDigits(n)

	// 각 자리 숫자들의 합이 소수인지 확인함
	if !IsPrime(sumDigits) {
		return false
	}

	// 두 조건을 모두 만족하므로 true 반환
	return true
}

func Zero4() {
	fmt.Println("정수를 입력하세요: ")

	var number int
	fmt.Scanln(&number)

	if IsSpecialPrime(number) {
		fmt.Println("스페셜 소수")
	} else {
		fmt.Println("스페셜 소수 아님")
	}
}
