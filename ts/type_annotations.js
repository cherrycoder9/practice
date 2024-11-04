"use strict";
// 타입 주석 기본 문법
// 변수 선언 시 타입을 명시적으로 지정
let age = 30;
// Number 객체 타입도 있으니 차이점에 유의할 것
// 문자열 타입 주석을 지정해 변수 선언
let username = "제로디"; // string 타입 주석을 사용해 변수 타입 명시
// 함수 매개변수와 반환값에 타입 주석을 지정
function add1(a, b) {
    return a + b;
}
// 타입 추론의 동작 원리
// 변수에 초기값을 할당하면 타입스크립트가 타입을 추론함
let inferredString = "Hello, TypeScript!"; // string 으로 추론함
let inferredNumber = 42; // number 타입으로 추론
let explicitType = true;
function multiply1(a, b = 2) {
    return a * b;
}
let result = multiply1(5); // multiply 함수 호출시 반환값의 타입이 자동으로 number로 추론됨
