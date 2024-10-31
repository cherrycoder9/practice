package main

import "fmt"

func anonymousAndClosure() {
	// 익명 함수는 이름이 없는 함수, 변수에 할당하거나 직접 호출할수 있음
	add := func(a int, b int) int {
		return a + b
	}
	fmt.Println("익명 함수 호출 결과:", add(5, 3))

	// 즉시 실행 함수 (IIFE)
	// 함수 정의와 동시에 즉시 실행되는 함수
	result := func(x int, y int) int {
		return x * y
	}(4, 5)
	fmt.Println("즉시 실행 함수 결과:", result)

	// 함수 리터럴과 코드 간결과
	// 익명 함수는 특정 상황에서 재사용을 피할 수 있음
	func(msg string) {
		fmt.Println("함수 리터럴로 출력된 메시지:", msg)
	}("Hello, Go!")

	// 클로저는 함수가 외부 변수에 접근할 수 있도록 하며, 외부 변수 생명주기를 연장함
	counter := getCounter() // getCounter 함수는 클로저를 반환
	fmt.Println("클로저 호출 결과 #1:", counter())
	fmt.Println("클로저 호출 결과 #2:", counter())
	fmt.Println("클로저 호출 결과 #3:", counter())

	// 클로저는 자신이 정의될 때의 외부 변수를 포획해 사용할 수 있음
	multiplier := getMultiplier(3) // multiplier 클로저는 factor 변수를 포획함
	fmt.Println("클로저로 포획한 변수 사용 결과:", multiplier(4))
	fmt.Println("클로저로 포획한 변수 사용 결과:", multiplier(5))

	// 클로저를 사용한 상태 유지 및 캡슐화
	// 클로저를 통해 외부에서 직접 접근할 수 없는 상태를 유지할 수 있음
	nextID := idGenerator() // 이 클로저는 내부의 id 값을 유지함
	fmt.Println("ID 생성 결과 #1:", nextID())
	fmt.Println("ID 생성 결과 #2:", nextID())
	fmt.Println("ID 생성 결과 #3:", nextID())

}

// getCounter 함수는 count 변수를 클로저로 캡슐화해 반환함
func getCounter() func() int {
	count := 0 // count 변수는 getCounter가 반환한 함수에서만 접근 가능
	return func() int {
		count++
		return count
	}
}

// getMultiplier 함수는 factor를 포획해 클로저로 반환
func getMultiplier(factor int) func(int) int {
	fmt.Println("factor: ", factor)
	return func(value int) int {
		fmt.Println("value: ", value)
		return factor * value
	}
}

// idGenerator 함수는 고유 ID를 생성하는 클로저를 반환함
func idGenerator() func() int {
	id := 0 // id 변수는 클로저 내부에서만 유지됨
	return func() int {
		id++
		return id
	}
}
