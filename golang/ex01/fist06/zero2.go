package main

import (
	"context"
	"fmt"
	"time"
)

// 타임아웃을 이용한 자동 취소

// 작업을 수행하는 함수
func performTaskTimeout(ctx context.Context, taskID int) {
	fmt.Printf("작업 %d 시작됨\n", taskID)
	select {
	case <-time.After(5 * time.Second):
		fmt.Printf("작업 %d 완료됨.\n", taskID)
	case <-ctx.Done():
		fmt.Printf("작업 %d 취소됨: %s\n", taskID, ctx.Err())
	}
}

func Zero2() {
	// 3초 타임아웃을 가진 컨텍스트 생성
	ctx, cancel := context.WithTimeout(context.Background(), 3*time.Second)
	defer cancel()

	// 고루틴에서 작업 수행
	go performTaskTimeout(ctx, 2)

	// 작업 완료 또는 취소 대기
	time.Sleep(6 * time.Second)
}
