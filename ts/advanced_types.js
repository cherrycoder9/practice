"use strict";
// 조건부 타입
// 타입이 특정 조건에 따라 달라지게 하여 유연성을 극대화하는 타입 시스템 기능
const addART = (a, b) => {
    return a + b;
};
const fetchData = async () => {
    return "데이터를 가져왔습니다.";
};
const userMNU = {
    name: "홍길동",
    age: 30,
    isAdmin: true,
};
userMNU.name = "이몽룡"; // 가능함
// userMNU.age = 35; // 오류 발생, age는 읽기 전용 
console.log(userMNU);
