package main

import (
	"context"
	"fmt"
	"time"
)

// 컨텍스트와 채널을 이용한 동시 작업 취소
func performTaskContextChannel(ctx context.Context, taskID int, done chan<- int) {
	fmt.Printf("작업 %d 시작됨.\n", taskID)
	select {
	case <-time.After(6 * time.Second):
		fmt.Printf("작업 %d 완료됨.\n", taskID)
		done <- taskID
	case <-ctx.Done():
		fmt.Printf("작업 %d 취소됨: %s\n", taskID, ctx.Err())
	}
}

func Zero5() {
	// 취소 가능한 컨텍스트 생성
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	// 완료 신호를 받을 채널 생성
	done := make(chan int)

	// 고루틴에서 작업 수행
	go performTaskContextChannel(ctx, 5, done)

	// 3초 후에 취소 호출
	time.Sleep(3 * time.Second)
	fmt.Println("취소 신호 전송됨.")
	cancel()

	// 작업 완료 대기 또는 취소
	select {
	case taskID := <-done:
		fmt.Printf("작업 %d 성공적으로 완료됨.\n", taskID)
	case <-ctx.Done():
		fmt.Println("메인 함수에서 취소 신호 수신됨.")
	}

	// 채널 닫기
	close(done)
}
