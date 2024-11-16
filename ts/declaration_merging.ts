// 선언 병합
// 동일한 이름의 여러 선언을 하나로 합치는 기능
// 1. 인터페이스 선언 병합
// 동일 이름의 인터페이스가 여러 번 선언되면 TS는 자동으로 병합함 
interface PersonDM {
    name: string; // 이름 속성
    age: number; // 나이 속성
}

interface PersonDM {
    gender: string; // 성별 속성 추가
}

const personDM: PersonDM = {
    name: "김철수",
    age: 30,
    gender: "남성"
};

console.log(`병합된 인터페이스 출력: ${JSON.stringify(personDM)}`);

// 2. 함수 오버로드 선언 병합
// 동일한 이름을 가진 함수 시그니처를 여러 번 정의해 오버로드 기능을 구현할 수 있음
function addDM(x: number, y: number): number;
function addDM(x: string, y: string): string;
function addDM(x: any, y: any): any {
    return x + y; // 실제 구현부에서는 모든 경우를 처리할 수 있도록 any 타입 사용함 
}

console.log(`숫자 덧셈: ${addDM(5, 10)}`);
console.log(`문자열 덧셈: ${addDM("안녕", "하세요")}`);

// 3. 모듈 병합
// 동일한 모듈 이름을 여러 번 선언해 모듈을 병합할 수 있음
// Node.js와 같은 환경에선 모듈 시스템이 다르므로 모듈 병합이 TS와 다르게 작동할 수 있음
namespace Animals {
    export class Dog {
        constructor(public name: string) { }
    }
}

namespace Animals {
    export class Cat {
        constructor(public name: string) { }
    }
}

const dogDM = new Animals.Dog("바둑이");
const catDM = new Animals.Dog("나비");

console.log(`모듈 병합 - 강아지 이름: ${dogDM.name}`);
console.log(`모듈 병합 - 고양이 이름: ${catDM.name}`);

// 4. 클래스 병합 (보강)
// 클래스를 직접 병합할 수는 없지만, 인터페이스를 사용해 클래스의 기능을 확장 가능함
class Car {
    brand: string;
    constructor(brand: string) {
        this.brand = brand;
    }
}

interface Car {
    model: string; // 클래스에 추가할 속성
}

Car.prototype.model = "모델명 미정";

const myCar = new Car("현대");
console.log(`클래스 병합 - 자동차 브랜드: ${myCar.brand}`);
console.log(`클래스 병합 - 자동차 모델: ${myCar.model}`);

// 5. 전역 스코프 병합 (node.js에선 안됨)
// 전역 객체의 속성을 여러 번 선언해 전역에서 사용할 수 있게 병합할 수 있음
// interface Window {
//     customProperty: string;
// }

// window.customProperty = "TS 선언 병합";
// console.log(`전역 스코프 병합: ${window.customProperty}`);

// node.js 테스트용 
interface CustomGlobal {
    customProperty: string;
}

(globalThis as any).customProperty = "TS 선언 병합";
console.log(`전역 스코프 병합: ${(globalThis as any).customProperty}`);
