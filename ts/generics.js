"use strict";
// 제네릭 타입을 사용하는 클래스를 정의해 다양한 타입의 Stack 자료구조를 만듦
class Stack {
    items = []; // T 타입의 배열 사용, Stack 요소들 저장
    // push 메서드, Stack에 요소 추가
    push(item) {
        this.items.push(item);
        console.log(`${item} 스택에 추가됨`);
    }
    // pop 메서드, Stack에서 마지막 요소를 제거하고 반환
    pop() {
        if (this.items.length === 0) {
            console.log(`스택이 비어 있음`);
            return undefined;
        }
        const removedItem = this.items.pop();
        console.log(`${removedItem} 스택에서 제거됨`);
        return removedItem;
    }
    // peek 메서드, Stack의 마지막 요소 확인 
    peek() {
        if (this.items.length === 0) {
            console.log(`스택이 비어 있음`);
            return undefined;
        }
        return this.items[this.items.length - 1];
    }
    // isEmpty 메서드, Stack이 비어 있는지 확인
    isEmpty() {
        return this.items.length === 0;
    }
}
// 제네릭 함수 정의
// 두 개의 매개변수를 받아 KeyValuePair 객체를 생성하는 함수
function createKeyValuePair(key, value) {
    return { key, value }; // 주어진 키와 값을 가진 객체를 반환 
}
// 제네릭 함수 사용
const numberStringPair = createKeyValuePair(1, "하나");
console.log(numberStringPair); // 출력: { key: 1, value: '하나' }
// 제네릭 클래스 사용
const numberStack = new Stack();
numberStack.push(10);
numberStack.push(20);
numberStack.pop();
// 제네릭 제약 조건이 있는 함수 정의
function logWithLength(item) {
    console.log(`길이: ${item.length}`);
}
logWithLength(`안녕하세요`); // 출력: 길이: 5
logWithLength([1, 2, 3, 4]); // 출력: 길이: 4
// 제네릭을 사용해 다양한 타입을 지원하는 데이터 저장소 클래스 정의
class MemoryRepository {
    storage = []; // 저장을 위한 배열
    // 요소를 저장소에 추가하는 메서드
    add(item) {
        this.storage.push(item);
        console.log(`${item} 저장소에 추가됨`);
    }
    // 요소를 저장소에서 제거하는 메서드
    remove(item) {
        const index = this.storage.indexOf(item);
        if (index > -1) {
            this.storage.splice(index, 1);
            console.log(`${item} 저장소에서 제거됨`);
        }
        else {
            console.log(`${item} 저장소에 존재하지 않음`);
        }
    }
    // 저장소의 모든 요소를 반환하는 메서드
    getAll() {
        return [...this.storage];
    }
}
// 메모리 저장소 클래스 사용
const stringRepository = new MemoryRepository();
stringRepository.add("타입스크립트");
stringRepository.add("제네릭");
stringRepository.remove("타입스크립트");
console.log(stringRepository.getAll());
