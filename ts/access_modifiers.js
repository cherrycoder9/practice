"use strict";
// 클래스 접근 제어자
// 기본 클래스 정의
class Animal {
    // public 접근 제어자: 어디서든 접근 가능
    name;
    // protected 접근 제어자: 이 클래스와 하위 클래스에서만 접근 가능
    age;
    // private 접근 제어자: 이 클래스 내에서만 접근 가능 
    id;
    // 생성자
    constructor(name, age, id) {
        this.name = name;
        this.age = age;
        this.id = id;
    }
    // public 메서드: 외부에서도 호출 가능
    getInfo() {
        return `이름: ${this.name}, 나이: ${this.age}, 아이디: ${this.id}`;
    }
    // private 메서드: 내부에서만 호출 가능
    generateUniqueId() {
        // 고유한 ID 생성 로직을 구현 
        return `ID-${Math.random().toString(36).substring(2, 11)}`;
    }
    // protected 메서드: 이 클래스와 하위 클래스에서 접근 가능
    getAge() {
        return this.age;
    }
}
// 상속을 사용한 하위 클래스 정의
class Dog extends Animal {
    breed;
    constructor(name, age, id, breed) {
        // 부모 클래스의 생성자를 호출해 속성을 초기화
        super(name, age, id);
        this.breed = breed;
    }
    getBreed() {
        return this.breed;
    }
    getDetailedInfo() {
        const age = this.getAge();
        return `이름: ${this.name}, 나이: ${age}, 품종: ${this.breed}`;
    }
}
const myDog = new Dog("바둑이", 3, "abc123", "진돗개");
console.log(myDog.getInfo());
console.log(myDog.getBreed());
console.log(myDog.getDetailedInfo());
