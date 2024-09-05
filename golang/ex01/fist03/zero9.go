package main

import (
	"fmt"
	"math"
)

// 가변 인수와 다중 반환값을 이용한 통계 함수
// 정수의 가변 인수를 받아 그 중 최대값, 최소값, 평균값을 반환하는 함수 작성
// 이 함수는 세 개의 값을 동시에 반환해야 함
// 평균값은 소수점 이하 두 자리까지 표시되도록 할것

// 정수의 가변 인수를 받아 최대값, 최소값, 평균값을 반환하는 함수
func CalculateStatistics(nums ...int) (max int, min int, avg float64) {
	// 만약 인수가 없으면, -을 반환하고 함수 종료
	if len(nums) == 0 {
		fmt.Println("입력된 숫자가 없습니다.")
		return 0, 0, 0.0
	}

	// 초기값 설정, max와 min을 첫번째 인수로 설정
	max = nums[0]
	min = nums[0]
	sum := 0 // 전체 합계를 저장할 변수

	// 가변 인수로 받은 슬라이스 반복
	for _, num := range nums {
		// 최대값 계산
		if num > max {
			max = num
		}
		// 최소값 계산
		if num < min {
			min = num
		}
		// 합계에 현재 숫자를 더함
		sum += num
	}

	// 평균값 계산 (float64로 변환해 소수점 처리가 되도록 함)
	avg = float64(sum) / float64(len(nums))

	// 평균값을 소수점 둘째 자리에서 반올림
	// math.Round는 소수점 n자리에서 반올림하는 함수
	avg = math.Round(avg*100) / 100

	return
}

func Zero9() {
	max, min, avg := CalculateStatistics(3, 5, 7, 2, 8, 10, 4)

	// 결과 출력
	fmt.Printf("최대값: %d\n", max)
	fmt.Printf("최소값: %d\n", min)
	fmt.Printf("평균값: %.2f\n", avg)
}
