package main

var a = 1 // 패키지 범위, 패키지 내 어디서나 접근 가능

func Zero2() {
	var b = 2 // 함수 범위, main 함수 내에서만 접근 가능
	if true {
		var c = 3 // 블록 범위, if블록 내에서만 접근 가능
		println(a, b, c)
	}
}
