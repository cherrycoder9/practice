package main

import (
	"fmt"
	"sync"
	"time"
)

// 고루틴을 이용한 팩토리얼 계산
// 사용자가 입력할 숫자의 개수는 3개로 고정
// 각 숫자에 대한 팩토리얼 계산은 별도의 고루틴에서 수행
// 모든 고루틴이 완료된 후, 각 숫자와 팩토리얼 값을 출력
// sync.WaitGroup을 사용해 고루틴의 완료를 기다림

// 팩토리얼 계산을 위한 구조체 정의
type FactorialTask struct {
	number int             // 팩토리얼을 계산할 숫자
	result int             // 계산된 팩토리얼 결과
	wg     *sync.WaitGroup // 고루틴 완료를 대기하기 위한 WaitGroup
}

// 팩토리얼을 계산하는 함수
func calculateFactorial(task *FactorialTask) {
	defer task.wg.Done() // 함수 종료 시 WaitGroup 카운트 감소

	factorial := 1 // 팩토리얼 초기값 설정
	for i := 1; i <= task.number; i++ {
		factorial *= i                    // 팩토리얼 계산
		time.Sleep(10 * time.Millisecond) // 계산 과정 시뮬레이션을 위한 지연
	}

	task.result = factorial // 계산된 팩토리얼 결과 저장
}

func Zero5() {
	var wg sync.WaitGroup             // WaitGroup 초기화
	tasks := make([]FactorialTask, 3) // 3개의 팩토리얼 작업을 위한 슬라이스 생성

	// 사용자로부터 3개의 숫자 입력 받기
	for i := 0; i < 3; i++ {
		fmt.Printf("숫자 %d 입력: ", i+1) // 입력 안내 메시지 출력
		fmt.Scan(&tasks[i].number)    // 숫자 입력 받기
		tasks[i].wg = &wg             // 각 작업에 WaitGroup 참조 할당
	}

	// 각 숫자에 대한 팩토리얼 계산을 고루틴으로 실행
	for i := 0; i < 3; i++ {
		wg.Add(1)                        // WaitGroup 카운트 증가
		go calculateFactorial(&tasks[i]) // 팩토리얼 계산 고루틴 실행
	}

	wg.Wait() // 모든 고루틴이 완료될 때까지 대기

	// 계산된 팩토리얼 결과 출력
	for i := 0; i < 3; i++ {
		fmt.Printf("%d! = %d\n", tasks[i].number, tasks[i].result) // 결과 출력
	}
}
