package main

import (
	"fmt"
	"time"
)

func main() {
	// 현재 시각 가져오기
	currentTime := time.Now()
	// 현재 시각 출력
	fmt.Println("현재 시각:", currentTime)
	fmt.Println("현재 시각:", currentTime.Format("2006-01-02 15:04:05"))
}
