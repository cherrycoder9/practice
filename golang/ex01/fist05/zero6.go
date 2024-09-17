package main

import (
	"fmt"
	"sync"
	"time"
)

// 버퍼링된 채널을 이용한 작업 큐 구현
// 버퍼 크기가 5인 정수형 채널을 생성
// 3개의 워커 고루틴을 실행해 채널에서 작업을 받아 처리
// 메인 고루틴은 10개의 작업(1부터 10까지의 정수)을 채널에 전송한 후 채널을 닫음
// 모든 워커 고루틴이 작업을 완료할 때까지 기다림
// 각 워커는 처리한 작업의 원래 값과 그 제곱을 출력해야 함

// 워커 고루틴이 작업을 처리하는 함수
func processTask(workerID int, taskChan chan int, wg *sync.WaitGroup) {
	defer wg.Done() // 워커가 완료되면 WaitGroup 카운트 감소

	for task := range taskChan { // 채널에서 작업을 수신
		square := task * task                                     // 수신한 작업의 제곱 계산
		fmt.Printf("워커 %d: %d의 제곱은 %d\n", workerID, task, square) // 결과 출력
		time.Sleep(10 * time.Millisecond)                         // 작업 처리 시뮬레이션을 위한 지연
	}
}

func Zero6() {
	// 버퍼 크기가 5인 정수형 채널 생성
	taskQueue := make(chan int, 5)

	var WaitGroup sync.WaitGroup // 고루틴 완료를 기다리기 위한 WaitGroup 생성

	// 3개의 워커 고루틴 실행
	for i := 1; i <= 3; i++ {
		WaitGroup.Add(1)                         // WaitGroup 카운트 증가
		go processTask(i, taskQueue, &WaitGroup) // 워커 고루틴 시작
	}

	// 1부터 10까지의 작업을 채널에 전송
	for j := 1; j <= 10; j++ {
		taskQueue <- j                      // 작업 전송
		fmt.Printf("메인 고루틴: 작업 %d 전송\n", j) // 작업 전송 메시지 출력
	}

	close(taskQueue) // 모든 작업 전송 후 채널 닫기

	WaitGroup.Wait() // 모든 워커 고루틴이 완료될 때까지 대기

	fmt.Println("모든 작업 완료") // 모든 작업 완료 메시지 출력
}
