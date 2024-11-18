"use strict";
// tsconfig.json 파일의 compilerOptions에서 strict mode를 활성화하면
// 타입이 잘못되거나 정의되지 않은 변수 사용을 엄격하게 검사함
// Null과 Undefined 처리를 위한 Non-null Assertion 
function getUserNameById(userId) {
    // userId가 null일 가능성이 있는 경우 에러를 발생시키지 않도록 타입 단언 사용 
    if (userId === null) {
        throw new Error("유효하지 않은 사용자 ID입니다.");
    }
    // Non-null 단언 연산자 (!)를 사용해 userId가 null이 아님을 보장함 
    return `user_${userId}`;
}
// 함수 호출
console.log(getUserNameById(42));
// console.log(getUserNameById(null)); // 오류
// Any 타입을 사용하지 않고 타입 안정성을 보장하기 위한 예시
// 엄격한 타입 시스템에서 any를 최대한 지양하고 명시적 타입을 사용하는 것 권장
function calculateTotal(items) {
    // 가격과 수량을 기반으로 총합 계산
    let total = 0;
    for (const item of items) {
        total += item.price * item.quantity;
    }
    return total;
}
// 함수 호출
const shoppingCart = [
    { price: 1000, quantity: 2 },
    { price: 2000, quantity: 1 },
    { price: 500, quantity: 4 }
];
console.log(calculateTotal(shoppingCart));
// Type Narrowing
// 타입 좁히기를 통해 코드 안정성을 높이고, 조건에 따라 타입을 특정함
function printId2(id) {
    if (typeof id === "string") {
        console.log(`ID는 문자열: ${id.toUpperCase()}`);
    }
    else {
        console.log(`ID는 숫자: ${id}`);
    }
}
printId2(101);
printId2("abc123");
function getUserCity(user) {
    // Optional chaining을 사용해 address가 존재하는지 확인 후 city에 접근함
    // 만약 address가 존재하지 않으면 undefined가 반환됨
    return user.address?.city ?? "도시 정보 없음";
}
// 함수 호출
const usertc1 = { id: 1, name: "머스크", address: { street: "안남로", city: "인천" } };
const usertc2 = { id: 2, name: "하사비스" }; // 주소 없음
console.log(getUserCity(usertc1)); // 인천
console.log(getUserCity(usertc2)); // 도시정보없음
