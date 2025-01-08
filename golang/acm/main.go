package main

import (
	"fmt"
	"time"
)

func main() {
	// 현재 시각 가져오기
	currentTime := time.Now()
	// 현재 시각 출력
	fmt.Println("현재 시각:", currentTime) // 날짜, 시간, 나노초, 타임존
	fmt.Println("현재 시각:", currentTime.Format("2006-01-02 15:04:05"))

	fmt.Println("년도:", currentTime.Year())
	fmt.Println("월:", currentTime.Month())
	fmt.Println("일:", currentTime.Day())
	fmt.Println("시간:", currentTime.Hour())
	fmt.Println("분:", currentTime.Minute())
	fmt.Println("초:", currentTime.Second())
	fmt.Println("나노초:", currentTime.Nanosecond())

	// time.Now()는 시스템의 기본 로컬 타임존에 따라 동작함
	// UTC 기준으로 가져오고 싶으면 time.Now().UTC() 사용

	// 시간 간 비교 시 time.Time 내장 메서드인 .Before(), .After(), .Equal() 사용
	time1 := time.Now()
	time.Sleep(1 * time.Second) // 1초 대기
	time2 := time.Now()
	fmt.Println("time1이 time2 이전인가?", time1.Before(time2))
	// time.Now()는 시스템 호출을 포함하므로 매우 빈번하게 호출시 성능에 영향을 줌
	// 특정 코드의 실행 시간을 측정할 때는 time.Now()와 함께 time.Since()를 사용하면 간단
	start := time.Now()
	time.Sleep(2 * time.Second)
	elapsed := time.Since(start)
	fmt.Println("코드 실행 시간:", elapsed)
}
