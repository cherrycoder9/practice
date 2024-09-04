package main

import "fmt"

// 포인터 리시버
// 메서드 내에서 호출자의 상태를 변경할 수 있음

// 사용자 정의 구조체
type Counter struct {
	count int
}

// 포인터 리시버를 사용한 메서드
func (c *Counter) increment() {
	c.count++
}

func Zero6() {
	counter := Counter{count: 0}
	counter.increment()
	counter.increment()
	counter.increment()
	fmt.Println(counter.count) // 3
}
