package main

func Zero3() {
	// Go는 강타입 언어로, 변수의 타입이 명확하게 지정되어야 함
	// 다른 타입 간의 연산이 필요할 경우, 명시적 타입 변환 필요
	var x int = 42
	var y float64 = float64(x) // int를 float64로 변환
	println(y)                 // +4.200000e+001

	// 상수
	const PI = 3.14
	// 여러 상수의 그룹화와 iota 사용
	// iota: Go에서 상수 선언을 위해 사용되는 특별 식별자, Go의 상수 생성기
	// 상수 선언 블록 내에서 사용되며 각 줄마다 0부터 시작해 1씩 증가하는 정수 값 자동 할당
	const (
		A = iota // 0
		B        // 1
		C        // 2
	)
	const (
		Read    = 1 << iota // 1 << 0 = 1
		Write               // 1 << 1 = 2
		Execute             // 1 << 2 = 4
	)
	println(PI, C, Execute)
}
