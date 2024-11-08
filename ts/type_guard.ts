// 타입가드는 런타임 시점에서 변수의 타입을 좁혀나가며 특정 타입으로
// 안전하게 동작할 수 있도록 해주는 함수 또는 조건문을 의미함

// 타입 내로잉은 변수의 타입을 좁히는 과정을 말하며
// 타입스크립트에서 안전한 코드 작성을 가능하게 함

// 타입 가드는 typeof, instanceof, 커스텀 타입 가드 함수 등을 활용해 구현할 수 있음
// typeof는 원시 타입을 판별, instanceof는 객체 타입을 판별
// 커스텀 타입 가드는 특정 타입 인터페이스를 구현했는지 여부를 판별할때 사용

interface Animal12 {
    name: string;
}

interface Bird12 extends Animal12 {
    canFly: boolean;
    wingspan: number; // 날개 길이
}

interface Fish12 extends Animal12 {
    canSwim: boolean;
    finCount: number; // 지느러미 개수 
}

// Bird인지 Fish인지 판별하는 커스텀 타입 가드 함수 
function isBird(animal: Animal12): animal is Bird12 {
    // canFly 속성 여부로 Bird 판별
    return (animal as Bird12).canFly !== undefined;
}

function isFish(animal: Animal12): animal is Fish12 {
    // canSwim 속성 여부로 Fish 판별
    return (animal as Fish12).canSwim !== undefined;
}

// Animal 배열을 인자로 받아 특정 동물의 행동을 출력하는 함수 
function describeAnimal(animal: Animal12): void {
    if (isBird(animal)) {
        console.log(`${animal.name}는 새다, 날수 있나?: ${animal.canFly}, 날개 길이: ${animal.wingspan}`);
    } else if (isFish(animal)) {
        console.log(`${animal.name}는 물고기다, 수영 가능?: ${animal.canSwim}, 지느러미 개수: ${animal.finCount}`);
    } else {
        console.log(`${animal.name} 새도 물고기도 아님`);
    }
}

// 실제 데이터로 함수 테스트
const eagle: Bird12 = {
    name: '독수리',
    canFly: true,
    wingspan: 220,
};

const salmon: Fish12 = {
    name: '연어',
    canSwim: true,
    finCount: 4,
};

const cat: Animal12 = {
    name: '고양이'
};

// describeAnimal 함수 호출 
console.log(`--- 동물 정보 출력 ---`);
describeAnimal(eagle);
describeAnimal(salmon);
describeAnimal(cat);

// 다중 타입 가드를 이용한 비슷한 패턴 내로잉 
function getAnimalAbilities(animal: Animal12): void {
    if (isBird(animal) && animal.canFly) {
        console.log(`${animal.name}는 날 수 있음`);
    } else if (isFish(animal) && animal.canSwim) {
        console.log(`${animal.name}는 수영할 수 있음`);
    } else {
        console.log(`${animal.name}는 날지도 수영하지도 못함`);
    }
}

console.log(`--- 동물 능력 정보 출력 ---`);
getAnimalAbilities(eagle);
getAnimalAbilities(salmon);
getAnimalAbilities(cat);

// typeof를 활용한 타입 내로잉
function printValue(value: string | number): void {
    // value가 string 타입일 때 처리
    if (typeof value === 'string') {
        console.log(`문자열 값: ${value}`);
    }
    // value가 number 타입일 때 처리
    else if (typeof value === 'number') {
        console.log(`숫자 값: ${value}`);
    }
}

console.log(`--- 값 출력 ---`);
printValue(`타입스크립트`);
printValue(42);

// instanceof 객체 타입 내로잉 
class Dog12 {
    constructor(public name: string) { }
    bark(): void {
        console.log(`${this.name}가 멍멍 짖습니다.`);
    }
}

class Cat12 {
    constructor(public name: string) { }
    meow(): void {
        console.log(`${this.name}가 야옹합니다.`);
    }
}

function makeSound(animal: Dog12 | Cat12): void {
    // animal이 Dog의 인스턴스일 때
    if (animal instanceof Dog12) {
        animal.bark();
    }
    // animal이 Cat의 인스턴스일 때
    else if (animal instanceof Cat12) {
        animal.meow();
    }
}

console.log(`--- 동물 소리 출력 ---`);
const dog = new Dog12(`바둑이`);
const cat2 = new Cat12(`나비`);
makeSound(dog);
makeSound(cat2);

