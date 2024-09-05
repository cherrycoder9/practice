package main

import "fmt"

// 메서드를 사용한 연봉 계산기
// Employee 구조체 정의
// CalculateSalary 메서드를 작성해 Employee 구조체에 바인딩
// 주당 근로시간이 160을 초과하면 1.5배의 급여 지급

// Employee 구조체
type Employee struct {
	Name        string // 직원 이름
	HourlyWage  int    // 시간당 임금
	HoursWorked int    // 총 근무 시간
}

// CalculateSalary 메서드
func (e Employee) CalculateSalary() int {
	regularHours := 160
	overtimeMultiplier := 1.5

	var salary int // 최종 연봉

	if e.HoursWorked > regularHours {
		// 초과 근무가 있을 경우
		overtimeHours := e.HoursWorked - regularHours // 초과 근무 시간
		salary = regularHours*e.HourlyWage + int(float64(overtimeHours)*float64(e.HourlyWage)*overtimeMultiplier)
		// 시간 시간에 대한 임금 + 초과 근무 시간에 대한 임금을 계산해 연봉에 합산
	} else {
		// 초과 근무가 없을 경우
		salary = e.HoursWorked * e.HourlyWage // 전체 근무 시간에 대해 임금 계산
	}
	return salary // 최종 연봉 반환
}

func Zero8() {
	// 직원1 생성
	employee1 := Employee{Name: "철수", HourlyWage: 20, HoursWorked: 170}
	// 직원2 생성
	employee2 := Employee{Name: "영호", HourlyWage: 25, HoursWorked: 160}
	// 직원3 생성
	employee3 := Employee{Name: "민재", HourlyWage: 15, HoursWorked: 150}

	// 각 직원 연봉 계산후 출력
	fmt.Printf("%s의 연봉: %d\n", employee1.Name, employee1.CalculateSalary())
	fmt.Printf("%s의 연봉: %d\n", employee2.Name, employee2.CalculateSalary())
	fmt.Printf("%s의 연봉: %d\n", employee3.Name, employee3.CalculateSalary())
}
