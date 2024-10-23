package constant_and_literals

import "fmt"

func constantDeclaration() {
	// 상수 선언시 const 키워드 사용
	// 상수는 재할당 불가, 컴파일 타임에 값 결정됨

	// 기본 상수 선언
	const pi float64 = 3.14159
	fmt.Println("원주율 값: ", pi)

	// 타입을 지정하지 않는 상수 선언
	// 타입을 추론함
	const greeting = "Hello, Go!"
	fmt.Println("인사말: ", greeting)

	// 여러 상수를 한번에 선언
	const (
		width  = 500 // 정수형 상수로 간주됨
		height = 300
	)
	fmt.Printf("폭: %d, 높이: %d\n", width, height)

	// iota 키워드를 사용한 상수 열거
	// iota는 특별한 키워드로, 0부터 시작해 자동으로 증가하는 상수 값을 정의할 때 사용
	const (
		Sunday    = iota // 0
		Monday           // 1
		Tuesday          // 2
		Wednesday        // 3
		Thursday         // 4
		Friday           // 5
		Saturday         // 6
	)
	fmt.Printf("Sunday: %d, Saturday: %d\n", Sunday, Saturday)

	// 수식에서 사용되는 상수
	// 상수는 컴파일 타임에 평가되므로, 수식에 사용 가능
	const area = width * height
	// area는 폭과 높이를 곱한 값으로 고정됨
	fmt.Println("면적 값: ", area)
}
