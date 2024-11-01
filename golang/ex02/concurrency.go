package main

import (
	"fmt"
	"sync"
	"time"
)

func concurrency() {
	fmt.Println("메인 고루틴 시작")

	// WaitGroup을 사용해 고루틴이 모두 완료될 때까지 대기
	var wg sync.WaitGroup

	// 고루틴을 생성해 비동기 처리 시작
	wg.Add(1) // WaitGroup 카운터 증가
	go func() {
		defer wg.Done() // 고루틴이 끝나면 WaitGroup 카운터 감소
		longRunningTask("고루틴 1")
	}()

	// 또 다른 고루틴 생성해 병렬 처리 수행
	wg.Add(1)
	go func() {
		defer wg.Done()
		longRunningTask("고루틴 2")
	}()

	// 메인 고루틴에서 비동기 처리를 위해 추가 작업 수행
	additionalTask("메인 고루틴")

	// 모든 고루틴이 완료될 때까지 대기
	wg.Wait()
	fmt.Println("메인 고루틴 종료")
}

// 오랜 시간이 걸리는 작업을 수행하는 함수
// 이름을 매개변수로 받아 해당 작업을 콘솔에 출력
func longRunningTask(name string) {
	for i := 0; i < 5; i++ {
		fmt.Printf("%s: %d번째 작업 수행 중...\n", name, i+1)
		time.Sleep(1 * time.Second) // 1초 대기
	}
	fmt.Printf("%s: 작업 완료\n", name)
}

// 메인 고루틴에서 수행하는 추가 작업
func additionalTask(name string) {
	for i := 0; i < 3; i++ {
		fmt.Printf("%s: 추가 작업 수행 중...\n", name)
		time.Sleep(500 * time.Millisecond) // 0.5초 대기
	}
}

// 고루틴은 go 키워드를 사용해 생성함
// 비동기 작업을 쉽게 구현 가능

// 익명 함수 형태로 정의된 함수를 고루틴으로 실행할 수 있음
// 별도 메서드 호출 없이 go 키워드로 실행 가능

// sync.WaitGroup을 사용해 고루틴이 종료될 때까지 메인 고루틴이 대기함
