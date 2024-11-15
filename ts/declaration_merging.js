"use strict";
const personDM = {
    name: "김철수",
    age: 30,
    gender: "남성"
};
console.log(`병합된 인터페이스 출력: ${JSON.stringify(personDM)}`);
function add(x, y) {
    return x + y; // 실제 구현부에서는 모든 경우를 처리할 수 있도록 any 타입 사용함 
}
console.log(`숫자 덧셈: ${add(5, 10)}`);
console.log(`문자열 덧셈: ${add("안녕", "하세요")}`);
// 3. 모듈 병합
// 동일한 모듈 이름을 여러 번 선언해 모듈을 병합할 수 있음
// Node.js와 같은 환경에선 모듈 시스템이 다르므로 모듈 병합이 TS와 다르게 작동할 수 있음
var Animals;
(function (Animals) {
    class Dog {
        name;
        constructor(name) {
            this.name = name;
        }
    }
    Animals.Dog = Dog;
})(Animals || (Animals = {}));
(function (Animals) {
    class Cat {
        name;
        constructor(name) {
            this.name = name;
        }
    }
    Animals.Cat = Cat;
})(Animals || (Animals = {}));
const dogDM = new Animals.Dog("바둑이");
const catDM = new Animals.Dog("나비");
console.log(`모듈 병합 - 강아지 이름: ${dogDM.name}`);
console.log(`모듈 병합 - 고양이 이름: ${catDM.name}`);
// 4. 클래스 병합 (보강)
// 클래스를 직접 병합할 수는 없지만, 인터페이스를 사용해 클래스의 기능을 확장 가능함
class Car {
    brand;
    constructor(brand) {
        this.brand = brand;
    }
}
Car.prototype.model = "모델명 미정";
const myCar = new Car("현대");
console.log(`클래스 병합 - 자동차 브랜드: ${myCar.brand}`);
console.log(`클래스 병합 - 자동차 모델: ${myCar.model}`);
globalThis.customProperty = "TS 선언 병합";
console.log(`전역 스코프 병합: ${globalThis.customProperty}`);
