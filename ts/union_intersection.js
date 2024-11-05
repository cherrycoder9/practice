"use strict";
// 유니언 타입은 여러 타입 중 하나의 타입을 가질 수 있도록 선언함
// 여기에서는 string 또는 number 타입을 허용하는 변수 선언
function printId(id) {
    // id가 number일 때를 처리
    if (typeof id === "number") {
        console.log(`숫자 ID: ${id}`); // number 타입으로 처리
    }
    else {
        // id가 string일 때를 처리
        console.log(`문자열 ID: ${id}`); // string 타입으로 처리 
    }
}
// 함수 호출
printId(101);
printId("user1234");
// Person6과 Employee6의 모든 속성을 가지는 타입 정의
const employee6 = {
    name: "김철수",
    age: 35,
    employeeId: 101,
    department: "개발부서"
};
// 교차 타입 객체 출력
console.log(`이름: ${employee6.name}, 나이: ${employee6.age}, 직원 ID: ${employee6.employeeId}, 부서: ${employee6.department}`);
// 유니언 타입을 사용해 다양한 입력을 처리하는 함수
function getArea(shape) {
    // shape 타입 좁히기 (타입 가드 사용)
    if ("radius" in shape) {
        // Circle 타입으로 좁히기
        return Math.PI * shape.radius ** 2;
    }
    else {
        // Rectangle 타입으로 좁히기 
        return shape.width * shape.height;
    }
}
// 함수 호출 예시
const circle = { radius: 10 };
const rectangle = { width: 5, height: 10 };
console.log(`원의 넓이: ${getArea(circle)}`);
console.log(`직사각형의 넓이: ${getArea(rectangle)}`);
// Vehicle 타입을 구현한 클래스
class FlyingCar {
    drive() {
        console.log(`자동자 운전 중`);
    }
    fly() {
        console.log(`하늘을 나는 중`);
    }
}
// 클래스 인스턴스 생성
const myFlyingCar = new FlyingCar();
myFlyingCar.drive();
myFlyingCar.fly();
// 유니언과 교차 타입을 결합해 여러 조건을 만족하는 함수 정의
function move(entity) {
    if ("drive" in entity) {
        entity.drive();
    }
    if ("fly" in entity) {
        entity.fly();
    }
    if ("swim" in entity) {
        entity.swim();
    }
}
// Flyable & Swimmable 타입을 가진 객체 생성 
const duck = {
    fly() {
        console.log(`오리가 날아오릅니다.`);
    },
    swim() {
        console.log(`오리가 물 위를 헤엄칩니다.`);
    }
};
// 함수 호출 
move(duck);
