package variables_and_data_types

import "fmt"

func variableDeclaration() {
	// 1. 기본적인 변수 선언 (명시적 타입 지정)
	var num int = 10
	fmt.Println("명시적 타입 변수 선언:", num)

	// 2. 타입 추론을 이용한 변수 선언
	var name = "Go 언어"
	fmt.Println("타입 추론 변수 선언:", name)

	// 3. 짧은 선언 연산자(:=)를 사용한 변수 선언
	// 이 방식은 함수 내에서만 사용 가능 (전역에선 사용 불가)
	age := 30
	fmt.Println("짧은 선언 연산자 사용:", age)

	// 4. 여러 변수의 동시 선언
	var a, b, c int = 1, 2, 3
	fmt.Println("여러 변수 동시 선언:", a, b, c)

	// 5. 기본값 (zero value)
	// 변수 선언만 하고 초기화를 하지 않으면 해당 타입의 기본값이 할당됨
	// int의 기본값은 0, string 기본값은 빈 문자열
	var defaultInt int
	var defaultString string
	fmt.Println("기본값 (int):", defaultInt)
	fmt.Println("기본값 (string):", defaultString)

	// 6. 포인터 변수를 이용한 선언
	// 포인터는 메모리 주소를 저장하는 변수임
	// *는 포인터 타입을 의미하며 &는 변수의 주소를 반환함
	var pointer *int = &num
	fmt.Println("포인터를 이용한 변수 선언:", pointer)
	fmt.Println("포인터가 가리키는 값:", *pointer)
}
