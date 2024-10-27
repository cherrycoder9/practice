package functions_definition_and_call

import "fmt"

func functionExamples() {
	fmt.Println("단순 함수 호출")
	simpleFunction()

	fmt.Println("매개변수와 반환 값을 갖는 함수 호출")
	result := addTwoNumbers(10, 20)
	fmt.Printf("결과: %d\n", result)

	fmt.Println("네임드 리턴 값을 갖는 함수 호출")
	namedResult := namedReturnFunction(5, 15)
	fmt.Printf("네임드 리턴 결과: %d\n", namedResult)

	fmt.Println("가변 인자를 사용하는 함수 호출")
	variadicResult := sum(1, 2, 3, 4, 5)
	fmt.Printf("가변 인자 함수 결과: %d\n", variadicResult)

	fmt.Println("다중 반환 값")
	quotient, remainder := divideAndRemainder(10, 3)
	fmt.Printf("몫: %d, 나머지: %d\n", quotient, remainder)
}

func simpleFunction() {
	fmt.Println("단순함수 호출완료")
}

// 매개변수 a와 b를 받아 두 값을 더한 결과를 반환
func addTwoNumbers(a int, b int) int {
	return a + b
}

// 두 개의 정수를 받아 더한 값을 네임드 리턴 변수 result에 저장 후 반환
func namedReturnFunction(x int, y int) (result int) {
	result = x + y
	return
}

// 가변 인자를 통해 여러 개의 정수를 받아 모두 더한 결과를 반환
func sum(numbers ...int) int {
	total := 0

	// range 키워드를 사용해 가변 인자를 순회
	for _, num := range numbers {
		total += num
	}
	return total
}

// Go 언어의 함수는 여러 값을 반환할 수 있음
// 파이썬의 tuple 반환과 유사하지만 Go 에서는 명시적임
func divideAndRemainder(a int, b int) (int, int) {
	quotient := a / b
	remainder := a % b
	return quotient, remainder
}
