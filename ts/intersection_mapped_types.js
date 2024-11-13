"use strict";
// PersonEmployee 타입을 가진 객체 생성
const employeeIMT = {
    name: "홍길동",
    age: 30,
    employeeId: 101,
    department: "개발팀"
};
console.log(`직원 정보: ${JSON.stringify(employeeIMT)}`);
// ReadonlyPersonEmployee 타입의 객체 생성
const readonlyEmployee = {
    name: "이순신",
    age: 45,
    employeeId: 102,
    department: "경영팀"
};
console.log(`읽기 전용 직원 정보: ${JSON.stringify(readonlyEmployee)}`);
// 객체 생성
const partialEmployee = {
    name: "김유신"
};
console.log(`선택적 지원 정보: ${JSON.stringify(partialEmployee)}`);
