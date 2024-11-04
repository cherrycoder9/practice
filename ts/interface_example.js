"use strict";
// Employee 인터페이스를 구현하는 클래스 정의
class Developer {
    name;
    age;
    employeeId;
    department;
    // 생성자를 통해 필수 프로퍼티들 초기화
    constructor(name, age, employeeId, department) {
        this.name = name;
        this.age = age;
        this.employeeId = employeeId;
        this.department = department;
    }
    // greet 메서드 구현 
    greet() {
        console.log(`이름은 ${this.name}이고, 나이는 ${this.age}입니다.`);
        console.log(`직원 아이디는 ${this.employeeId}, 부서는 ${this.department}입니다.`);
    }
}
// 인터페이스는 확장(상속) 가능하고, 클래스에서 구현될 수 있음
// 타입 앨리어스는 기존 타입을 재사용하거나 결합하는데 유용
// 유연성 측면에서는 인터페이스가 더 강력하지만, 특정 상황에서는 타입 앨리어스가 더 간결
// 문자열 조작 함수 타입을 사용하는 변수 정의
const concatenateStrings = (a, b) => {
    return `${a} ${b}`;
};
// Developer 클래스의 인스턴스 생성 및 사용
const dev = new Developer('홍길동', 30, 'DEV123', '개발팀');
dev.greet(); // 홍길동 개발자의 정보 출력
// 문자열 조작 함수 호출
console.log(concatenateStrings('타입스크립트', '인터페이스'));
// 타입 앨리어스를 통한 제품 객체 생성
const product = {
    productId: 'P12345',
    productName: '노트북',
    price: 1500000,
};
// 제품 정보 출력
console.log(`제품명: ${product.productName}, 가격: ${product.price}원`);
