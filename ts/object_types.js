"use strict";
// 선택적 프로퍼티를 가진 객체
const product1 = {
    id: 101,
    name: "노트북",
}; // description은 선택적이므로 생략 가능
const product2 = {
    id: 102,
    name: "스마트폰",
    description: "최신 모델 스마트폰"
}; // 선택적 프로퍼티 포함
// 객체 구조 분해는 객체에서 특정 속성을 추출해 변수에 할당하는 방법
// 타입스크립트에서는 구조 분해와 함께 변수의 타입도 명시할 수 있음
const employee = {
    userInfo: {
        name: "김철수",
        age: 30,
    },
    position: "개발자",
    address: {
        city: "서울",
        zipCode: "12345",
    },
};
// 객체 구조 분해를 통해 Employee 객체의 특정 속성 추출
const { userInfo, address } = employee;
console.log(`사용자 이름: ${userInfo.name}`);
console.log(`사용자 나이: ${userInfo.age}`);
console.log(`도시: ${address.city}`);
console.log(`우편번호: ${address.zipCode}`);
// 함수의 매개변수에 객체 타입을 정의해 객체 구조를 명시적으로 요구할 수 있음
function printEmployeeInfo(employee) {
    const { userInfo, position, address } = employee;
    console.log(`직원 이름: ${userInfo.name}`);
    console.log(`직원 직급: ${position}`);
    console.log(`도시: ${address.city}, 우편번호: ${address.zipCode}`);
}
printEmployeeInfo(employee);
