"use strict";
// 객체를 초기화할 때 name과 createdAt은 반드시 할당해야 함
const user5 = {
    name: "홍길동",
    createdAt: new Date(),
};
// 선택적 속성에 대한 기본값 설정을 위한 함수
function getUserEmail(user5) {
    // 이메일이 undefined일 경우 기본값 반환
    return user5.email ?? "기본 이메일 없음";
}
console.log(`사용자 이름: ${user5.name}`);
console.log(`사용자 이메일: ${getUserEmail(user5)}`);
console.log(`사용자 이름: ${user5.createdAt}`);
// user.createdAt = new Date(); // 오류, 읽기 전용 속성은 수정할 수 없음
// Date는 날짜와 시간을 다루기 위한 객체
// 선택적 속성과 읽기 전용 속성을 포함한 클래스
class Product5 {
    name;
    price;
    id;
    createdAt;
    constructor(name, id, price) {
        this.name = name;
        this.id = id;
        this.price = price;
        this.createdAt = new Date(); // 객체 생성시 현재 시간을 설정 
    }
    // 상품 정보를 출력하는 메서드
    printDetails() {
        console.log(`상품명: ${this.name}`);
        console.log(`상품 ID: ${this.id}`);
        console.log(`생성일: ${this.createdAt}`);
        if (this.price !== undefined) {
            console.log(`가격: ${this.price}원`);
        }
        else {
            console.log(`가격 정보 없음`);
        }
    }
}
const product5 = new Product5("컴퓨터", 101);
product5.printDetails();
// product.id = 202; // 오류, 읽기 전용 프로퍼티는 수정 불가
// 읽기 전용 배열(ReadonlyArray) 예시
const readonlyArray = ["사과", "바나나", "체리"];
console.log(`읽기 전용 배열 요소:`);
readonlyArray.forEach((fruit) => console.log(fruit));
// 읽기 전용 배열에 push 메서드 사용 못함
// 읽기 전용 배열을 변형하는 방법: 복사 후 변경
const updatedArray = [...readonlyArray, "포도"];
console.log(`업데이트된 배열: ${updatedArray}`);
// readonly 키워드는 TS에서 특정 필드가 객체가 생성된 후에 수정되지 않도록 보호함
// 주로 한번 설정되면 변경되지 말아야 하는 데이터에 사용
// 클래스나 인터페이스의 속성 선언시 사용 가능 
