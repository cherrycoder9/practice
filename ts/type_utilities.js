"use strict";
// user의 일부 프로퍼티만 채우고 싶을 때 Partial을 사용해 모든 프로퍼티가 선택적으로 됨
const updateUser = (user) => {
    console.log(`업데이트된 사용자 정보: ${JSON.stringify(user)}`);
};
updateUser({ name: "홍길동" }); // 선택적으로 name만 업데이트 
updateUser({ email: "hong@example.com" }); // 선택적으로 email만 업데이트
// Readonly 유틸리티 타입
// Readonly<T>는 타입 T의 모든 프로퍼티를 읽기 전용으로 만들어줌
const originalUser = {
    name: "이순신",
    age: 45,
    email: "lee@example.com"
};
// originalUser.age = 50; // 오류, Readonly로 선언된 프로퍼티를 수정 불가
console.log(`사용자 정보 (읽기 전용): ${JSON.stringify(originalUser)}`);
// Product3 타입에서 id와 name만 선택해 새로운 타입 정의
const getProductSummary = (product) => {
    console.log(`상품 요약: ID(${product.id}), 이름(${product.name})`);
};
getProductSummary({ id: 1, name: "노트북" });
const rolePermissions = {
    admin: { permissions: ["read", "write", "delete"] },
    user: { permissions: ["read", "write"] },
    guest: { permissions: ["read"] }
};
console.log(`사용자 권한 정보: ${JSON.stringify(rolePermissions)}`);
const color = "red";
console.log(`기본 색상: ${color}`);
// Car 타입에서 owner 프로퍼티를 제외한 타입 정의
const createCarWithoutOwner = (car) => {
    console.log(`차량 정보: 브랜드(${car.brand}), 모델(${car.model}), 연식(${car.year})`);
};
createCarWithoutOwner({ brand: "현대", model: "소나타", year: 2022 });
const myAddress2 = {
    street: "서울특별시",
};
console.log(`선택적 주소 정보: ${JSON.stringify(myAddress2)}`);
const contactInfo = "010-1234-5678";
console.log(`유효한 연락처 정보: ${contactInfo}`);
