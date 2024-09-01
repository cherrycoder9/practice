package main

var k = 10

func Zero4() {
	// 단일 선언
	var x, y int = 10, 20

	// 다중 선언
	var (
		a int    = 1
		b string = "hello"
		c bool   = true
	)
	println(x, y, a, b, c)

	// 섀도잉
	var k = 20 // 패키지 범위의 k를 가림, 함수 내에서만 사용 가능하게 됨
	println(k)

	// Go에서는 변수의 메모리 관리를 자동으로 수행함
	// 하지만 포인터를 통해 메모리 주소를 다룰 수 있음
	var m = 10
	var p = &m  // m의 메모리 주소를 가리키는 포인터 p
	println(*p) // 포인터 p가 가리키는 값 출력
}
