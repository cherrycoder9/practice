package main

import "fmt"

// 2차원 배열과 레이블을 활용한 합계 계산
// 주어진 2차원 배열 matrix에서, 배열의 특정 값 target이 등장하는
// 모든 행과 열에 있는 숫자의 합을 계산해 반환하는 함수 작성
// target 값이 존재하지 않을 경우엔 -1을 반환
// 레이블과 반복문을 활용해 반복문 최적화

// 특정 값 target이 등장하는 모든 행과 열의 합을 계산하는 함수
func SumOfTargetRowsAndCols(matrix [][]int, target int) int {
	// 행과 열의 개수를 추출
	numRows := len(matrix) // 행의 개수
	if numRows == 0 {
		return -1 // 빈 배열이면 -1을 반환
	}
	numCols := len(matrix[0]) // 열의 개수

	// target이 존재하는 행과 열을 기록할 맵(map)을 만듬
	rowsToSum := make(map[int]bool) // target이 있는 행을 기록하는 맵
	colsToSum := make(map[int]bool) // target이 있는 열을 기록하는 맵

	// target이 배열의 어느 위치에 있는지 찾기 위해 중첩 반복문 사용
	for i := 0; i < numRows; i++ {
		for j := 0; j < numCols; j++ {
			if matrix[i][j] == target {
				// target 값을 찾으면 해당 위치의 행과 열을 기록
				rowsToSum[i] = true
				colsToSum[j] = true
			}
		}
	}

	// target이 배열에 없으면 -1을 반환
	if len(rowsToSum) == 0 && len(colsToSum) == 0 {
		return -1
	}

	sum := 0 // 결과 합을 저장할 변수

	// 행을 순회하면서 기록된 행들의 모든 요소를 합산
	for i := range rowsToSum {
		for j := 0; j < numCols; j++ {
			sum += matrix[i][j]
		}
	}

	// 열을 순회하면서 기록된 열들의 모든 요소 합산
	for j := range rowsToSum {
		for i := 0; i < numRows; i++ {
			// 이미 더한 행에 있는 값은 제외하기 위해 조건 추가
			if !rowsToSum[i] {
				sum += matrix[i][j]
			}
		}
	}

	return sum // 최종 합계 반환
}
func Zero5() {
	// 테스트를 위한 예제 2차원 배열과 target 값
	matrix := [][]int{
		{1, 2, 3},
		{4, 5, 6},
		{7, 8, 5},
	}
	target := 5

	fmt.Println("target 값이 있는 행과 열의 합: ", SumOfTargetRowsAndCols(matrix, target))
}
