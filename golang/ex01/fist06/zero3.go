package main

import (
	"context"
	"fmt"
	"time"
)

// 데드라인을 이용한 취소

func performTaskDeadline(ctx context.Context, taskID int) {
	fmt.Printf("작업 %d 시작됨.\n", taskID)
	select {
	case <-time.After(4 * time.Second):
		fmt.Printf("작업%d 완료됨.\n", taskID)
	case <-ctx.Done():
		fmt.Printf("작업 %d 취소됨: %s\n", taskID, ctx.Err())
	}

}

func Zero3() {
	// 현재 시간으로부터 3초 후 데드라인 설정
	deadline := time.Now().Add(3 * time.Second)
	ctx, cancel := context.WithDeadline(context.Background(), deadline)
	defer cancel()

	// 고루틴에서 작업 수행
	go performTaskDeadline(ctx, 3)

	// 작업 완료 또는 취소 대기
	time.Sleep(5 * time.Second)
}
