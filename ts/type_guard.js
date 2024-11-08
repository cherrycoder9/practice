"use strict";
// 타입가드는 런타임 시점에서 변수의 타입을 좁혀나가며 특정 타입으로
// 안전하게 동작할 수 있도록 해주는 함수 또는 조건문을 의미함
// Bird인지 Fish인지 판별하는 커스텀 타입 가드 함수 
function isBird(animal) {
    // canFly 속성 여부로 Bird 판별
    return animal.canFly !== undefined;
}
function isFish(animal) {
    // canSwim 속성 여부로 Fish 판별
    return animal.canSwim !== undefined;
}
// Animal 배열을 인자로 받아 특정 동물의 행동을 출력하는 함수 
function describeAnimal(animal) {
    if (isBird(animal)) {
        console.log(`${animal.name}는 새다, 날수 있나?: ${animal.canFly}, 날개 길이: ${animal.wingspan}`);
    }
    else if (isFish(animal)) {
        console.log(`${animal.name}는 물고기다, 수영 가능?: ${animal.canSwim}, 지느러미 개수: ${animal.finCount}`);
    }
    else {
        console.log(`${animal.name} 새도 물고기도 아님`);
    }
}
// 실제 데이터로 함수 테스트
const eagle = {
    name: '독수리',
    canFly: true,
    wingspan: 220,
};
const salmon = {
    name: '연어',
    canSwim: true,
    finCount: 4,
};
const cat = {
    name: '고양이'
};
// describeAnimal 함수 호출 
console.log(`--- 동물 정보 출력 ---`);
describeAnimal(eagle);
describeAnimal(salmon);
describeAnimal(cat);
// 다중 타입 가드를 이용한 비슷한 패턴 내로잉 
function getAnimalAbilities(animal) {
    if (isBird(animal) && animal.canFly) {
        console.log(`${animal.name}는 날 수 있음`);
    }
    else if (isFish(animal) && animal.canSwim) {
        console.log(`${animal.name}는 수영할 수 있음`);
    }
    else {
        console.log(`${animal.name}는 날지도 수영하지도 못함`);
    }
}
console.log(`--- 동물 능력 정보 출력 ---`);
getAnimalAbilities(eagle);
getAnimalAbilities(salmon);
getAnimalAbilities(cat);
// typeof를 활용한 타입 내로잉
function printValue(value) {
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
    name;
    constructor(name) {
        this.name = name;
    }
    bark() {
        console.log(`${this.name}가 멍멍 짖습니다.`);
    }
}
class Cat12 {
    name;
    constructor(name) {
        this.name = name;
    }
    meow() {
        console.log(`${this.name}가 야옹합니다.`);
    }
}
function makeSound(animal) {
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
