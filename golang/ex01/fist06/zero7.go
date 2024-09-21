package main

import (
	"context"
	"fmt"
	"sync"
	"time"
)

// 웹페이지 크롤링 타임아웃 설정
// 여러 웹 페이지 동시 크롤링
// 각 크롤링 작업은 고루틴에서 수행됨
// 타임아웃시 모든 크롤링 작업을 중단
// 완료된 작업과 취소된 작업을 콘솔에 출력

func crawlPageURL(ctx context.Context, url string, wg *sync.WaitGroup) {
	defer wg.Done() // 작업 완료시 WaitGroup 카운터 감소

	// URL 유효성 검사
	if url == "" {
		fmt.Println("빈 URL 입력됨")
		return
	}

	fmt.Printf("웹 페이지 크롤링 시작: %s\n", url)

	// 크롤링 작업 시뮬레이션 (랜덤 시간 소요)
	crawlDuration := time.Duration(2+len(url)) * time.Second

	// 타이머 생성
	timer := time.NewTimer(crawlDuration)

	select {
	case <-ctx.Done():
		// 컨텍스트 취소 신호 수신시 처리
		fmt.Printf("크롤링 취소됨: %s, 이유: %s\n", url, ctx.Err())
		if !timer.Stop() {
			<-timer.C // 타이머가 이미 만료된 경우 채널에서 읽기
		}
	case <-timer.C:
		// 크롤링 완료시 처리
		fmt.Printf("웹 페이지 크롤링 완료: %s\n", url)
	}

}

func Zero7() {
	// 크롤링할 웹 페이지 URL 목록 정의
	urls := []string{
		"https://ziglang.org",
		"https://julialang.org/",
		"https://go.dev/",
		"https://rust-lang.org/",
		"", // 유효성 검사 테스트
	}

	// WaitGroup 생성
	var wg sync.WaitGroup

	// 10초 타임아웃을 가진 컨텍스트 생성
	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel() // 메인 함수 종료시 컨텍스트 취소

	// 모든 크롤링 작업 시작
	for _, url := range urls {
		wg.Add(1)                      // WaitGroup 카운터 증가
		go crawlPageURL(ctx, url, &wg) // 고루틴에서 크롤링 작업 실행
	}

	// 모든 고루틴이 완료될 때까지 대기
	wg.Wait()

	// 모든 작업 완료 메시지 출력
	fmt.Println("모든 크롤링 작업 완료 또는 취소됨")
}
